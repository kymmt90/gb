fn main() {
    println!("Hello, world!");

    gb::boot_rom::BootRom::new(Box::new([0; 0x100]));
}
