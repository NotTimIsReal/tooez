use crate::assembler::{self, Instructions};
use std::{fs, process::exit};
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
    output: Vec<Vec<u8>>,
}
#[derive(Debug, Clone)]
struct Variable {
    name: u8,
    value: Vec<u8>,
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
    fn _find_variable(self, name: &str) -> Option<Variable> {
        for variable in self.memory.iter() {
            //convert name to u8
            if variable.name == name.as_bytes()[0] {
                return Some(variable.clone());
            }
        }
        None
    }
    pub fn run(mut self) {
        for i in 0..self._binary.len() {
            let exec = self._binary[self.pc];
            match exec {
                Instructions::PRINT => {
                    let length = self.instrction_maps[i].values.len();
                    if length < 1 {
                        println!("\n");
                        let new_line = vec![10]; //convert \n to u8

                        self.output.push(new_line);

                        continue;
                    };
                    let char_out: Vec<char> = self.instrction_maps[i]
                        .values
                        .iter()
                        .map(|x| *x as char)
                        .collect();

                    let out = char_out.iter().collect::<String>();
                    if out == "_" {
                        //convert \n to bytes
                        let new_line = vec![10];
                        self.output.push(new_line);
                        continue;
                    };
                    println!("{}", out);
                    //convert out to u8 vector
                    let out_bytes = out.as_bytes();
                    self.output.push(out_bytes.to_vec());
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
                    self.reg[i] = 0;
                    self.output
                        .push(format!("{}", self.reg[i]).as_bytes().to_vec());
                }
                Instructions::MEMSET => {
                    let length = self.instrction_maps[i].values.len();
                    if length < 2 {
                        println!(
                            "FAILED RUNNING, EXPECTED AT LEAST 2 ARGUMENTS FOR MEMSET GOT {}",
                            length
                        );
                        exit(1);
                    }
                    let map = &self.instrction_maps[i];
                    let name = map.values[0];
                    let mut values = map.values[1..].to_vec();
                    //convert values to string
                    let char_values: Vec<char> = values.iter().map(|x| *x as char).collect();

                    let value = char_values.iter().collect::<String>();

                    //split value by space
                    let split_value: Vec<&str> = value.split(" ").collect();
                    //convet Vec<&str> to Vec<str>
                    let mut split_value_string: Vec<String> = Vec::new();
                    for s in split_value {
                        split_value_string.push(s.to_string());
                    } //remove the first element
                    split_value_string.remove(0);
                    //check if split_value_string can split by _L
                    for i in 0..split_value_string.len() {
                        let refers_to_line = split_value_string[i].contains("_L");
                        if refers_to_line {
                            //get the index that _L is at
                            let line = split_value_string[0].split("_L").collect::<Vec<&str>>()[1];
                            if !line.parse::<i32>().is_ok() {
                                println!(
                                "FAILED RUNNING, EXPECTED INTEGER FOR MEMSET LINE REFERENCE GOT {}",
                                line
                            );
                                exit(1);
                            }
                            let line = line.parse::<usize>().unwrap();
                            let output = &self.output[line - 1];
                            //convert output to Vec<char>
                            let output_char: Vec<char> =
                                output.iter().map(|x| *x as char).collect();
                            //convert output_char to string
                            let output_string = output_char.iter().collect::<String>();
                            //split output_string by space
                            let split_output_string: Vec<&str> = output_string.split(" ").collect();
                            //convert split_output_string to Vec<str>
                            let mut split_output_string_string: Vec<String> = Vec::new();
                            for s in split_output_string {
                                split_output_string_string.push(s.to_string());
                            }
                            split_value_string = split_output_string_string;

                            break;
                        }
                        continue;
                    }

                    values = split_value_string
                        .iter()
                        .map(|x| x.bytes().next().unwrap())
                        .collect();
                    let variable = Variable {
                        name,
                        value: values,
                        type_of: find_type(&split_value_string[0]),
                    };
                    self.memory.push(variable);
                    self.output.push("".as_bytes().to_vec());
                }
                Instructions::MEMDEL => {
                    let length = self.instrction_maps[i].values.len();
                    if length < 1 {
                        println!(
                            "FAILED RUNNING, EXPECTED AT LEAST 1 ARGUMENTS FOR MEMDEL GOT {}",
                            length
                        );
                        exit(1);
                    }
                    let name = self.instrction_maps[i].values[0];
                    let mut found = false;
                    for i in 0..self.memory.len() {
                        if self.memory[i].name == name {
                            found = true;
                            self.memory.remove(i);
                            break;
                        }
                    }
                    if !found {
                        println!("FAILED RUNNING, EXPECTED VARIABLE {} NOT FOUND", name);
                        exit(1);
                    }
                    self.output.push("".as_bytes().to_vec());
                }
                _ => {
                    println!("Invalid Instruction");
                }
            }

            self.pc += 1;
        }
    }
}
fn find_type(value: &str) -> Types {
    if value.parse::<i32>().is_ok() {
        return Types::Number;
    }
    if value == "true" || value == "false" {
        return Types::Boolean;
    }
    return Types::String;
}
