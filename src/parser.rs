use core::panic;
use std::collections::HashMap;
use std::default;
use serde::{Deserialize, Serialize};

use crate::lexer::{LexerToken, Lexer};
use crate::value::Value;
use crate::expr::{AstExpr};

pub struct Parser {
    input: Vec<LexerToken>,
    tokens: Vec<ParserToken>
}

#[derive(Debug)]
pub enum ParserToken {
    DeclareVariable(String), // Pops a value from stack and stores it to stack
    GetVariable(String),     // Pushes the variables value to stack
    Operation(String),       // Pops 2 values from stack as arguments and pushes a result
    Push(Value),
    Pop()
}

impl Parser {
    pub fn parse(mut tokens: Vec<LexerToken>) -> Vec<ParserToken> {
        tokens.reverse(); // ! has to be done :D, vec doesn't have pop_front and too lazy to refactor to use VecDeque
        let mut parser = Parser::new(tokens);

        'parse_loop : loop {
            let token_opt = parser.peek();
            if token_opt.is_none() {
                println!("Eof");
                break 'parse_loop;
            }
            let token = token_opt.unwrap();
            if let LexerToken::Keyword(kw) = token {
                match kw.as_str() {
                    "let" => { parser.variable_decleration(); },
                    _ => { panic!("Unimplumented keyword {}", kw); }
                }
            }
            else 
            {
                return AstExpr::evaluate(&mut parser.input);
            }

        }

        parser.tokens
    }

    fn variable_decleration(&mut self) {
        // 'let' keyword
        self.eat().unwrap();
        
        // identifier
        let tk_identifier = self.eat().expect("expected an identifier after 'let' keyword");
        if let LexerToken::Identifier(identifier) = tk_identifier {
            // symbol '='
            let symbol = self.eat().expect("expected an identifier after 'let' keyword");
            if symbol != LexerToken::Symbol('=') {
                panic!("'=' expected got '{:?}' instead", symbol);
            }
            
            // Get expression
            let mut expr = self.eat_expr(LexerToken::Symbol(';'));
            let mut evaluated = AstExpr::evaluate(&mut expr);
            self.tokens.append(&mut evaluated);
            self.tokens.push(ParserToken::DeclareVariable(identifier.clone()));
            return;
        }
        panic!("expected an identifier after 'let' keyword");
    }

    /**
     * Terminator is used to determine when the expression is suppost to end, terminator gets eaten. e.g: 
     * "LexerToken::Symbol(';')" for "let x = 2+2;"
     * "LexerToken::Symbol(',')" for "fn foo(2+2+2, 0)"
     * "LexerToken::Operator(')')" for "fn foo(2+2+2)" // this is going to be a fucking problem, lol.
     */
    fn eat_expr(&mut self, terminator: LexerToken) -> Vec<LexerToken> {
        let mut out_tks = vec![];
        'get_tokens: loop {
            let eat_opt = self.eat();
            if eat_opt.is_none() {
                panic!("Expected '{:?}' got EOF instead!", terminator);
            }
        
            let token = eat_opt.unwrap().clone();
            if token == terminator {
                break 'get_tokens;
            }
            out_tks.push(token);
        }

        return out_tks;
    }

    fn peek(&self) -> Option<&LexerToken> {
        if self.input.len() == 0 {
            return None;
        }
        let idx = self.input.len() - 1;
        self.input.get(idx)
    }

    fn eat(&mut self) -> Option<LexerToken> {
        self.input.pop()
    }

    fn new(tks: Vec<LexerToken>) -> Parser { 
        Parser {
            input: tks,
            tokens: vec![]
        }
    }
}
