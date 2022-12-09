use core::panic;
use std::collections::{VecDeque};

use crate::lexer::{LexerToken};
use crate::value::Value;
use crate::expr::{AstExpr};

pub struct Parser {
    input: VecDeque<LexerToken>,
    is_expr: bool
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParserToken {
    DeclareVariable(String), // Pops a value from stack and stores it to stack
    StoreVariable(String),   // Pops and stores it
    DeclareFunction(String, Vec<ParserToken>),
    GetVariable(String),     // Pushes the variables value to stack
    Operation(String),       // Pops 2 values from stack as arguments and pushes a result
    Push(Value),
    Pop(),
    Call(String, Vec<ParserToken>), // Second are arguments, executed before calling.
    Return(),
}

impl Parser {
    #[must_use]
    pub fn parse(tokens: VecDeque<LexerToken>, is_expr: bool) -> Vec<ParserToken> {
        let mut parser = Parser::new(tokens, is_expr);
        parser.parse_until(LexerToken::Eof)
    }

    #[must_use]
    fn parse_until(&mut self, tk: LexerToken) -> Vec<ParserToken> {
        assert!(!self.is_expr);

        let mut tokens = vec![];
        'parse_loop : loop {
            let peek = self.peek();
            if peek.is_none() || peek.unwrap() == &tk {
                break 'parse_loop;
            }
            let token = peek.unwrap();
            
            if let LexerToken::Keyword(kw) = token {
                match kw.as_str() {
                    "let" => { 
                        tokens.append(&mut self.variable_decleration());
                    },
                    "fn" => { 
                        tokens.append(&mut self.function_decleration());
                    },
                    "return" => { 
                        tokens.append(&mut self.function_return());
                    },
                    _ => { panic!("Unimplumented keyword {}", kw); }
                }

            }
            else if let LexerToken::Identifier(ident) = token.clone() {
                self.eat(); // Identifier
                let next = self.eat();

                if let LexerToken::Operator(op) = next.unwrap() {
                    match op.as_str() {
                        "=" => { tokens.append(&mut self.variable_assignment(ident)); }
                        "(" => { tokens.append(&mut self.function_call(ident)); }
                        _ => { 
                            panic!("Invalid operator! {}", op); 
                        }
                    }
                }
                
            }
            else if token == &LexerToken::NewLine {
                self.eat();
            }
            else 
            {
                panic!("Invalid syntax");
            }
        }
        tokens
    }

