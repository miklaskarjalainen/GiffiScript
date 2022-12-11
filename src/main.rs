#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unreachable_code)]

use std::{env, io::{self, Write}};

mod expr;
mod value;
mod lexer;
mod parser;
mod interpreter;
mod giffiscript;

fn get_line() -> String {
    // 
    print!(" >");
    io::stdout().flush().unwrap();

    // get line
    let mut line = String::from("");
    io::stdin().read_line(&mut line).expect("error reading stdin");
    line
}

fn main() {
    let mut machine = giffiscript::GiffiScript::new();
    
    // run file
    let mut args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let code = std::fs::read_to_string(&args[1]);
        if code.is_err() {
            panic!("Error occured when trying to read file: {}", code.unwrap_err());
        }
        machine.execute(code.unwrap());
        return;
    }
    
    // cmd interpreter
    println!("Giffi's awesome intepreter has been started");
    loop {
        let st = get_line();
        let code = st.replace("\n", "");
        machine.execute(code);
    }
}
