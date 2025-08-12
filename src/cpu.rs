pub struct Cpu {
    registers: Vec<u8>,
    ram: Vec<u8>,
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

const ALL_16BIT_ADDRESSES: [R16Address; 6] = [
    R16Address::AF,
    R16Address::BC,
    R16Address::DE,
    R16Address::HL,
    R16Address::SP,
    R16Address::PC,
];

impl Cpu {
    pub fn new() -> Cpu {
        let registers = vec![0; 12];
        let ram = vec![0; 65536];
        Cpu { registers, ram }
    }

    pub fn print_8bit_registers(&mut self) {
        println!(
            "{}",
            ALL_8BIT_ADDRESSES.map(|address|
                format!("{:?}: {:X}", address.clone(), self.read_r8(address))
            ).join(", ")
        );
    }

    pub fn print_16bit_registers(&mut self) {
        println!(
            "{}",
            ALL_16BIT_ADDRESSES.map(|address|
                format!("{:?}: {:X}", address.clone(), self.read_r16(address))
            ).join(", ")
        );
    }

    pub fn set_all_registers(&mut self, value: u8) {
        for register in self.registers.iter_mut() {
            *register = value;
        }
    }

    fn read_r8(&mut self, address: R8Address) -> u8 {
        self.registers[address as usize]
    }

    fn read_r16(&mut self, address: R16Address) -> u16 {
        let register_index = address as usize;
        let high = self.registers[register_index] as u16;
        let low = self.registers[register_index + 1] as u16;
        (high << 8) | low
    }

    fn write_r8(&mut self, address: R8Address, value: u8) {
        self.registers[address as usize] = value;
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

    pub fn debug_routine(&mut self) {
        self.print_8bit_registers();
        self.print_16bit_registers();
        println!();

        self.set_all_registers(0xff);
        self.print_8bit_registers();
        self.print_16bit_registers();
        println!();

        self.write_r8(R8Address::H, 0x18);
        self.print_8bit_registers();
        self.print_16bit_registers();
        println!();

        self.write_r16(R16Address::AF, 0xabcd);
        self.print_8bit_registers();
        self.print_16bit_registers();
        println!();

        println!("{:X}", self.read_ram(0x0000));
        self.write_ram(0x0000, 42);
        println!("{:X}", self.read_ram(0x0000));
        println!();
    }
}