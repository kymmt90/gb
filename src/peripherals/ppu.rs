#[derive(Debug)]
pub struct Ppu {
    mode: Mode,
    lcdc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,
    vram: Box<[u8; 0x2000]>,
    oam: Box<[u8; 0xA0]>,
    buffer: Box<[u8; LCD_WIDTH * LCD_HEIGHT]>,
    cycles: u8,
}

// LCDC register bits
const PPU_ENABLED: u8 = 1 << 7;
const WINDOW_TILE_MAP: u8 = 1 << 6;
const WINDOW_ENABLED: u8 = 1 << 5;
const TILE_DATA_ADDRESSING_MODE: u8 = 1 << 4;
const BG_TILE_MAP: u8 = 1 << 3;
const SPRITE_SIZE: u8 = 1 << 2;
const SPRITE_ENABLED: u8 = 1 << 1;
const BG_WINDOW_ENABLED: u8 = 1;

// STAT register bits
const LYC_INTERRUPT: u8 = 1 << 6;
const OAM_INTERRUPT: u8 = 1 << 5;
const VBLANK_INTERRUPT: u8 = 1 << 4;
const HBLANK_INTERRUPT: u8 = 1 << 3;
const LYC_EQ_LY: u8 = 1 << 2;

const LCD_HEIGHT: usize = 144;
const LCD_WIDTH: usize = 160;

impl Ppu {
    pub fn new() -> Self {
        Self {
            mode: Mode::ScanningOam,
            lcdc: 0,
            stat: 0b1000_0000,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
            vram: Box::new([0; 0x2000]),
            oam: Box::new([0; 0xA0]),
            buffer: Box::new([0; LCD_WIDTH * LCD_HEIGHT]),
            cycles: 0,
        }
    }

    pub fn emulate_cycle(&mut self) -> bool {
        if self.lcdc & PPU_ENABLED == 0 {
            return false;
        }

        self.cycles -= 1;
        if self.cycles > 0 {
            return false;
        }

        let mut ret = false;
        match self.mode {
            Mode::AwaitingHBlank => {
                self.ly += 1;
                if self.ly < LCD_WIDTH as u8 {
                    self.mode = Mode::ScanningOam;
                    self.cycles = 20;
                } else {
                    self.mode = Mode::AwaitingVBlank;
                    self.cycles = 114;
                }
                self.check_lyc_eq_ly();
            }
            Mode::AwaitingVBlank => {
                self.ly += 1;
                if self.ly > 153 {
                    ret = true;
                    self.ly = 0;
                    self.mode = Mode::ScanningOam;
                    self.cycles = 20;
                } else {
                    self.cycles = 114;
                }
                self.check_lyc_eq_ly();
            }
            Mode::ScanningOam => {
                self.mode = Mode::Drawing;
                self.cycles = 43;
            }
            Mode::Drawing => {
                self.render_bg();
                self.mode = Mode::AwaitingHBlank;
                self.cycles = 51;
            }
        }

        ret
    }

    fn check_lyc_eq_ly(&mut self) {
        if self.ly == self.lyc {
            self.stat |= LYC_EQ_LY;
        } else {
            self.stat &= !LYC_EQ_LY;
        }
    }

    fn get_pixel_from_tile(&self, title_idx: usize, row: u8, col: u8) -> u8 {
        let r = (row * 2) as usize;
        let c = (7 - col) as usize;
        let tile_addr = title_idx << 4;
        let low = self.vram[(tile_addr | r) & 0x1fff];
        let high = self.vram[(tile_addr | (r + 1)) & 0x1fff];

        ((low >> c) & 1) | (((high >> c) & 1) << 1)
    }

    fn get_tile_idx_from_tile_map(&self, tile_map: bool, row: u8, col: u8) -> usize {
        let start_addr: usize = 0x1800 | ((tile_map as usize) << 10);
        let ret = self.vram[start_addr | (((row as usize) << 5) + col as usize)];

        if self.lcdc & TILE_DATA_ADDRESSING_MODE > 0 {
            ret as usize
        } else {
            ((ret as i8 as i16) + 0x100) as usize
        }
    }

    fn render_bg(&mut self) {
        if self.lcdc & BG_WINDOW_ENABLED == 0 {
            return;
        }

        let y = self.ly.wrapping_add(self.scy);
        for i in 0..LCD_WIDTH {
            let x = (i as u8).wrapping_add(self.scx);

            let tile_idx =
                self.get_tile_idx_from_tile_map(self.lcdc & BG_TILE_MAP > 0, y >> 3, x >> 3);

            let pixel = self.get_pixel_from_tile(tile_idx, y & 7, x & 7);

            self.buffer[LCD_WIDTH * self.ly as usize + i] = match (self.bgp >> (pixel << 1)) & 0b11
            {
                0b00 => 0xff,
                0b01 => 0xaa,
                0b10 => 0x55,
                _ => 0x00,
            };
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9fff => {
                if self.mode == Mode::Drawing {
                    0xff
                } else {
                    self.vram[addr as usize & 0x1fff]
                }
            }
            0xfe00..=0xfe9f => {
                if self.mode == Mode::Drawing {
                    0xff
                } else {
                    self.oam[addr as usize & 0xff]
                }
            }
            0xff40 => self.lcdc,
            0xff41 => 0x80 | self.stat | self.mode as u8, // bit 7 is always set
            0xff42 => self.scy,
            0xff43 => self.scx,
            0xff44 => self.ly,
            0xff45 => self.lyc,
            0xff47 => self.bgp,
            0xff48 => self.obp0,
            0xff49 => self.obp1,
            0xff4a => self.wy,
            0xff4b => self.wx,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x8000..=0x9fff => {
                if self.mode != Mode::Drawing {
                    self.vram[addr as usize & 0x1fff] = val;
                }
            }
            0xfe00..=0xfe9f => {
                if self.mode != Mode::ScanningOam && self.mode != Mode::Drawing {
                    self.oam[addr as usize & 0xff] = val;
                }
            }
            0xff40 => self.lcdc = val,

            // clear 0-2 bits with 0xf8 because they are read-only
            0xff41 => self.stat = (self.stat & LYC_EQ_LY) | (val & 0xf8),

            0xff42 => self.scy = val,
            0xff43 => self.scx = val,
            0xff44 => {}
            0xff45 => self.lyc = val,
            0xff47 => self.bgp = val,
            0xff48 => self.obp0 = val,
            0xff49 => self.obp1 = val,
            0xff4a => self.wy = val,
            0xff4b => self.wx = val,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Mode {
    AwaitingHBlank = 0,
    AwaitingVBlank = 1,
    #[default]
    ScanningOam = 2,
    Drawing = 3,
}
