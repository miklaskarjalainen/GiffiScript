use crate::value::Value;
use std::collections::{VecDeque};

const SYMBOLS: [char; 6] = ['{', '}', '.', ',', ':', ';'];
const OPERATORS: [&'static str; 12] = ["+", "-", "/", "*", "%", "(", ")", "=", "!", "==", "!=", "||"]; // Not chars cuz need to add operators like "&&", "||"
const KEYWORDS: [&'static str; 6] = ["let", "return", "fn", "if", "else", "while"];

#[derive(Debug, Clone, PartialEq)]
pub enum LexerToken {
    Keyword(String),
    Value(Value),
    Symbol(char),
    Operator(String),
    Identifier(String),
    NewLine,
    Eof
}

#[derive(Debug)]
pub struct Lexer {
    is_literal: bool,
    current_word: String,
    lexer_tokens: VecDeque<LexerToken>
}

impl Lexer {
    pub fn lex(code: String) -> VecDeque<LexerToken> {
        let mut lexer = Lexer::new();

        let mut iter = code.chars().peekable();
        loop {
            let opt_c = iter.next();
            if opt_c.is_none() {
                break;
            }
            let c = opt_c.unwrap();
            
            // Strings
            if c == '"' {
                lexer.flush();
                lexer.is_literal = !lexer.is_literal;
                continue;
            }
            if lexer.is_literal {
                lexer.current_word.push(c);
                continue;
            }
            
            if c.is_whitespace() {
                lexer.flush();
                continue;
            }
            if SYMBOLS.contains(&c)
            {
                lexer.flush();
                lexer.push_token(LexerToken::Symbol(c));
                continue;
            }
            if OPERATORS.contains(&String::from(c).as_str())
            {
                lexer.flush();
                
                lexer.current_word.push(c);
                let peek = iter.peek();
                if peek.is_none() {
                    lexer.flush();
                    break;
                }
                else {
                    let c = peek.unwrap();
                    lexer.current_word.push(c.clone());
                    if OPERATORS.contains(&lexer.current_word.as_str()) {
                        iter.next();
                    }
                    else {
                        lexer.current_word.pop();
                    }
                    lexer.flush();
                    continue;
                }
            }

            lexer.current_word.push(c);
        }
        lexer.flush();

        if lexer.is_literal {
            panic!("String literal is missing a '\"'");
        }
        
        lexer.lexer_tokens.push_back(LexerToken::Eof);
        lexer.lexer_tokens
    }

    fn new() -> Lexer {
        Lexer{
            is_literal: false,
            current_word: String::from(""),
            lexer_tokens: VecDeque::new()
        }
    }

    fn flush(&mut self) {
        // Literals can be empty (just 2 quotes)!
        if self.is_literal {
            self.push_token(LexerToken::Value(Value::Literal(self.current_word.clone())));
            self.current_word.clear();
            return;
        }
        if self.current_word.is_empty() {
            return;
        }

        let word = self.current_word.clone();
        self.current_word.clear();

        
        if let Ok(v) = Value::parse(&word) {
            self.push_token(LexerToken::Value(v));
        }
        else if OPERATORS.contains(&word.as_str()) { 
            self.push_token(LexerToken::Operator(word.clone()));
        }
        else if KEYWORDS.contains(&word.as_str()) {
            self.push_token(LexerToken::Keyword(word.clone()));
        }
        else {
            self.push_token(LexerToken::Identifier(word));
        }
    }

    fn push_token(&mut self, tk: LexerToken) {
        self.lexer_tokens.push_back(tk);
    }

}
