use crate::memory_bus::MemoryBus;
use crate::instruction::Instruction;
use crate::instruction::ArithmeticTarget;

struct CPU {
    pc: u16,
    bus: MemoryBus,
    registers: Registers
}

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

impl CPU {
    fn step(&mut self) {
        let mut byte = self.bus.read_byte(self.pc);
        let instruction = Instruction::from_byte(byte);
        let next_pc = self.execute(instruction);
        self.pc = next_pc;
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(ArithmeticTarget::C) => {
                let result = self.registers.a.wrapping_add(self.registers.c);
                self.registers.a = result;
                return self.pc + 1; // need to keep track of reading bytes and words and stuf
            },
            Instruction::ADD(_) => {
                return self.pc;
            },
            Instruction::INC(_) => {
                return self.pc;
            }
        }
    }
}