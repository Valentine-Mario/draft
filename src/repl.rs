use std;
use std::io;
use std::io::Write;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use crate::builder::codegen;
use crate::parser::*;
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
        println!("The Draft repl");
        let mut repl_string=String::from("");
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
                    println!("Exiting the repl!");
                    std::process::exit(0);
                }
                "load"=>{
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
                    
                    unsafe {
                        codegen(parsed_input);
                    }
                    let llvm_output = Command::new("sh").args(&["cmd.sh"]).output().expect("error running command");
                    println!("{:?}", io::stdout().write_all(&llvm_output.stdout).unwrap());
                }
                "history"=>{
                    println!("{}", repl_string);
                }
                "clear"=>{
                    repl_string=String::from("");
                }
                _=>{
                    repl_string.push_str(buffer);
                    repl_string.push_str("\n");
                    let parsed_input = arithmetic::expression(&repl_string).unwrap();
                    
                    unsafe {
                        codegen(parsed_input);
                    }
                    let llvm_output = Command::new("sh").args(&["cmd.sh"]).output().expect("error running command");
                    println!("{:?}", io::stdout().write_all(&llvm_output.stdout).unwrap());
                }
            }
        }
    }
}