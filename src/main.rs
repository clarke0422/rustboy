mod cpu;
use crate::cpu::Cpu;

fn main() {
    println!("Hello, world!");
    let mut cpu = Cpu::new();
    cpu.debug_routine();
}