use crate::lexer::{Lexer};
use crate::parser::{Parser};
use crate::interpreter::{Interpreter};

pub struct GiffiScript {
    interpreter: Interpreter
}

impl GiffiScript {
    pub fn new() -> GiffiScript{
        GiffiScript {
            interpreter: Interpreter::new()
        }
    }

    pub fn execute(&mut self, code: String) {
        use colored::Colorize;
        use std::time::Instant;

        println!("{}", format!("<---Lexing Started!--->").green().bold());
            let now = Instant::now();
            let ltokens = Lexer::lex(code);
            let end = Instant::now();
            let lexer_time = end - now;
            println!("Lexer Result: {:#?}", ltokens);
        println!("{}", format!(">---Lexing Ended!---<").green().bold());

        println!("{}", format!("<---Parsing Started!--->").cyan().bold());
            let now = Instant::now();
            let ptokens = Parser::parse(ltokens);
            let end = Instant::now();
            let parser_time = end - now;
        println!("Parser Result: {:#?}", ptokens);
        println!("{}", format!(">---Parsing Ended!---<").cyan().bold());
        
        let now = Instant::now();
        self.interpreter.execute_tokens(&ptokens);
        let end = Instant::now();
        let interpreting_time = end - now;

        println!("Lexing Time: {:?}", lexer_time);
        println!("Parsing Time: {:?}", parser_time);
        println!("Interpriting Time: {:?}", interpreting_time);
    }
}
