#![allow(dead_code)]
#![allow(unused_imports)]

use std::{process::exit, io::{self, Write}};
use serde::{Serialize, Deserialize};

mod value;
mod lexer;
mod parser;
mod interpreter;

use value::{Value, ValueAdder};
use lexer::{LexerToken};
use parser::{AstNode};

fn get_line() -> Result<String, String> {
    // 
    print!(" >");
    io::stdout().flush().unwrap();

    // get line
    let mut line = String::from("");
    io::stdin().read_line(&mut line).expect("error reading stdin");
    Ok(line)
}

fn main() {
    println!("Giffi's awesome intepreter has been started");
    println!("Exit by typin \"quit()\"");

    let mut interpreter = interpreter::Interpreter::new();
    'running : loop {
        let st = get_line();
        if st.is_ok() {
            let l = st.unwrap().replace("\n", "");
            if l == "quit()" {
                break 'running;
            }
            else {
                let opt = lexer::Lexer::lex(l).unwrap();
                println!("Lexer: {:?}", opt);
                
                let tokens = parser::Parser::parse(opt);
                println!("Parser: {:?}", tokens);
                
                
                interpreter.execute_tokens(&tokens);
                // std::fs::write("./output.json", serde_json::to_string_pretty(&tokens).unwrap().as_str()).expect("error writing to disk");
            }
        }
        else {
            println!("An error occurred {}", st.unwrap_err());
        }
    }
}
