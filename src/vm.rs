use crate::instructions::Opcode;
pub struct VM {
    pub registers: [i32; 64],
    pub pc: usize,
    pub program: Vec<u8>,
    pub remainder: u32,
}
impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 64],
            pc: 0,
            program: vec![],
            remainder: 0,
        }
    }
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }
    #[deny(clippy::never_loop)]
    pub fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }
        match self.decode_opcode() {
            Opcode::HLT => {
                return false;
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize; // We cast to usize so we can use it as an index into the array
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32; // Our registers are i32s, so we need to cast it. We'll cover that later.
            }
            Opcode::MULT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            }
            Opcode::JMP => {
                let register = self.registers[self.next_8_bits() as usize];
                self.pc = register as usize;
            }
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc += value as usize;
            }
            Opcode::JMPB => {
                let register = self.registers[self.next_8_bits() as usize];
                self.pc -= register as usize;
            }
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let dest = self.next_8_bits() as usize;
                if register1 == register2 {
                    self.registers[dest] = 1;
                } else {
                    self.registers[dest] = 0;
                }
            }
            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let dest = self.next_8_bits() as usize;
                if register1 != register2 {
                    self.registers[dest] = 1;
                } else {
                    self.registers[dest] = 0;
                }
            }
            Opcode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let dest = self.next_8_bits() as usize;
                if register1 > register2 {
                    self.registers[dest] = 1;
                } else {
                    self.registers[dest] = 0;
                }
            }
            Opcode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let dest = self.next_8_bits() as usize;
                if register1 < register2 {
                    self.registers[dest] = 1;
                } else {
                    self.registers[dest] = 0;
                }
            }
            Opcode::GTE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let dest = self.next_8_bits() as usize;
                if register1 >= register2 {
                    self.registers[dest] = 1;
                } else {
                    self.registers[dest] = 0;
                }
            }
            Opcode::LTE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let dest = self.next_8_bits() as usize;
                if register1 <= register2 {
                    self.registers[dest] = 1;
                } else {
                    self.registers[dest] = 0;
                }
            }
            Opcode::JEE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let dest = self.next_8_bits() as usize;
                if register1 == register2 {
                    self.pc = self.registers[dest] as usize;
                }
            }
            Opcode::PRINT => {
                let register = self.registers[self.next_8_bits() as usize];
                println!("{}", register);
                self.next_16_bits();
            }
            _ => {
                println!("Unrecognized opcode found! Terminating!");
                return false;
            }
        }
        true
    }
    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }
    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }
    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        //convert 500 to 2 bytes
        let number = 500;
        let byte1 = (number >> 8) as u8;
        let byte2 = number as u8;
        test_vm.program = vec![3, 0, byte1, byte2]; // Remember, this is how we represent 500 using two u8s in little endian format
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }
    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 2];
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 20;
        test_vm.run();
        assert_eq!(test_vm.registers[2], 30);
    }
    #[test]
    fn test_opcode_sub() {
        let mut test_vm = VM::new();
        test_vm.program = vec![2, 0, 1, 2];
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.run();
        assert_eq!(test_vm.registers[2], 10);
    }
    #[test]
    fn test_opcode_mult() {
        let mut test_vm = VM::new();
        test_vm.program = vec![4, 0, 1, 2];
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.run();
        assert_eq!(test_vm.registers[2], 200);
    }
    #[test]
    fn test_opcode_div() {
        let mut test_vm = VM::new();
        test_vm.program = vec![5, 0, 1, 2];
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.run();
        assert_eq!(test_vm.registers[2], 2);
        assert_eq!(test_vm.remainder, 0);
    }
    #[test]
    fn test_opcode_jmp() {
        let mut test_vm = VM::new();
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.registers[0] = 2;
        test_vm.run();
        assert_eq!(test_vm.pc, 2);
    }
    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 2;
        test_vm.program = vec![7, 1, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }
    #[test]
    fn test_opcode_jmpb() {
        let mut test_vm = VM::new();
        test_vm.program = vec![8, 0, 0, 0];
        test_vm.registers[0] = 2;
        test_vm.run();
        assert_eq!(test_vm.pc, 0);
    }
    #[test]
    fn test_opcode_eq() {
        let mut test_vm = VM::new();
        test_vm.program = vec![9, 0, 1, 2, 9, 0, 1, 2];
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
        test_vm.registers[0] = 9;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
    }
    #[test]
    fn test_opcode_neq() {
        let mut test_vm = VM::new();
        test_vm.program = vec![10, 0, 1, 2, 10, 0, 1, 2];
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
        test_vm.registers[0] = 9;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
    }
    #[test]
    fn test_opcode_gt() {
        let mut test_vm = VM::new();
        test_vm.program = vec![11, 0, 1, 2, 11, 0, 1, 2, 11, 0, 1, 2];
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
        test_vm.registers[0] = 9;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
        test_vm.registers[0] = 11;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
    }
    #[test]
    fn test_opcode_lt() {
        let mut test_vm = VM::new();
        test_vm.program = vec![12, 0, 1, 2, 12, 0, 1, 2, 12, 0, 1, 2];
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
        test_vm.registers[0] = 9;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
        test_vm.registers[0] = 11;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
    }
    #[test]
    fn test_opcode_gte() {
        let mut test_vm = VM::new();
        test_vm.program = vec![13, 0, 1, 2, 13, 0, 1, 2, 13, 0, 1, 2];
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
        test_vm.registers[0] = 9;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
        test_vm.registers[0] = 11;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
    }
    #[test]
    fn test_opcode_lte() {
        let mut test_vm = VM::new();
        test_vm.program = vec![14, 0, 1, 2, 14, 0, 1, 2, 14, 0, 1, 2];
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
        test_vm.registers[0] = 9;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
        test_vm.registers[0] = 11;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
    }
    #[test]
    fn test_opcode_jee() {
        let mut test_vm = VM::new();
        test_vm.program = vec![15, 0, 1, 2, 15, 0, 1, 2, 15, 0, 1, 2];
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.registers[2] = 6;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 6);
        test_vm.pc = 4;
        test_vm.registers[0] = 9;
        test_vm.registers[1] = 10;
        test_vm.registers[2] = 4;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 8);
        test_vm.registers[0] = 11;
        test_vm.registers[1] = 10;
        test_vm.registers[2] = 4;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 12);
    }
}
