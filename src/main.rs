pub mod instructions;
pub mod repl;
pub mod vm;
use repl::REPL;
use std::env;
use std::io;
#[macro_use]
extern crate nom;
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
                println!("Assembler REPL not implemented yet");
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
