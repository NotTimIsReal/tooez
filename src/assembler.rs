use std::io::Write;
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instructions {
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JEQ,
    JNE,
    JLT,
    JLE,
    JGT,
    JGE,
    PUSH,
    POP,
    CALL,
    RET,
    PRINT,
    HALT,
    MEMSET,
    MEMDEL,
    NULL,
}
// enum Letters {
//     A,
//     B,
//     C,
//     D,
//     E,
//     F,
//     G,
//     H,
//     I,
//     J,
//     K,
//     L,
//     M,
//     N,
//     O,
//     P,
//     Q,
//     R,
//     S,
//     T,
//     U,
//     V,
//     W,
//     X,
//     Y,
//     Z,
// }
pub struct Assembler {
    instructions: Vec<Instructions>,
    labels: Vec<String>,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            instructions: Vec::new(),
            labels: Vec::new(),
        }
    }
    pub fn assemble(&mut self, content: Vec<&str>) {
        for line in content {
            let mut tokens = line.split_whitespace();
            let instruction = tokens.next().unwrap();
            //join the all the content afyer the first token
            let content = tokens.fuse().collect::<Vec<&str>>().join(" ");
            self.labels.push(content.to_string());
            match instruction {
                "ADD" => self.instructions.push(Instructions::ADD),
                "SUB" => self.instructions.push(Instructions::SUB),
                "MUL" => self.instructions.push(Instructions::MUL),
                "DIV" => self.instructions.push(Instructions::DIV),
                "JMP" => self.instructions.push(Instructions::JMP),
                "JEQ" => self.instructions.push(Instructions::JEQ),
                "JNE" => self.instructions.push(Instructions::JNE),
                "JLT" => self.instructions.push(Instructions::JLT),
                "JLE" => self.instructions.push(Instructions::JLE),
                "JGT" => self.instructions.push(Instructions::JGT),
                "JGE" => self.instructions.push(Instructions::JGE),
                "PUSH" => self.instructions.push(Instructions::PUSH),
                "POP" => self.instructions.push(Instructions::POP),
                "CALL" => self.instructions.push(Instructions::CALL),
                "RET" => self.instructions.push(Instructions::RET),
                "PRINT" => self.instructions.push(Instructions::PRINT),
                "MEMSET" => self.instructions.push(Instructions::MEMSET),
                "MEMDEL" => self.instructions.push(Instructions::MEMDEL),
                "HALT" => self.instructions.push(Instructions::HALT),
                _ => {
                    if instruction.starts_with(":") {
                        self.labels.push(instruction.to_string());
                    } else {
                        println!("Unknown instruction: {}", instruction);
                    }
                }
            }
        }
    }
    pub fn write(self) {
        //make directory called cache
        let err = std::fs::create_dir("cache");
        match err {
            Ok(_) => {}
            Err(_) => {}
        }
        let mut f = std::fs::File::create("cache/out.tzo").unwrap();
        let mut i: usize = 0;
        for instruction in self.instructions {
            match instruction {
                Instructions::ADD => f.write(&[0x01]).unwrap(),
                Instructions::SUB => f.write(&[0x02]).unwrap(),
                Instructions::MUL => f.write(&[0x03]).unwrap(),
                Instructions::DIV => f.write(&[0x04]).unwrap(),
                Instructions::JMP => f.write(&[0x05]).unwrap(),
                Instructions::JEQ => f.write(&[0x06]).unwrap(),
                Instructions::JNE => f.write(&[0x07]).unwrap(),
                Instructions::JLT => f.write(&[0x08]).unwrap(),
                Instructions::JLE => f.write(&[0x09]).unwrap(),
                Instructions::JGT => f.write(&[0x0a]).unwrap(),
                Instructions::JGE => f.write(&[0x0b]).unwrap(),
                Instructions::PUSH => f.write(&[0x0c]).unwrap(),
                Instructions::POP => f.write(&[0x0d]).unwrap(),
                Instructions::CALL => f.write(&[0x0e]).unwrap(),
                Instructions::RET => f.write(&[0x0f]).unwrap(),
                Instructions::PRINT => f.write(&[0x10]).unwrap(),
                Instructions::HALT => f.write(&[0x11]).unwrap(),
                Instructions::MEMSET => f.write(&[0x12]).unwrap(),
                Instructions::MEMDEL => f.write(&[0x13]).unwrap(),
                Instructions::NULL => f.write(&[0x00]).unwrap(),
            };
            let label = &self.labels[i];
            let out = f.write(&label.as_bytes());
            match out {
                Ok(_) => {}
                Err(e) => println!("Error: {}", e),
            };
            i += 1;
        }
    }
}
pub fn find_instruction(instruction: u8) -> Instructions {
    match instruction {
        0x01 => Instructions::ADD,
        0x02 => Instructions::SUB,
        0x03 => Instructions::MUL,
        0x04 => Instructions::DIV,
        0x05 => Instructions::JMP,
        0x06 => Instructions::JEQ,
        0x07 => Instructions::JNE,
        0x08 => Instructions::JLT,
        0x09 => Instructions::JLE,
        0x0a => Instructions::JGT,
        0x0b => Instructions::JGE,
        0x0c => Instructions::PUSH,
        0x0d => Instructions::POP,
        0x0e => Instructions::CALL,
        0x0f => Instructions::RET,
        0x10 => Instructions::PRINT,
        0x11 => Instructions::HALT,
        0x12 => Instructions::MEMSET,
        0x13 => Instructions::MEMDEL,
        0x00 => Instructions::NULL,
        _ => Instructions::NULL,
    }
}
