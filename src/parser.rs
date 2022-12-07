use core::panic;
use std::collections::HashMap;
use std::default;
use serde::{Deserialize, Serialize};

use crate::lexer::{LexerToken, Lexer};
use crate::interpreter::{Interpreter};
use crate::value::Value;
use crate::expr::{AstExpr};

pub struct Parser {
    input: Vec<LexerToken>,
    tokens: Vec<ParserToken>
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParserToken {
    DeclareVariable(String), // Pops a value from stack and stores it to stack
    GetVariable(String),     // Pushes the variables value to stack
    Operation(String),       // Pops 2 values from stack as arguments and pushes a result
    Push(Value),
    Pop(),
    Call(String, u8),        // Second argument for amount of arguments
    Ret(),
}

impl Parser {
    pub fn parse(mut tokens: Vec<LexerToken>) -> Vec<ParserToken> {
        tokens.reverse(); // ! has to be done :D, vec doesn't have pop_front and too lazy to refactor to use VecDeque
        let mut parser = Parser::new(tokens);

        'parse_loop : loop {
            let token = parser.peek().unwrap();
            
            if let LexerToken::Keyword(kw) = token {
                match kw.as_str() {
                    "let" => { parser.variable_decleration(); },
                    _ => { panic!("Unimplumented keyword {}", kw); }
                }
            }
            else if let LexerToken::Identifier(fn_name) = token.clone() {
                parser.eat(); // Identifier
                let next = parser.eat().expect("Syntax error");
                if LexerToken::Operator("(".to_string()) == next {
                    parser.function_call(fn_name.clone());
                }
            }
            else 
            {
                return AstExpr::evaluate(&mut parser.input);
            }

            if parser.input.len() == 0 {
                break 'parse_loop;
            }
        }

        parser.tokens
    }

    fn variable_decleration(&mut self) {
        // 'let' is already eaten
        self.eat();

        // identifier
        let tk_identifier = self.eat().expect("expected an identifier after 'let' keyword");
        if let LexerToken::Identifier(identifier) = tk_identifier {
            // symbol '='
            self.eat_expect(LexerToken::Symbol('='));
            
            // Get expression
            let mut expr = self.eat_expr(vec![LexerToken::Symbol(';')]);
            let mut evaluated = AstExpr::evaluate(&mut expr);
            self.tokens.append(&mut evaluated);
            self.tokens.push(ParserToken::DeclareVariable(identifier.clone()));

            // symbol ';'
            self.eat_expect(LexerToken::Symbol(';'));
            return;
        }
        panic!("expected an identifier after 'let' keyword");
    }

    fn function_call(&mut self, fn_name: String) {
        // TODO: Parse Arguments
        let rparen = self.eat().expect("Syntax Error");
        if rparen != LexerToken::Operator(")".to_string()) {
            panic!("Expected ')' got {:?} instead!", rparen);
        }

        self.tokens.push(ParserToken::Call(fn_name, 0));
    }

    /**
     * Terminator is used to determine when the expression is suppost to end, terminator doesn't get eaten. e.g: 
     * "LexerToken::Symbol(';')" for "let x = 2+2;"
     * "LexerToken::Symbol(',')" for "fn foo(2+2+2, 0)"
     * "LexerToken::Operator(')')" for "fn foo(2+2+2)" // this is going to be a fucking problem, lol.
     */
    fn eat_expr(&mut self, terminator: Vec<LexerToken>) -> Vec<LexerToken> {
        let mut out_tks = vec![];
        'get_tokens: loop {
            let peeked = self.peek();
            if peeked.is_none() {
                panic!("Expected '{:?}' got EOF instead!", terminator);
            }
            if terminator.contains(peeked.unwrap()) {
                break 'get_tokens;
            }

            let token = self.eat().unwrap();
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

    fn eat_checked(&mut self) -> LexerToken {
        let popped = self.input.pop();
        if popped.is_none() {
            panic!("Got unexpected EOF");
        }
        popped.unwrap()
    }

    fn eat_expect(&mut self, expect: LexerToken) -> LexerToken {
        let popped = self.input.pop();
        if popped.is_none() {
            panic!("Expected {:?} got EOF instead!", expect);
        }
        let tk = popped.unwrap();
        if tk != expect {
            panic!("Expected {:?} got {:?} instead! :(", expect, tk);
        }
        tk
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
