use crate::value::Value;
use std::collections::{VecDeque};

const SYMBOLS: [char; 6] = ['{', '}', '.', ',', ':', ';'];
const OPERATORS: [&'static str; 7] = ["+", "-", "/", "*", "(", ")", "="]; // Not chars cuz need to add operators like "&&", "||"
const KEYWORDS: [&'static str; 3] = ["let", "return", "fn"];

#[derive(Debug, Clone, PartialEq)]
pub enum LexerToken {
    Keyword(String),
    Value(Value),
    Symbol(char),
    Operator(String),
    Identifier(String),
    Eof
}

#[derive(Debug)]
pub struct Lexer {
    is_litreal: bool,
    current_word: String,
    lexer_tokens: VecDeque<LexerToken>
}

impl Lexer {
    pub fn lex(code: String) -> VecDeque<LexerToken> {
        let mut lexer = Lexer::new();

        for c in code.chars() {
            // Strings
            if c == '"' {
                lexer.flush();
                lexer.is_litreal = !lexer.is_litreal;
                continue;
            }
            if lexer.is_litreal {
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
                lexer.flush();
                continue;
            }

            lexer.current_word.push(c);
        }
        lexer.flush();

        if lexer.is_litreal {
            panic!("String literal is missing a '\"'");
        }
        
        lexer.lexer_tokens.push_back(LexerToken::Eof);
        lexer.lexer_tokens
    }

    fn new() -> Lexer {
        Lexer{
            is_litreal: false,
            current_word: String::from(""),
            lexer_tokens: VecDeque::new()
        }
    }

    fn flush(&mut self) {
        if self.current_word.is_empty() {
            return;
        }

        let word = self.current_word.clone();
        self.current_word.clear();

        if self.is_litreal {
            self.push_token(LexerToken::Value(Value::Litreal(word)));
        }
        else if let Ok(v) = Value::parse(&word) {
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
