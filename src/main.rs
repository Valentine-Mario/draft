mod lexer;
use crate::lexer::*;
mod parser;
use crate::parser::*;
mod driver;
use crate::driver::*;
use docopt::Docopt;
use serde::Deserialize;
const USAGE: &'static str = "
Usage: iron_kaleidoscope [(-l | -p | -i)]

Options:
    -l  Run only lexer and show its output.
    -p  Run only parser and show its output.
    -i  Run only IR builder and show its output.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_l: bool,
    flag_p: bool,
    flag_i: bool
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let stage = if args.flag_l {
        Tokens
    } else {
        AST
    };

    main_loop(stage);
}
