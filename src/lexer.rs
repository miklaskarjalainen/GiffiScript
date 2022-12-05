use crate::value::Value;

const SYMBOLS: [char; 7] = ['{', '}', '=', '.', ',', ':', ';'];
const OPERATORS: [&'static str; 6] = ["+", "-", "/", "*", "(", ")"]; // Not chars cuz need to add operators like "&&", "||"
const KEYWORDS: [&'static str; 1] = ["let"];

#[derive(Debug, Clone)]
pub enum LexerToken {
    Keyword(String),
    Value(Value),
    Symbol(char),
    Operator(String),
    Identifier(String)
}

#[derive(Debug)]
pub struct Lexer {
    current_word: String,
    lexer_tokens: Vec<LexerToken>
}

impl Lexer {
    pub fn lex(code: String) -> Result<Vec<LexerToken>, String> {
        let mut lexer = Lexer::new();


        for c in code.chars() {
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

        Ok(lexer.lexer_tokens)
    }

    fn new() -> Lexer {
        Lexer{
            current_word: String::from(""),
            lexer_tokens: vec![]
        }
    }

    fn flush(&mut self) {
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
        self.lexer_tokens.push(tk);
    }

}
