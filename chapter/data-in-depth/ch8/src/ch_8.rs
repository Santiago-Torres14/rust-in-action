//! Rust in action
//! chapter 5
//! Implementing a CPU to establish that functions are also data

/// 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0
/// 15 - 12 = instruction
/// 11 - 8 = first value
/// 7 - 4 = second value
/// 3 - 0 = operand
pub struct Cpu {
    pub current_operation: u16,
    pub registers: [u8; 2],
}

impl Cpu {
    fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    fn run(&mut self) {
        // loop {
        let opcode = self.read_opcode();

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = (opcode & 0x000F) as u8;

        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode {:04x}", opcode),
        }
        // }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}
