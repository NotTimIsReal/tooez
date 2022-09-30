use crate::instructions::Opcode;
pub mod assembler;
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
}
#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    pub opcode: Token,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}
