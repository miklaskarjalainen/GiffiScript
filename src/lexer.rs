use crate::value::Value;
use std::collections::{VecDeque};

const SYMBOLS: [char; 7] = ['{', '}', ',', ':', ';', '[', ']'];
const OPERATORS: [&'static str; 16] = ["+", "-", "/", "*", "%", "<", ">", "(", ")", "=", "!", "|", "==", "!=", "||" , "&&"];
const KEYWORDS: [&'static str; 9] = ["let", "return", "fn", "if", "else", "while", "import", "break", "continue"];

#[derive(Debug, Clone, PartialEq)]
pub enum LexerTokenType {
    Keyword(String),
    Value(Value),
    Symbol(char),
    Operator(String),
    Identifier(String),
    NewLine,
    Eof
}

#[derive(Debug, Clone)]
pub struct LexerToken {
    pub token: LexerTokenType,
    pub line: u16, pub column: u16
}

#[derive(PartialEq)]
enum CommentType {
    None,
    Line,
    MultiLine
}

#[derive(Debug)]
pub struct Lexer {
    is_literal: bool,
    current_word: String,
    lexer_tokens: VecDeque<LexerToken>,
    line: u16, column: u16,
}

impl Lexer {
    pub fn lex(code: String) -> VecDeque<LexerToken> {
        let mut lexer = Lexer::new();

        let mut is_commented = CommentType::None;
        let mut iter = code.chars().peekable();

        loop {
            let opt_c = iter.next();
            if opt_c.is_none() {
                break;
            }
            let c = opt_c.unwrap();

            // Counter
            lexer.column += 1;
            if c == '\n' {
                lexer.line += 1;
                lexer.column = 1;
            }

            // Comments
            if is_commented == CommentType::Line {
                if c == '\n' {
                    is_commented = CommentType::None;
                }
                continue;
            }
            else if is_commented == CommentType::MultiLine {
                if c != '*' {
                    continue;
                }
                let peeked_c = iter.peek();
                if peeked_c.is_none() {
                    break;
                }
                let next_c = peeked_c.unwrap();
                if next_c == &'/' {
                    is_commented = CommentType::None;
                    iter.next().unwrap();
                }
                continue;
            }

            if c == '/' {
                let peeked_c = iter.peek();
                if peeked_c.is_none() {
                    break;
                }
                let next_c = peeked_c.unwrap();

                if &'/' == next_c {
                    lexer.flush();
                    is_commented = CommentType::Line;
                    iter.next().unwrap();
                    continue;
                }
                else if &'*' == next_c {
                    lexer.flush();
                    is_commented = CommentType::MultiLine;
                    iter.next().unwrap();
                    continue;
                }
            }
            
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
                lexer.push_token(LexerTokenType::Symbol(c));
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
                    // Negative numbers
                    let peeked_c = peek.unwrap();
                    if c == '-' && peeked_c.is_numeric() {
                        continue;
                    }

                    // 2 char operators like "==", "&&"
                    let possible_op = format!("{}{}", c, peeked_c);
                    if OPERATORS.contains(&possible_op.as_str()) {
                        iter.next();
                        lexer.current_word = possible_op;
                        lexer.flush();
                    }
                    else {
                        lexer.flush();
                    }

                    continue;
                }
            }

            lexer.current_word.push(c);
        }
        lexer.flush();

        if lexer.is_literal {
            panic!("String literal is missing a '\"'");
        }
        
        lexer.push_token(LexerTokenType::Eof);
        lexer.lexer_tokens
    }

    fn new() -> Lexer {
        Lexer{
            is_literal: false,
            current_word: String::from(""),
            lexer_tokens: VecDeque::new(),
            line: 1, column: 1
        }
    }

    fn flush(&mut self) {
        // Literals can be empty (just 2 quotes)!
        if self.is_literal {
            self.push_token(LexerTokenType::Value(Value::Literal(self.current_word.clone())));
            self.current_word.clear();
            return;
        }
        if self.current_word.is_empty() {
            return;
        }

        let word = self.current_word.clone();
        self.current_word.clear();

        
        if let Ok(v) = Value::parse(&word) {
            self.push_token(LexerTokenType::Value(v));
        }
        else if OPERATORS.contains(&word.as_str()) { 
            self.push_token(LexerTokenType::Operator(word.clone()));
        }
        else if KEYWORDS.contains(&word.as_str()) {
            self.push_token(LexerTokenType::Keyword(word.clone()));
        }
        else {
            self.push_token(LexerTokenType::Identifier(word));
        }
    }

    fn push_token(&mut self, tk: LexerTokenType) {
        self.lexer_tokens.push_back(
            LexerToken { token: tk, line: self.line, column: self.column }
        );
    }

}
