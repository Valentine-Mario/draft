use crate::lexer::*;
//> ch-0
use crate::parser::*;
use std::io;
use std::io::Write;

pub use self::Stage::{
       AST,
       Tokens
    };
#[derive(PartialEq, Clone, Debug)]
pub enum Stage {
    AST,
    Tokens
}

pub fn main_loop(stage: Stage) {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut parser_settings = default_parser_settings();

    'main: loop {
        print!("> ");
        stdout.flush().unwrap();
        input.clear();
        stdin.read_line(&mut input).ok().expect("Failed to read line");
        if input.as_str() == ".quit\n" {
            break;
        }

        // the constructed AST
        let mut ast = Vec::new();
        // tokens left from the previous lines
        let mut prev = Vec::new();
        loop {
            let tokens = tokenize(input.as_str());
            if stage == Tokens {
                println!("{:?}", tokens);
                continue 'main
            }
            prev.extend(tokens.into_iter());

            let parsing_result = parse(prev.as_slice(), ast.as_slice(), &mut parser_settings);
            match parsing_result {
                Ok((parsed_ast, rest)) => {
                    ast.extend(parsed_ast.into_iter());
                    if rest.is_empty() {
                        // we have parsed a full expression
                        break
                    } else {
                        prev = rest;
                    }
                },
                Err(message) => {
                    println!("Error occured: {}", message);
                    continue 'main
                }
            }
            print!(". ");
            stdout.flush().unwrap();
            input.clear();
            stdin.read_line(&mut input).ok().expect("Failed to read line");
        }

        if stage == AST {
            println!("{:?}", ast);
            continue
        }
    }
}