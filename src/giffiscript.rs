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
        let ltokens = Lexer::lex(code);
        println!("Lexer: {:?}", ltokens);

        let ptokens = Parser::parse(ltokens);
        println!("Parser: {:?}", ptokens);

        self.interpreter.execute_tokens(&ptokens);
    }
}
