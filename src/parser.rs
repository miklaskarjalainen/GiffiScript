use core::panic;
use std::collections::HashMap;
use std::default;
use serde::{Deserialize, Serialize};

use crate::lexer::{LexerToken, Lexer};
use crate::value::Value;
use crate::expr::{AstExpr};

pub struct Parser;
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
        let mut parser = Parser::new();

        /*
        Todo: Doesn't work without modifications if the source code is directly read from a while,
        but because we're doing this through console ("line by line") it's okay for now
        */
        let mut iter = tokens.iter().peekable();
        let peeked = iter.peek().expect("nothing to do");
        if let LexerToken::Keyword(kw) = peeked {
            // variable decleration syntax
            if kw == "let" {
                // 'let' keyword
                iter.next(); 
                
                // identifier
                let tk_identifier = iter.next().expect("nothing after keyword 'let'");
                let identifier: String;
                match tk_identifier {
                    LexerToken::Identifier(id) => { identifier = id.clone(); }
                    _ => {
                        panic!("Not identifier after 'let'");
                    }
                }

                // symbol '='
                let symbol = iter.next().expect("nothing after keyword 'let'");
                if let LexerToken::Symbol(s) = symbol {
                    assert!(s.clone() == '=');
                }

                // value
                // todo: get expression
                let value = iter.next().expect("nothing after keyword 'let'");
                if let LexerToken::Value(val) = value {
                    return vec![ParserToken::Push(val.clone()), ParserToken::DeclareVariable(identifier)];
                }
                panic!("Unexpected assign!");
            }
            else {
                panic!("Non implumented keyword: \"{}\"", kw);
            }
        }
        else 
        {
            return AstExpr::evaluate(&mut tokens);
        }
    }

    fn new() -> Parser { Parser {}}
    
    
}
