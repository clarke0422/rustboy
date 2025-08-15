use std::fs::File;
use std::io::{self, Read};
use std::time::{Duration, Instant};
use std::thread::sleep;

mod cpu;
use crate::cpu::Cpu;

const FRAME_CYCLES: u64 = 70224; // cycles per 59.7 Hz frame, based on 4.194304 MHz clock speed

fn main() -> io::Result<()> {
    // CPU setup
    let mut file = File::open("roms/tetris.gb")?; // hard-coded for now

    let mut rom = Vec::new();
    file.read_to_end(&mut rom)?;
    // if let Some(program_counter) = rom.iter().position(|&byte| byte == 0x41u8) {
    //     println!("Found 0x41u8 instruction at: {:04X}", program_counter);
    // }

    let mut cpu = Cpu::new(rom);

    // CPU timing
    let frame_duration = Duration::from_secs_f64(1.0 / 59.7);
    let mut last_frame = Instant::now();
    let mut cycles_run_during_frame: u64 = 0;

    loop {
        let cycles_run_during_instruction = cpu.execute_instruction();
        cycles_run_during_frame += cycles_run_during_instruction;

        if cycles_run_during_frame >= FRAME_CYCLES {
            cycles_run_during_frame -= FRAME_CYCLES;

            let elapsed = last_frame.elapsed();
            if elapsed < frame_duration {
                sleep(frame_duration - elapsed);
            }
            last_frame = Instant::now();

            // render frame
        }
    }

    Ok(())
}