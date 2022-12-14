use core::panic;
use std::collections::{VecDeque};

use crate::interpreter::{Interpreter};
use crate::lexer::{LexerTokenType, LexerToken, Lexer};
use crate::value::Value;
use crate::expr::{AstExpr};

pub struct Parser {
    input: VecDeque<LexerToken>,
    is_expr: bool,
    last_line: u16, last_column: u16
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParserToken {
    Nop, // fallback used by errors.
    DeclareVariable(String), // Pops a value from stack and stores it to stack
    StoreVariable(String),   // Pops and stores it
    MakeArray(u32),          // How many arguments to pop from the stack to create the array
    GetArrayElement(Vec<ParserToken>),
    GetVariableArrayElement(String, Vec<ParserToken>),
    StoreVariableArrayElement(String), // first pop is assignment, second is index.
    DeclareFunction(String, Vec<ParserToken>),
    GetVariable(String),     // Pushes the variables value to stack
    Operation(String),       // Pops 2 values from stack as arguments and pushes a result
    Push(Value),
    Pop(),
    If(Vec<ParserToken>, Vec<ParserToken>), // Pops value, if true executes first, else the second
    While(Vec<ParserToken>, Vec<ParserToken>), // First expression used for comparision, if true executes second (which is the body)
    Call(String, Vec<ParserToken>), // Second are arguments, executed before calling.
    CallNative(fn(*mut Interpreter)), // Added by libraries when imported
    Return(),
    Import(String),
}

impl Parser {
    #[must_use]
    pub fn parse(tokens: VecDeque<LexerToken>, is_expr: bool) -> Vec<ParserToken> {
        let mut parser = Parser::new(tokens, is_expr);
        parser.parse_until(LexerTokenType::Eof)
    }

