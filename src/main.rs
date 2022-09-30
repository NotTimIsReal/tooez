mod asm_repl;
pub mod instructions;
pub mod lexer;
pub mod vm;
pub mod vm_repl;
use std::env;
use std::io;
use vm_repl::REPL;
fn main() {
    //analysze args

    if env::args().count() != 2 {
        println!("Which REPL do you want to use?");
        println!("1. VM");
        println!("2. Assembler");
        println!("3. Language");
        //TODO: add more REPLs
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        //clear screen
        print!("{}[2J", 27 as char);
        match input {
            "1" => {
                let mut repl = REPL::new();
                repl.run();
            }
            "2" => {
                let mut repl = asm_repl::ASM_REPL::new();
                repl.run();
            }
            "3" => {
                println!("Language REPL not implemented yet");
            }
            _ => {
                println!("Invalid REPL");
            }
        }
    }
}
