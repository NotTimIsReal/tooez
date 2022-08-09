use crate::assembler::{self, Instructions};
use std::fs;
#[derive(Debug, Clone)]
enum Types {
    String,
    Number,
    Boolean,
}
#[derive(Debug, Clone)]
struct InstructionMapping {
    instruction_index: usize,
    instuction: assembler::Instructions,
    values: Vec<u8>,
}
//construct a simple vm that consists of ram and a register
#[derive(Debug, Clone)]
pub struct VM {
    memory: Vec<Variable>,
    reg: Vec<i32>,
    pc: usize,
    sp: usize,
    instrction_maps: Vec<InstructionMapping>,
    _binary: Vec<Instructions>,
    output: Vec<u8>,
}
#[derive(Debug, Clone)]
struct Variable {
    name: u8,
    value: u8,
    type_of: Types,
}
impl VM {
    pub fn new() -> VM {
        VM {
            memory: Vec::new(),
            reg: Vec::new(),
            pc: 0,
            sp: 0,
            _binary: Vec::new(),
            instrction_maps: Vec::new(),
            output: Vec::new(),
        }
    }
    pub fn load(&mut self, filename: &str) {
        let file = fs::read_to_string(filename).unwrap();
        let content = file.as_bytes();
        for c in content {
            let instr = &assembler::find_instruction(*c);
            if *instr == Instructions::NULL {
                //get the last instruction
                let last_instruction = self.instrction_maps.last_mut().unwrap();
                last_instruction.values.push(*c);
            } else {
                let instruction_map = InstructionMapping {
                    instruction_index: self.instrction_maps.len(),
                    instuction: *instr,
                    values: Vec::new(),
                };
                self.instrction_maps.push(instruction_map);
                self._binary.push(*instr);
            }
        }
        //print the instructions and values
        // for instruction_map in self.instrction_maps.iter() {
        //     println!("{:?}", instruction_map.instuction);
        //     //convert the values from byte to a char
        //     let values: Vec<char> = instruction_map.values.iter().map(|x| *x as char).collect();
        //     println!("{:?}", values.iter().collect::<String>());
        // }
    }
    pub fn run(mut self) {
        for i in 0..self._binary.len() {
            let exec = self._binary[self.pc];
            match exec {
                Instructions::PRINT => {
                    let char_out: Vec<char> = self.instrction_maps[i]
                        .values
                        .iter()
                        .map(|x| *x as char)
                        .collect();
                    let out = char_out.iter().collect::<String>();
                    self.output.push(out.bytes().next().unwrap());
                    println!("{}", out);
                }
                Instructions::ADD => {
                    //set length of reg
                    for _ in 0..i + 1 {
                        self.reg.push(0);
                    }

                    let length = self.instrction_maps[i].values.len();
                    for j in 0..length {
                        let value = self.instrction_maps[i].values[j];
                        //convert value to number
                        let number = value as i32;
                        self.reg[i] += number;
                    }
                    self.output.push(self.reg[i] as u8);
                }
                _ => {
                    println!("Invalid Instruction");
                }
            }
            self.pc += 1;
        }
    }
}
