
pub enum Instruction {
    ADD(ArithmeticTarget),
    INC(IncDecTarget),
    DEC(IncDecTarget),
    // need LD functions

    NOOP()
}

pub enum ArithmeticTarget {
    A, B, C, D, E, H, L
}

pub enum IncDecTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    BC,
    DE
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Instruction {
        match byte {
            0x00 => Instruction::NOOP(),
            0x02 => Instruction
            0x03 => Instruction::INC(IncDecTarget::BC),
            0x03 => Instruction::INC
            0x13 => Instruction::INC(IncDecTarget::DE),
            _ => Instruction::ADD(ArithmeticTarget::B)
        }
    }
}