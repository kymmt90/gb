use std::fs;

use clap::Parser;

use log::debug;

#[derive(Parser)]
struct Cli {
    boot_rom_filename: Option<String>,
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    if let Some(boot_rom_filename) = cli.boot_rom_filename {
        debug!("boot ROM: {}", boot_rom_filename);

        let raw_boot_rom = fs::read(boot_rom_filename)
            .expect("failed to read boot ROM")
            .into_boxed_slice();

        debug!("boot ROM loaded");
        debug!("boot ROM size: {}", raw_boot_rom.len());

        let mut console = gb::console::Console::new(gb::peripherals::BootRom::new(raw_boot_rom));

        console.run();
    }
}
