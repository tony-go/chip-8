const NOOP_OPCODE: u16 = 0;

struct CPU {
    current_opcode: u16,
    registers: [u8; 2]
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        self.current_opcode 
    }

    fn run(& mut self) {
        let opcode = self.read_opcode();

        // select nible with & (AND) operator
        // then move the bits to lowest place
        let c = ((opcode & 0xF000) >> 12) as u8; // opcode group
        let x = ((opcode & 0x0F00) >> 8) as u8; // register x
        let y = ((opcode & 0x00F0) >> 4) as u8; // register y
        let d = ((opcode & 0x000F) >> 0) as u8; // opcode subgroup

        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode {:04x}", opcode),
        }

        self.current_opcode = NOOP_OPCODE;
    }

    fn add_xy (&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}


fn main() {
    println!("Creating CPU...");
    let mut cpu = CPU {
        current_opcode: NOOP_OPCODE, // NOOP
        registers: [0; 2],
    };

    cpu.current_opcode = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 15;

    cpu.run();

    assert_eq!(cpu.registers[0], 15);
    println!("op finished!");
}
