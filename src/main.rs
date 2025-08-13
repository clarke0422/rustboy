use std::fs::File;
use std::io::{self, Read};

mod cpu;
use crate::cpu::Cpu;

fn main() -> io::Result<()> {
    let mut file = File::open("roms/tetris.gb")?; // hard-coded for now

    let mut rom = Vec::new();
    file.read_to_end(&mut rom)?;
    println!("Read {} bytes", rom.len());

    let mut cpu = Cpu::new(rom);
    cpu.debug_routine();

    Ok(())
}