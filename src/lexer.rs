use regex::Regex;

pub use self::Token::{
    Def,
    Extern,
    Delimiter,
    OpeningParenthesis,
    ClosingParenthesis,
    Comma,
    Ident,
    Number,
    Operator
};

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Def,
    Extern,
    Delimiter, //';' character
    OpeningParenthesis,
    ClosingParenthesis,
    Comma,
    Ident(String),
    Number(f64),
    Operator(String)
}

pub fn tokenize(input: &str) -> Vec<Token> {
    // regex for commentaries (start with #, end with the line end)
    let comment_re = Regex::new(r"(?m)#.*\n").unwrap();

    // remove commentaries from the input stream
    let preprocessed = comment_re.replace_all(input, "\n");
    let mut process_str:String="".to_string();
    process_str.push_str(&preprocessed);
    let mut result:Vec<Token> = Vec::new();

    // regex for token, just union of straightforward regexes for different token types
    // operators are parsed the same way as identifier and separated later
    let token_re = Regex::new(r"(?P<def>def)|(?P<extern>extern)|(?P<ident>\p{Alphabetic}\w*)|(?P<number>\d+\.?\d*)|(?P<delimiter>;)|(?P<oppar>\()|(?P<clpar>\))|(?P<comma>,)|(?P<operator>\S)").unwrap();

    for cap in token_re.captures_iter(&process_str) {
        let token = if cap.name("ident").is_some() {
            match cap.name("ident").unwrap() {
                ident => Ident(String::from(ident.as_str()))
            }
        } else if cap.name("number").is_some() {
            match cap.name("number").unwrap() {
                digit => Number(digit.as_str().parse().unwrap()),
            }
        }
         else if cap.name("delimiter").is_some() {
            Delimiter
        } else if cap.name("oppar").is_some() {
            OpeningParenthesis
        } else if cap.name("clpar").is_some() {
            ClosingParenthesis
        } else if cap.name("comma").is_some() {
            Comma
        }else if cap.name("def").is_some(){
            Def
        }else if cap.name("extern").is_some(){
            Extern
        }
         else {
            Operator(String::from(cap.name("operator").unwrap().as_str()))
        };

        result.push(token)
    }

    result
}