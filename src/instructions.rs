#[derive(Debug, PartialEq, Eq)]
pub enum Opcode {
    HLT,
    ADD,
    SUB,
    LOAD,
    MULT,
    DIV,
    JMP,
    JMPF,
    JMPB,
    EQ,
    NEQ,
    GT,
    LT,
    GTE,
    LTE,
    JEE,
    PRINT,
    IGL,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    opcode: Opcode,
}
impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}
impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::HLT,
            1 => Opcode::ADD,
            2 => Opcode::SUB,
            3 => Opcode::LOAD,
            4 => Opcode::MULT,
            5 => Opcode::DIV,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GT,
            12 => Opcode::LT,
            13 => Opcode::GTE,
            14 => Opcode::LTE,
            15 => Opcode::JEE,
            16 => Opcode::PRINT,
            _ => Opcode::IGL,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
