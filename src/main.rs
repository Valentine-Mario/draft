
extern crate peg;
extern crate llvm_sys as llvm;
mod builder;
use crate::builder::codegen;
mod parser;
use crate::parser::*;
use std::fs::File;
use std::io::Read;
mod repl;
use crate::repl::*;
use std::env::args;
use std::path::Path;
use std::process::Command;
use std::io;
use std::io::Write;


fn main() {
    let mut args = args();
    if  args.len() < 2 {
        let mut repl= REPL::new();
        repl.run();
    }else{
        args.next();
       match args.next() {
            Some(arg) => {
                let mut input = String::new();
                let mut f = File::open(Path::new(&arg)).unwrap();
                f.read_to_string(&mut input).unwrap();
                //lli-3.9 out.ll ; echo $?

                let parsed_input = arithmetic::expression(&input).unwrap();
               // println!("{:?}", parsed_input);
                
                unsafe {
                    codegen(parsed_input);
                }
                let llvm_output = Command::new("sh").args(&["cmd.sh"]).output().expect("error running command");
                println!("{:?}", io::stdout().write_all(&llvm_output.stdout).unwrap());
            },
            None => println!("Didn't get a query string"),
        };
    }
    
}



    
    