    #[must_use]
    fn parse_until(&mut self, tk: LexerTokenType) -> Vec<ParserToken> {
        assert!(!self.is_expr);

        let mut tokens = vec![];
        let mut scopes: Vec<char> = vec![];
        'parse_loop : loop {
            let peek = self.peek();
            if peek.is_none() {
                break 'parse_loop;
            }

            // Terminator
            let token = peek.unwrap();
            if scopes.len() == 0 && &token.token == &tk {
                break 'parse_loop;
            }
            // Scopes
            if let LexerTokenType::Symbol(symbol) = &token.token {
                if symbol == &'{' {
                    scopes.push('{');
                }
                else if symbol == &'}' {
                    let symbol = scopes.pop().expect(format!("not matching '}}' for '{{' got {} instead", symbol).as_str());
                    if symbol != '{' {
                        self.error(format!("not matching '}}' for '{{' got {} instead", symbol));
                    }
                }
            }
            else if let LexerTokenType::Keyword(kw) = &token.token {
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
                    "if" => {
                        tokens.append(&mut self.if_statement())
                    }
                    "while" => {
                        tokens.append(&mut self.while_statement())
                    }
                    "import" => {
                        tokens.append(&mut self.import_keyword())
                    }
                    _ => { self.error(format!("Unimplumented keyword {}", kw)); }
                }

            }
            else if let LexerTokenType::Identifier(ident) = token.token.clone() {
                self.eat(); // Identifier
                let next = self.eat();
                let tk = next.unwrap();
                
                // Is array
                if let LexerTokenType::Symbol(symbol) = &tk.token {
                    if symbol == &'[' {
                        tokens.append(&mut self.array_assignment(ident.clone()));
                    }
                }
                else if let LexerTokenType::Operator(op) = &tk.token {
                    match op.as_str() {
                        "=" => { tokens.append(&mut self.variable_assignment(ident.clone())); }
                        "(" => { tokens.append(&mut self.function_call(ident.clone())); }
                        _ => { 
                            self.error(format!("Invalid operator! {}", op));
                        }
                    }
                }
                
            }
            else if token.token == LexerTokenType::NewLine {
                self.eat();
            }
            else 
            {
                self.error(format!("Invalid syntax found token {:?}", token));
            }
        }
        tokens
    }

    /**
     * Turns an expression like [Identifier("foo"), Operator("+"), Int(5)] to
     * [GetVariable("foo"), Operation("+"), Push(5)]
     */
    fn parse_expression(&mut self) -> Vec<ParserToken> {
        assert!(self.is_expr);

        let mut tokens = vec![];
        // This is an array
        if self.peek().is_some() && LexerTokenType::Symbol('[') == self.peek().unwrap().token {
            self.eat_expect(LexerTokenType::Symbol('['));

            let mut element_count = 0u32;
            loop {
                let mut array_element = self.eat_expr(vec![LexerTokenType::Symbol(','), LexerTokenType::Symbol(']')]);
                tokens.append(&mut array_element);
                
                let peek = self.peek();
                if peek.is_none() {
                    self.error(format!("Expected ']' near '['"));
                }

                if let LexerTokenType::Symbol(symbol) = &peek.unwrap().token {
                    if symbol == &',' {
                        self.eat_expect(LexerTokenType::Symbol(','));
                        element_count += 1;
                    }
                    else if symbol == &']' {
                        self.eat_expect(LexerTokenType::Symbol(']'));
                        break;
                    }
                    else {
                        self.error(format!("Expected ']' near '['"));
                    }
                }
            }

            // element_count is 1 off, because it counts ','s so we add 1 here, if there was atleast 1 argument.
            element_count += if tokens.len() > 0 { 1 } else { 0 };
            tokens.push(ParserToken::MakeArray(element_count));
            return tokens;
        }

        // Other expression
        'parse_loop : loop {
            let peek = self.peek();
            if peek.is_none() {
                break 'parse_loop;
            }
            let token = self.eat().unwrap().token;
            
            match token {
                LexerTokenType::Identifier(ident) => {
                    let next = self.eat();
                    if next.is_none() {
                        tokens.push(ParserToken::GetVariable(ident));
                        break;
                    }

                    // Still need to determine between: "identifier, fncall(args), array[0]""
                    let next_token = next.unwrap().token;
                    match next_token {
                        LexerTokenType::Operator(op) => {
                            match op.as_str() {
                                "(" => { 
                                    tokens.append(&mut self.function_call(ident)); 
                                }
                                _ => { 
                                    tokens.push(ParserToken::GetVariable(ident));
                                    tokens.push(ParserToken::Operation(op));
                                }
                            }
                        }
                        // Indexing into array
                        LexerTokenType::Symbol(symbol) => {
                            if symbol != '[' {
                                self.error(format!("Unexpected '['"));
                            }
                            let argument = self.eat_expr(vec![LexerTokenType::Symbol(']')]);
                            self.eat_expect(LexerTokenType::Symbol(']'));
                            tokens.push(ParserToken::GetVariableArrayElement(ident, argument));
                        }
                        _ => { 
                            self.error(format!("Unexpected {:?} after expression!", next_token));
                        }
                    }
                }
                LexerTokenType::Operator(op) => {
                    tokens.push(ParserToken::Operation(op));
                },
                LexerTokenType::Value(val) => {
                    tokens.push(ParserToken::Push(val));
                } 
                _ => { self.error(format!("Invalid syntax {:?}", token)); }
            }
        }
        tokens
    }

    #[must_use]
    fn import_keyword(&mut self) -> Vec<ParserToken> {
        // Syntax "<keyword->import> <literal><semicolon>"
        self.eat_expect(LexerTokenType::Keyword("import".to_string()));
        
        let library = self.eat();
        if library.is_some() {
            if let LexerTokenType::Value(library_val) = library.unwrap().token {
                if let Value::Literal(library_name) = library_val {
                    self.eat_expect(LexerTokenType::Symbol(';'));
                    return vec![ParserToken::Import(library_name.clone())];
                }
            }
        }
        self.error("Expected Library as String after \"import\" got EOF instead!".to_string());
    }

    #[must_use]
    fn if_statement(&mut self) -> Vec<ParserToken> {
        self.eat_expect(LexerTokenType::Keyword("if".to_string()));

        // If comparision 
        let mut expr = self.eat_expr(vec![LexerTokenType::Symbol('{')]);
        if expr.len() == 0 {
            self.error("Expecte an comparision after 'if' statement!".to_string());
        }
        
        // If(true) body
        self.eat_expect(LexerTokenType::Symbol('{'));
        let if_body = self.parse_until(LexerTokenType::Symbol('}'));
        self.eat_expect(LexerTokenType::Symbol('}'));

        // else body, if followed by an else statement
        let mut else_body = vec![];
        let peek = self.peek();
        if let Some(tk) = peek {
            if let LexerTokenType::Keyword(kw) = &tk.token {
                if kw == "else" {
                    self.eat_expect(LexerTokenType::Keyword("else".to_string()));

                    let next = self.peek().expect("Expected '{' or 'if' efter keyword else");
                    if LexerTokenType::Keyword("if".to_string()) == next.token {
                        // "else if" body
                        else_body = self.if_statement();
                    }
                    else {
                        // else body
                        self.eat_expect(LexerTokenType::Symbol('{'));
                        else_body = self.parse_until(LexerTokenType::Symbol('}'));
                        self.eat_expect(LexerTokenType::Symbol('}'));
                    }

                }
            }
        }

        // Tokens
        let mut tokens = vec![];
        tokens.append(&mut expr);
        tokens.push(
            ParserToken::If(if_body, else_body)
        );
        tokens
    }

    #[must_use]
    fn while_statement(&mut self) -> Vec<ParserToken> {
        self.eat_expect(LexerTokenType::Keyword("while".to_string()));

        // While comparision 
        let expr = self.eat_expr(vec![LexerTokenType::Symbol('{')]);
        if expr.len() == 0 {
            panic!("Expecte an expression after 'while' statement!");
        }
        
        // If(true) body
        self.eat_expect(LexerTokenType::Symbol('{'));
        let if_body = self.parse_until(LexerTokenType::Symbol('}'));
        self.eat_expect(LexerTokenType::Symbol('}'));

        // Tokens
        let mut tokens = vec![];
        tokens.push(
            ParserToken::While(expr, if_body)
        );
        tokens
    }

    #[must_use]
    fn function_return(&mut self) -> Vec<ParserToken> {
        self.eat_expect(LexerTokenType::Keyword("return".to_string()));
        let expr = self.eat_expr(vec![LexerTokenType::Symbol(';')]);
        self.eat_expect(LexerTokenType::Symbol(';'));

        let mut tokens;
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
    fn array_assignment(&mut self, var_name: String) -> Vec<ParserToken> {
        // Get Index
        let idx_expr = self.eat_expr(vec![LexerTokenType::Symbol(']')]);
        self.eat_expect(LexerTokenType::Symbol(']'));

        // Get Assigment
        self.eat_expect(LexerTokenType::Operator("=".to_string()));
        let mut assign_expr = self.eat_expr(vec![LexerTokenType::Symbol(';')]);
        self.eat_expect(LexerTokenType::Symbol(';'));

        let mut tokens = idx_expr;
        tokens.append(&mut assign_expr);
        tokens.push(ParserToken::StoreVariableArrayElement(var_name));
        tokens
    }

    #[must_use]
    fn variable_assignment(&mut self, var_name: String) -> Vec<ParserToken> {
        let mut tokens = self.eat_expr(vec![LexerTokenType::Symbol(';')]);
        self.eat_expect(LexerTokenType::Symbol(';'));
        tokens.push(ParserToken::StoreVariable(var_name));
        tokens
    }

    #[must_use]
    fn variable_decleration(&mut self) -> Vec<ParserToken> {
        // eat "let" keyword
        self.eat();
    
        // identifier
        let tk_identifier = self.eat().expect("expected an identifier after 'let' keyword").token;
        if let LexerTokenType::Identifier(identifier) = tk_identifier {
            // Syntax
            self.eat_expect(LexerTokenType::Operator("=".to_string()));
            let mut expr = self.eat_expr(vec![LexerTokenType::Symbol(';')]);
            self.eat_expect(LexerTokenType::Symbol(';'));
            
            // Tokens        
            if expr.len() == 0 {
                self.error("Expected an expression after '=', before ';'".to_string());
            }
            let mut tokens = vec![];
            tokens.append(&mut expr);
            tokens.push(ParserToken::DeclareVariable(identifier.clone()));
    
            return tokens;
        }
        self.error("expected an identifier after 'let' keyword".to_string());
    }

    #[must_use]
    fn function_decleration(&mut self) -> Vec<ParserToken> {
        // eat "fn" keyword
        self.eat();

        // identifier
        let tk_identifier = self.eat().expect("expected an identifier after 'fn' keyword").token;
        if let LexerTokenType::Identifier(fn_name) = tk_identifier {
            // eat operator '('
            self.eat_expect(LexerTokenType::Operator("(".to_string()));

            // get argument names
            let mut fn_tokens: Vec<ParserToken> = vec![];
            'args : loop {
                let tk = self.eat().expect("Invalid function decleration").token;

                if let LexerTokenType::Identifier(arg_identifier) = tk {
                    // When calling the function the values are pushed to the stack, here just use them to declare 
                    // variables out of them (btw this has to be done in a reverse order, hence the reverse after the loop)
                    fn_tokens.push(ParserToken::DeclareVariable(arg_identifier));

                    let next  = self.eat().expect("Invalid function decleration").token;
                    if next == LexerTokenType::Symbol(',') {
                        continue;
                    }
                    else if next == LexerTokenType::Operator(")".to_string()) {
                        break 'args;
                    }
                    self.error("Syntax error".to_string());
                }
                else if tk == LexerTokenType::Operator(")".to_string()) {
                    break 'args;
                }
                else {
                    self.error("Syntax error".to_string());
                }
            }
            fn_tokens.reverse();

            self.eat_expect(LexerTokenType::Symbol('{'));
            let mut fn_body = self.parse_until(LexerTokenType::Symbol('}'));
            self.eat_expect(LexerTokenType::Symbol('}'));

            fn_tokens.append(&mut fn_body);
            
            // push tokens
            return vec![(ParserToken::DeclareFunction(fn_name, fn_tokens))];
        }
        self.error("expected an identifier after 'fn' keyword".to_string());
    }

    #[must_use]
    fn function_call(&mut self, fn_name: String) -> Vec<ParserToken> {
        let mut arg_tokens = vec![];
        'args : loop {
            let tk = &self.peek().expect(format!("Invalid function call '{}()'", fn_name).as_str()).token;

            if tk == &LexerTokenType::Operator(")".to_string()) {
                self.eat().unwrap();
                break 'args;
            }
            else {
                let mut expr = self.eat_expr(
                    vec![
                        LexerTokenType::Symbol(','),
                        LexerTokenType::Operator(")".to_string()),
                    ]
                );
                arg_tokens.append(&mut expr);

                let next = self.eat().expect("syntax error").token;
                if next == LexerTokenType::Symbol(',') {
                    continue;
                }
                else if next == LexerTokenType::Operator(")".to_string()) {
                    break 'args;
                }
                else {
                    self.error("Syntax error".to_string());
                }
            }
        }

        if !self.is_expr {
            self.eat_expect(LexerTokenType::Symbol(';'));
        }
        
        let mut tokens = vec![];
        tokens.push(ParserToken::Call(fn_name, arg_tokens));
        return tokens;
    }

    /**
     * Terminator is used to determine when the expression is suppost to end, terminator doesn't get eaten. e.g: 
     * "LexerTokenType::Symbol(';')" for "let x = 2+2;"
     * "LexerTokenType::Symbol(',')" for "fn foo(2+2+2, 0)"
     * "LexerTokenType::Operator(')')" for "fn foo(2+2+2)" // this is going to be a fucking problem, lol.
     */
    fn eat_until(&mut self, terminator: Vec<LexerTokenType>) -> VecDeque<LexerToken> {
        let mut out_tks = VecDeque::<LexerToken>::new();

        let mut scopes: Vec<char> = vec![]; // '(' gets pushed in, and ')' pushes them out.
        'get_tokens: loop {
            let peeked = self.peek();
            if peeked.is_none() {
                if terminator.contains(&LexerTokenType::Eof) {
                    break 'get_tokens;
                }
                self.error(format!("Expected '{:?}' got EOF instead!", terminator));
            }

            // Don't eat before this, we don't want to eat the terminator.
            if scopes.len() == 0 && terminator.contains(&peeked.unwrap().token) {
                break 'get_tokens;
            }

            let token = self.eat().unwrap();
            if let LexerTokenType::Operator(op) = &token.token {
                match op.as_str() {
                    "(" => {
                        scopes.push('(');
                    },
                    ")" => {
                        let popped = scopes.pop().expect("No matching ']' for '['");
                        if popped != '(' {
                            self.error(format!("Expected ')' for '(', but got {} instead!", popped));
                        }
                    }
                    _ => {}
                }
            }

            // Arrays
            if let LexerTokenType::Symbol(s) = &token.token {
                match s.clone() {
                    '[' => {
                        scopes.push(s.clone());
                    },
                    ']' => {
                        let popped = scopes.pop().expect("No matching ']' for '['");
                        if popped != '[' {
                            self.error(format!("Expected ']' for '[', but got {} instead!", popped));
                        }
                    }
                    _ => {}
                }
            }
            
            out_tks.push_back(token);
        }
        return out_tks;
    }

    /**
     * Also evaluated
     */
    fn eat_expr(&mut self, terminator: Vec<LexerTokenType>) -> Vec<ParserToken> {
        let expr = self.eat_until(terminator);
        let mut parse = Parser::new(expr, true);
        let mut parsed = parse.parse_expression();
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
            self.error("Got unexpected EOF".to_string());
        }
        popped.unwrap()
    }

    fn eat_expect(&mut self, expect: LexerTokenType) -> LexerToken {
        let popped = self.eat();
        if popped.is_none() {
            self.error(format!("Expected {:?} got EOF instead!", expect));
        }
        let tk = popped.unwrap();
        if tk.token != expect {
            self.error(format!("Expected {:?} got {:?} instead! :(", expect, tk.token));
        }
        tk
    }

    fn eat(&mut self) -> Option<LexerToken> {
        let popped = self.input.pop_front();
        if let Some(tk) = popped {
            self.last_line = tk.line;
            self.last_column = tk.column;
            return Some(tk);
        }
        popped
    }

    fn error(&self, msg: String) -> ! {
        use colored::Colorize;
        // TODO: Filename
        println!("{}", format!("An error occurred while parsing at {}:{} '{}'", self.last_line, self.last_column, msg).red());
        panic!("");
    }

    fn new(tks: VecDeque<LexerToken>, is_expr: bool) -> Parser { 
        Parser {
            input: tks,
            is_expr: is_expr,
            last_column: 0,
            last_line: 0
        }
    }
}
