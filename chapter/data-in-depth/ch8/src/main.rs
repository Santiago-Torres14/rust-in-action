mod ch_8;

fn main() {
    let mut cpu = ch_8::Cpu {
        current_operation: 0,
        registers: [0; 2],
    };

    cpu.current_operation = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
}

