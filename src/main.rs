mod assembler;
mod vm;
use std::env;
use std::io::prelude::*;
use vm::VM;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    //remove args[0]
    let filename = &args[1];
    //open file
    let mut f = std::fs::File::open(filename).unwrap();
    //read file
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    //split file into lines
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut assem = assembler::Assembler::new();
    assem.assemble(lines);
    assem.write();
    let mut vm = VM::new();
    vm.load("cache/out.tzo");
    vm.run();
}
