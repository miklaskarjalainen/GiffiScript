use core::panic;
use std::collections::HashMap;
use std::default;
use serde::{Deserialize, Serialize};

use crate::lexer::{LexerToken, Lexer};
use crate::value::Value;

pub struct Parser;
#[derive(Debug, Deserialize, Serialize)]
pub enum ParserToken {
    DeclareVariable(String), // Pops a value from stack and stores it to stack
    GetVariable(String),     // Pushes the variables value to stack
    Operation(String),       // Pops 2 values from stack as arguments and pushes a result
    Push(Value),
    Pop()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AstNode {
    pub token: LexerToken,
    pub rhs: Option<Box<AstNode>>,
    pub lhs: Option<Box<AstNode>>,
}

impl AstNode {
    pub fn new(token: LexerToken, lhs: Option<Box<AstNode>>, rhs: Option<Box<AstNode>>) -> AstNode {
        AstNode {
            token: token,
            lhs: lhs,
            rhs: rhs,
        }
    }

    pub fn to_tokens(&self) -> Vec<ParserToken> {
        let mut v:Vec<ParserToken> = vec![];

        if let Some(lhs) = &self.lhs {
            v.append(&mut lhs.to_tokens())
        }

        if let Some(rhs) = &self.rhs {
            v.append(&mut rhs.to_tokens())
        }

        let tk = self.token.clone();
        if let LexerToken::Identifier(var_name) = tk {
            v.push(ParserToken::GetVariable(var_name));
        }
        else if let LexerToken::Value(value) = tk {
            v.push(ParserToken::Push(value));
        }
        else if let LexerToken::Operator(op) = tk {
            v.push(ParserToken::Operation(op));
        }
        else {
            panic!("should not happen");
        }
        return v;
    }
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
            return parser.evaluate(&mut tokens);
        }
    }

    fn new() -> Parser { Parser {}}
    
    /**
     * Returns an evaluated vector
     * (40 + 40) * 2 -> (40, 40, '+', 2, '*')
     */
    fn evaluate(&mut self, mut expr: &mut Vec<LexerToken>) -> Vec<ParserToken> {
        let ast = Parser::evaluate_ast(&mut expr, 0).expect("error in evaluation");
        return ast.to_tokens();
    }

    fn parse_primary(input: &mut Vec<LexerToken>) -> AstNode {
        let token = input.pop().expect("failed to pop");
        if let LexerToken::Value(_) = &token {
            return AstNode::new(token, None, None);
        }
        else if let LexerToken::Identifier(_) = &token {
            return AstNode::new(token, None, None);
        }
        // TODO: do parens
        panic!("lol");
    }

    fn evaluate_ast(input: &mut Vec<LexerToken>, prec: u8) -> Option<AstNode> {
        if prec >= 2 {
            return Some(Parser::parse_primary(input));
        }

        let lhs = Parser::evaluate_ast(input, prec + 1).expect("lhs is none");
        let token_opt = input.pop();

        if let Some(token) = token_opt {
            if Parser::get_precedence(&token) == prec {
                let rhs = Parser::evaluate_ast(input, prec).expect("rhs is None"); 
                return Some(AstNode::new(token, Some(Box::new(lhs)), Some(Box::new(rhs))));
            }
            else {
                input.push(token);
            }
        } 
        
        return Some(lhs);
    }

    fn get_precedence(tk: &LexerToken) -> u8 {
        if let LexerToken::Operator(op) = tk {
            match op.as_str() {
                "+" | "-" => {
                    return 0u8;
                }
                "*" | "/" | "%" => {
                    return 1u8;
                }
                "(" | ")" => {
                    return 2u8;
                }
                _ => {
                    // assert!(false, "unkown operator");
                    return 2u8;
                }
            }
        }
        panic!("{:?} is not an Operator", tk);
    }
}
