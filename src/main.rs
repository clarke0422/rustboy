use std::fs::File;
use std::io::{self, Read};

mod cpu;
use crate::cpu::Cpu;

fn main() -> io::Result<()> {
    let mut file = File::open("roms/tetris.gb")?; // hard-coded for now

    let mut rom = Vec::new();
    file.read_to_end(&mut rom)?;
    println!("Read {} bytes", rom.len());
    if let Some(program_counter) = rom.iter().position(|&byte| byte == 0x41u8) {
        println!("Found 0x41u8 instruction at: {:04X}", program_counter);
    }

    let mut cpu = Cpu::new(rom);
    cpu.debug_routine();

    Ok(())
}