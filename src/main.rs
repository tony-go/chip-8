const NOOP_OPCODE: u16 = 0;

struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let pos = self.position_in_memory;
        let op_part1 = self.memory[pos] as u16;
        let op_part2 = self.memory[pos + 1] as u16;

        op_part1 << 8 | op_part2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            println!("position_in_memory: {:0x}", self.position_in_memory);
            println!("stack_pointer {}", self.stack_pointer);
            self.position_in_memory += 2;

            // select nible with & (AND) operator
            // then move the bits to lowest place
            let c = ((opcode & 0xF000) >> 12) as u8; // opcode group
            let x = ((opcode & 0x0F00) >> 8) as u8; // register x
            let y = ((opcode & 0x00F0) >> 4) as u8; // register y
            let d = ((opcode & 0x000F) >> 0) as u8; // opcode subgroup

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    println!("OP END");
                    return;
                }
                (0, 0, 0xE, 0xE) => {
                    println!("OP RET");
                    self.ret()
                },
                (0x2, _, _, _) => {
                    println!("OP CALL");
                    let nnn = opcode & 0x0FFF;
                    self.call(nnn);
                }
                (0x8, _, _, 0x4) => {
                    println!("OP ADD");
                    self.add_xy(x, y)
                },
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let x_value = self.registers[x as usize];
        let y_value = self.registers[y as usize];

        let (val, has_overflow) = x_value.overflowing_add(y_value);
        self.registers[x as usize] = val;

        if has_overflow {
            self.registers[0xF] = 1;
        }
        else {
            self.registers[0xF] = 1;
        }
    }

    fn call(&mut self, addr: u16) {
        let pointer = self.stack_pointer;
        let stack = &mut self.stack;

        if pointer > stack.len() {
            panic!("Stack overflow");
        }

        stack[pointer] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }
        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.position_in_memory = addr as usize;
    }
}

fn main() {
    println!("Creating CPU...");
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    mem[0x000] = 0x21; mem[0x001] = 0x00;
    mem[0x002] = 0x21; mem[0x003] = 0x00;
    mem[0x004] = 0x00; mem[0x005] = 0x00;

    mem[0x100] = 0x80; mem[0x101] = 0x14;
    mem[0x102] = 0x80; mem[0x103] = 0x14;
    mem[0x104] = 0x00; mem[0x105] = 0xEE;

    println!("{:?}", &mem[0x100..0x106]);

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("end");
}