    fn parse_expression(&mut self) -> Vec<ParserToken> {
        assert!(self.is_expr);

        let mut tokens = vec![];
        'parse_loop : loop {
            let peek = self.peek();
            if peek.is_none() {
                break 'parse_loop;
            }
            let token = peek.unwrap();
            if token == &LexerToken::NewLine { self.eat(); continue; }
            
            if let LexerToken::Identifier(ident) = token.clone() {
                self.eat(); // Identifier
                let next = self.eat();
                if next.is_none() {
                    tokens.push(ParserToken::GetVariable(ident.clone()));
                    break;
                }
                if let LexerToken::Operator(op) = next.unwrap() {
                    match op.as_str() {
                        "(" => { tokens.append(&mut self.function_call(ident)); }
                        _ => { 
                            tokens.push(ParserToken::GetVariable(ident.clone()));
                            tokens.push(ParserToken::Operation(op));
                        }
                    }
                }
                
            }
            else 
            {
                match token {
                    LexerToken::Operator(op) => {
                        tokens.push(ParserToken::Operation(op.clone()));
                    },
                    LexerToken::Identifier(id) => {
                        tokens.push(ParserToken::GetVariable(id.clone()));
                    },
                    LexerToken::Value(val) => {
                        tokens.push(ParserToken::Push(val.clone()));
                    } 
                    _ => { panic!("Invalid syntax {:?}", token); }
                }
                self.eat();
            }
        }
        tokens
    }

    #[must_use]
    fn function_return(&mut self) -> Vec<ParserToken> {
        self.eat_expect(LexerToken::Keyword("return".to_string()));
        let expr = self.eat_expr(vec![LexerToken::Symbol(';')]);
        self.eat_expect(LexerToken::Symbol(';'));

        let mut tokens = vec![];
        if expr.len() > 0 {
            tokens = expr;
        }
        // Implicit "return;" -> "return null;"
        else {
            tokens = vec![ParserToken::Push(Value::Null)];
        }
        tokens.push(ParserToken::Return());
        tokens
    }

    #[must_use]
    fn variable_assignment(&mut self, var_name: String) -> Vec<ParserToken> {
        let mut tokens = self.eat_expr(vec![LexerToken::Symbol(';')]);
        self.eat_expect(LexerToken::Symbol(';'));
        tokens.push(ParserToken::StoreVariable(var_name));
        tokens
    }

    #[must_use]
    fn variable_decleration(&mut self) -> Vec<ParserToken> {
        let mut tokens = vec![];

        // eat "let" keyword
        self.eat();

        // identifier
        let tk_identifier = self.eat().expect("expected an identifier after 'let' keyword");
        if let LexerToken::Identifier(identifier) = tk_identifier {
            // symbol '='
            self.eat_expect(LexerToken::Operator("=".to_string()));
            
            // Get expression
            let mut expr = self.eat_expr(vec![LexerToken::Symbol(';')]);
            tokens.append(&mut expr);

            self.eat_expect(LexerToken::Symbol(';'));
            tokens.push(ParserToken::DeclareVariable(identifier.clone()));

            return tokens;
        }
        panic!("expected an identifier after 'let' keyword");
    }

    #[must_use]
    fn function_decleration(&mut self) -> Vec<ParserToken> {
        // eat "fn" keyword
        self.eat();

        // identifier
        let tk_identifier = self.eat().expect("expected an identifier after 'fn' keyword");
        if let LexerToken::Identifier(fn_name) = tk_identifier {
            // eat operator '('
            self.eat_expect(LexerToken::Operator("(".to_string()));

            // get argument names
            let mut fn_tokens: Vec<ParserToken> = vec![];
            'args : loop {
                let tk = self.eat().expect("Invalid function decleration");

                if let LexerToken::Identifier(arg_identifier) = tk {
                    // When calling the function the values are pushed to the stack, here just use them to declare 
                    // variables out of them (btw this has to be done in a reverse order, hence the reverse after the loop)
                    fn_tokens.push(ParserToken::DeclareVariable(arg_identifier));

                    let next  = self.eat().expect("Invalid function decleration");
                    if next == LexerToken::Symbol(',') {
                        continue;
                    }
                    else if next == LexerToken::Operator(")".to_string()) {
                        break 'args;
                    }
                    panic!("Syntax error");
                }
                else if tk == LexerToken::Operator(")".to_string()) {
                    break 'args;
                }
                else {
                    panic!("Syntax error");
                }
            }
            fn_tokens.reverse();

            self.eat_expect(LexerToken::Symbol('{'));
            let mut fn_body = self.parse_until(LexerToken::Symbol('}'));
            self.eat_expect(LexerToken::Symbol('}'));

            fn_tokens.append(&mut fn_body);
            
            // push tokens
            return vec![(ParserToken::DeclareFunction(fn_name, fn_tokens))];
        }
        panic!("expected an identifier after 'fn' keyword");
    }

    #[must_use]
    fn function_call(&mut self, fn_name: String) -> Vec<ParserToken> {
        let mut arg_tokens = vec![];
        'args : loop {
            let tk = self.peek().expect("Invalid function decleration");

            if tk == &LexerToken::Operator(")".to_string()) {
                self.eat().unwrap();
                break 'args;
            }
            else {
                println!("{}", fn_name);
                let mut expr = self.eat_expr(
                    vec![
                        LexerToken::Symbol(','),
                        LexerToken::Operator(")".to_string()),
                    ]
                );
                arg_tokens.append(&mut expr);

                let next = self.eat().expect("syntax error");
                if next == LexerToken::Symbol(',') {
                    continue;
                }
                else if next == LexerToken::Operator(")".to_string()) {
                    break 'args;
                }
                else {
                    panic!("Syntax error");
                }
            }
        }

        if !self.is_expr {
            self.eat_expect(LexerToken::Symbol(';'));
        }
        
        let mut tokens = vec![];
        tokens.push(ParserToken::Call(fn_name, arg_tokens));
        return tokens;
    }

    /**
     * Terminator is used to determine when the expression is suppost to end, terminator doesn't get eaten. e.g: 
     * "LexerToken::Symbol(';')" for "let x = 2+2;"
     * "LexerToken::Symbol(',')" for "fn foo(2+2+2, 0)"
     * "LexerToken::Operator(')')" for "fn foo(2+2+2)" // this is going to be a fucking problem, lol.
     */
    fn eat_until(&mut self, terminator: Vec<LexerToken>) -> VecDeque<LexerToken> {
        let mut out_tks = VecDeque::new();
        'get_tokens: loop {
            let peeked = self.peek();
            if peeked.is_none() {
                if terminator.contains(&LexerToken::Eof) {
                    break 'get_tokens;
                }
                panic!("Expected '{:?}' got EOF instead!", terminator);
            }
            if terminator.contains(peeked.unwrap()) {
                break 'get_tokens;
            }

            let token = self.eat().unwrap();
            out_tks.push_back(token);
        }
        return out_tks;
    }

    /**
     * Also evaluated
     */
    fn eat_expr(&mut self, terminator: Vec<LexerToken>) -> Vec<ParserToken> {
        let expr = self.eat_until(terminator);
        let mut parse = Parser::new(expr, true);
        let mut parsed = parse.parse_expression();
        println!("parsed {:?}", parsed);
        let evaluated = AstExpr::evaluate(&mut parsed);
        return evaluated;
    }

    fn peek(&self) -> Option<&LexerToken> {
        if self.input.len() == 0 {
            return None;
        }
        self.input.front()
    }

    fn eat_checked(&mut self) -> LexerToken {
        let popped = self.eat();
        if popped.is_none() {
            panic!("Got unexpected EOF");
        }
        popped.unwrap()
    }

    fn eat_expect(&mut self, expect: LexerToken) -> LexerToken {
        let popped = self.eat();
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
        println!("ate {:?}", self.input.front());
        self.input.pop_front()
    }

    fn new(tks: VecDeque<LexerToken>, is_expr: bool) -> Parser { 
        Parser {
            input: tks,
            is_expr: is_expr
        }
    }
}
