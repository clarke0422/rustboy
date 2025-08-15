pub struct Cpu {
    registers: Vec<u8>,
    ram: Vec<u8>,
    rom: Vec<u8>,
}

#[derive(Debug, Clone)]
enum R8Address {
    A = 0,
    F = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    H = 6,
    L = 7,
}

#[allow(dead_code)]
const ALL_8BIT_ADDRESSES: [R8Address; 8] = [
    R8Address::A,
    R8Address::F,
    R8Address::B,
    R8Address::C,
    R8Address::D,
    R8Address::E,
    R8Address::H,
    R8Address::L,
];

#[derive(Debug, Clone)]
enum R16Address {
    AF = 0,
    BC = 2,
    DE = 4,
    HL = 6,
    SP = 8,
    PC = 10,
}

#[allow(dead_code)]
const ALL_16BIT_ADDRESSES: [R16Address; 6] = [
    R16Address::AF,
    R16Address::BC,
    R16Address::DE,
    R16Address::HL,
    R16Address::SP,
    R16Address::PC,
];

impl Cpu {
    pub fn new(rom:Vec<u8>) -> Cpu {
        let registers = vec![0; 12];
        let ram = vec![0; 65536];
        Cpu { registers, ram, rom }
    }

    fn read_r8(&mut self, address: R8Address) -> u8 {
        self.registers[address as usize]
    }

    fn write_r8(&mut self, address: R8Address, value: u8) {
        self.registers[address as usize] = value;
    }

    fn read_r16(&mut self, address: R16Address) -> u16 {
        let register_index = address as usize;
        let high = self.registers[register_index] as u16;
        let low = self.registers[register_index + 1] as u16;
        (high << 8) | low
    }

    fn write_r16(&mut self, address: R16Address, value: u16) {
        let high = (value >> 8) as u8;
        let low = value as u8;
        let register_index = address as usize;
        self.registers[register_index] = high;
        self.registers[register_index + 1] = low;
    }

    fn read_ram(&mut self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
    }

    fn read_rom(&mut self) -> u8 {
        let program_counter = self.read_r16(R16Address::PC);
        let byte = self.rom[program_counter as usize];
        self.write_r16(R16Address::PC, program_counter+1);
        byte
    }

    pub fn execute_instruction(&mut self) -> u64 {
        let opcode = self.read_rom();
        let cycles_run = match opcode {
            0x0u8 => 4, // NOP

            0x40u8 => self.load_r8_r8(R8Address::B, R8Address::B),
            0x41u8 => self.load_r8_r8(R8Address::B, R8Address::C),
            0x42u8 => self.load_r8_r8(R8Address::B, R8Address::D),
            0x43u8 => self.load_r8_r8(R8Address::B, R8Address::E),
            0x44u8 => self.load_r8_r8(R8Address::B, R8Address::H),
            0x45u8 => self.load_r8_r8(R8Address::B, R8Address::L),
            0x46u8 => self.load_r8_ram(R8Address::B),
            0x47u8 => self.load_r8_r8(R8Address::B, R8Address::A),
            0x48u8 => self.load_r8_r8(R8Address::C, R8Address::B),
            0x49u8 => self.load_r8_r8(R8Address::C, R8Address::C),
            0x4Au8 => self.load_r8_r8(R8Address::C, R8Address::D),
            0x4Bu8 => self.load_r8_r8(R8Address::C, R8Address::E),
            0x4Cu8 => self.load_r8_r8(R8Address::C, R8Address::H),
            0x4Du8 => self.load_r8_r8(R8Address::C, R8Address::L),
            0x4Eu8 => self.load_r8_ram(R8Address::C),
            0x4Fu8 => self.load_r8_r8(R8Address::C, R8Address::A),

            0x50u8 => self.load_r8_r8(R8Address::D, R8Address::B),
            0x51u8 => self.load_r8_r8(R8Address::D, R8Address::C),
            0x52u8 => self.load_r8_r8(R8Address::D, R8Address::D),
            0x53u8 => self.load_r8_r8(R8Address::D, R8Address::E),
            0x54u8 => self.load_r8_r8(R8Address::D, R8Address::H),
            0x55u8 => self.load_r8_r8(R8Address::D, R8Address::L),
            0x56u8 => self.load_r8_ram(R8Address::D),
            0x57u8 => self.load_r8_r8(R8Address::D, R8Address::A),
            0x58u8 => self.load_r8_r8(R8Address::E, R8Address::B),
            0x59u8 => self.load_r8_r8(R8Address::E, R8Address::C),
            0x5Au8 => self.load_r8_r8(R8Address::E, R8Address::D),
            0x5Bu8 => self.load_r8_r8(R8Address::E, R8Address::E),
            0x5Cu8 => self.load_r8_r8(R8Address::E, R8Address::H),
            0x5Du8 => self.load_r8_r8(R8Address::E, R8Address::L),
            0x5Eu8 => self.load_r8_ram(R8Address::E),
            0x5Fu8 => self.load_r8_r8(R8Address::E, R8Address::A),

            0x60u8 => self.load_r8_r8(R8Address::H, R8Address::B),
            0x61u8 => self.load_r8_r8(R8Address::H, R8Address::C),
            0x62u8 => self.load_r8_r8(R8Address::H, R8Address::D),
            0x63u8 => self.load_r8_r8(R8Address::H, R8Address::E),
            0x64u8 => self.load_r8_r8(R8Address::H, R8Address::H),
            0x65u8 => self.load_r8_r8(R8Address::H, R8Address::L),
            0x66u8 => self.load_r8_ram(R8Address::H),
            0x67u8 => self.load_r8_r8(R8Address::H, R8Address::A),
            0x68u8 => self.load_r8_r8(R8Address::L, R8Address::B),
            0x69u8 => self.load_r8_r8(R8Address::L, R8Address::C),
            0x6Au8 => self.load_r8_r8(R8Address::L, R8Address::D),
            0x6Bu8 => self.load_r8_r8(R8Address::L, R8Address::E),
            0x6Cu8 => self.load_r8_r8(R8Address::L, R8Address::H),
            0x6Du8 => self.load_r8_r8(R8Address::L, R8Address::L),
            0x6Eu8 => self.load_r8_ram(R8Address::L),
            0x6Fu8 => self.load_r8_r8(R8Address::L, R8Address::A),

            0x70u8 => self.load_ram_r8(R8Address::B),
            0x71u8 => self.load_ram_r8(R8Address::C),
            0x72u8 => self.load_ram_r8(R8Address::D),
            0x73u8 => self.load_ram_r8(R8Address::E),
            0x74u8 => self.load_ram_r8(R8Address::H),
            0x75u8 => self.load_ram_r8(R8Address::L),
            // HALT
            0x77u8 => self.load_ram_r8(R8Address::A),
            0x78u8 => self.load_r8_r8(R8Address::A, R8Address::B),
            0x79u8 => self.load_r8_r8(R8Address::A, R8Address::C),
            0x7Au8 => self.load_r8_r8(R8Address::A, R8Address::D),
            0x7Bu8 => self.load_r8_r8(R8Address::A, R8Address::E),
            0x7Cu8 => self.load_r8_r8(R8Address::A, R8Address::H),
            0x7Du8 => self.load_r8_r8(R8Address::A, R8Address::L),
            0x7Eu8 => self.load_r8_ram(R8Address::A),
            0x7Fu8 => self.load_r8_r8(R8Address::A, R8Address::A),

            0xCDu8 => 4 + self.execute_prefixed_instruction(),

            _ => {println!("instruction not found for opcode: {:02X}", opcode); 4},
        };
        cycles_run
    }

