use crate::lexer::assembler::get_token;
use crate::lexer::Token;
use crate::vm::VM;
use std::io;
use std::io::Write;

pub struct AsmRepl {
    vm: VM,
    command_buffer: Vec<String>,
}
impl Default for AsmRepl {
    fn default() -> Self {
        Self::new()
    }
}
impl AsmRepl {
    pub fn new() -> AsmRepl {
        AsmRepl {
            vm: VM::new(),
            command_buffer: Vec::new(),
        }
    }
    pub fn run(&mut self) {
        println!("Starting ASM REPL");
        loop {
            let mut buffer = String::new();

            // Blocking call until the user types in a command
            let stdin = io::stdin();

            // Annoyingly, `print!` does not automatically flush stdout like `println!` does, so we
            // have to do that there for the user to see our `>>> ` prompt.
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            // Here we'll look at the string the user gave us.
            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".quit" => {
                    println!("Exiting...");
                    std::process::exit(0);
                }
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                "" => {
                    continue;
                }
                ".program" => {
                    println!("Listing instructions currently in VM's program vector:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of Program Listing");
                }
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of Register Listing")
                }
                ".clear" => {
                    self.vm.registers = [0; 64];
                    self.vm.program = vec![];
                    self.vm.pc = 0;
                }
                _ => {
                    if buffer.starts_with('.') {
                        println!("Unknown command: {}", buffer);
                        continue;
                    }
                    let token = get_token(buffer);
                    //check if token is an error

                    match token {
                        Ok(token) => {
                            //extract the opcode from the token
                            let mut program: Vec<u8> = Vec::new();
                            match token.opcode {
                                Token::Op { code } => {
                                    //push the opcode to the program vector
                                    program.push(code as u8);
                                }
                                _ => {
                                    println!("Invalid token");
                                }
                            }
                            match token.operand1 {
                                Some(Token::Register { reg_num }) => {
                                    program.push(reg_num);
                                }
                                Some(Token::IntegerOperand { value }) => {
                                    program.push(value as u8);
                                }
                                _ => {
                                    println!("Invalid token");
                                }
                            }
                            match token.operand2 {
                                Some(Token::Register { reg_num }) => {
                                    program.push(reg_num);
                                }
                                Some(Token::IntegerOperand { value }) => {
                                    program.push(value as u8);
                                }
                                _ => {
                                    program.push(0);
                                }
                            }
                            match token.operand3 {
                                Some(Token::Register { reg_num }) => {
                                    program.push(reg_num);
                                }
                                Some(Token::IntegerOperand { value }) => {
                                    program.push(value as u8);
                                }
                                _ => {
                                    //move program[2] to program[3]
                                    program.push(program[2]);
                                    //set program[2] to 0
                                    program[2] = 0;
                                }
                            }
                            //push the program vector to the vm's program vector
                            self.vm.program.append(&mut program);
                            self.vm.run_once();
                        }
                        Err(e) => {
                            println!(
                                "Error! Likely An Invalid Instruction Or Not Enough Arguments CODE: {:?}",
                                e
                            );
                        }
                    }
                }
            }
        }
    }
}
