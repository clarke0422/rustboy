const CYCLES_PER_LINE: u16 = 456;

pub struct Ppu {
    cycles_run_during_line: u16,
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu { cycles_run_during_line: 0 }
    }

    pub fn advance( cycles: u16 ) {
    }
}