    fn execute_prefixed_instruction(&mut self) -> u64 {
        let opcode = self.read_rom();
        let cycles_run = match opcode {
            _ => {println!("prefixed instruction not found for opcode: {:02X}", opcode); 4},
        };
        cycles_run
    }

    fn load_r8_r8(&mut self, dest: R8Address, source: R8Address) -> u64 {
        let value = self.read_r8(source);
        self.write_r8(dest, value);
        4 // cycles run
    }

    fn load_r8_ram(&mut self, dest: R8Address) -> u64 {
        let ram_address = self.read_r16(R16Address::HL);
        let value = self.read_ram(ram_address);
        self.write_r8(dest, value);
        4 // cycles run
    }

    fn load_ram_r8(&mut self, source: R8Address) -> u64 {
        let ram_address = self.read_r16(R16Address::HL);
        let value = self.read_r8(source);
        self.write_ram(ram_address, value);
        4 // cycles run
    }

    #[allow(dead_code)]
    fn print_8bit_registers(&mut self) {
        println!(
            "{}",
            ALL_8BIT_ADDRESSES.map(|address|
                format!("{:?}: {:X}", address.clone(), self.read_r8(address))
            ).join(", ")
        );
    }

    #[allow(dead_code)]
    fn print_16bit_registers(&mut self) {
        println!(
            "{}",
            ALL_16BIT_ADDRESSES.map(|address|
                format!("{:?}: {:X}", address.clone(), self.read_r16(address))
            ).join(", ")
        );
    }

    #[allow(dead_code)]
    fn set_all_registers(&mut self, value: u8) {
        for register in self.registers.iter_mut() {
            *register = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_r8_r8() {
        let mut rom = Vec::new();
        rom.insert(0, 0x41u8);

        let mut cpu = Cpu::new(rom);
        cpu.set_all_registers(0);
        cpu.write_r8(R8Address::C, 1);

        assert_eq!(cpu.read_r8(R8Address::B), 0);
        cpu.execute_instruction();
        assert_eq!(cpu.read_r8(R8Address::B), 1);
    }

    #[test]
    fn test_load_r8_ram() {
        let mut rom = Vec::new();
        rom.insert(0, 0x46u8);

        let mut cpu = Cpu::new(rom);
        cpu.set_all_registers(0);
        cpu.write_ram(0x100, 1);
        cpu.write_r16(R16Address::HL, 0x100);

        assert_eq!(cpu.read_r8(R8Address::B), 0);
        cpu.execute_instruction();
        assert_eq!(cpu.read_r8(R8Address::B), 1);
    }
}