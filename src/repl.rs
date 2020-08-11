use std;
use std::io;
use std::io::Write;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use crate::builder::codegen;
use crate::parser::*;
use crate::parser::Expression;
use std::process::Command;

pub struct REPL{
    command:Vec<String>
}

impl REPL{
    pub fn new()->REPL{
        REPL{
            command:vec![]
        }
    }

    pub fn run(&mut self) {
        println!("The draft repl");
        loop{
            let mut buffer = String::new();
            let stdin = io::stdin();
    
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");
    
            stdin.read_line(&mut buffer).expect("Unable to read line from user");
            let buffer = buffer.trim();

            self.command.push(buffer.to_string());
            match buffer {
                "quit"=>{
                    println!("Farewell! Have a great day!");
                    std::process::exit(0);
                }
                "load_draft"=>{
                    print!("Please enter the path to the file you wish to load: ");
                    io::stdout().flush().expect("Unable to flush stdout");
                    let mut tmp = String::new();
                    stdin.read_line(&mut tmp).expect("Unable to read line from user");
                    let tmp = tmp.trim();
                    let filename = Path::new(&tmp);
                    let mut f = File::open(Path::new(&filename)).expect("File not found");
                    let mut contents = String::new();
                    f.read_to_string(&mut contents).expect("There was an error reading from the file");
                    let parsed_input = arithmetic::expression(&contents).unwrap();
                    println!("{:?}", parsed_input);
                    
                    unsafe {
                        codegen(parsed_input);
                    }
                }
                _=>{
                    let parsed_input = arithmetic::expression(&buffer).unwrap();
                    println!("{:?}", parsed_input);
                    
                    unsafe {
                        codegen(parsed_input);
                    }
                    let llvm_output = Command::new("cat").args(&["out.ll"]).output().expect("error running command");
                    println!("{:?}", io::stdout().write_all(&llvm_output.stdout).unwrap());
                }
            }
        }
    }
}