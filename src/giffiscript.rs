use crate::lexer::{Lexer};
use crate::parser::{Parser};
use crate::interpreter::{Interpreter};

pub struct GiffiScript {
    interpreter: Interpreter
}

impl GiffiScript {
    pub fn new() -> GiffiScript{
        GiffiScript {
            interpreter: Interpreter::new()
        }
    }

    pub fn execute(&mut self, code: String) {
        use colored::Colorize;
        use std::time::Instant;

        println!("{}", format!("<---Lexing Started!--->").green().bold());
            let now = Instant::now();
            let ltokens = Lexer::lex(code);
            let end = Instant::now();
            let lexer_time = end - now;
            println!("Lexer Result: {:#?}", ltokens);
        println!("{}", format!(">---Lexing Ended!---<").green().bold());

        println!("{}", format!("<---Parsing Started!--->").cyan().bold());
            let now = Instant::now();
            let ptokens = Parser::parse(ltokens, false);
            let end = Instant::now();
            let parser_time = end - now;
        println!("Parser Result: {:#?}", ptokens);
        println!("{}", format!(">---Parsing Ended!---<").cyan().bold());
        
        let now = Instant::now();
        self.interpreter.execute_tokens(&ptokens);
        let end = Instant::now();
        let interpreting_time = end - now;

        println!("Lexing Time: {:?}", lexer_time);
        println!("Parsing Time: {:?}", parser_time);
        println!("Interpriting Time: {:?}", interpreting_time);


    }
}


#[cfg(test)]
mod test {
    use crate::giffiscript::{GiffiScript};
    use crate::value::{Value};

    /**
     * Variable checked is 'r'
     */
    fn test_code(code: String, expected: Value) {
        let mut m = GiffiScript::new();
        m.execute(code);
        assert_eq!(m.interpreter.get_variable_value(&"r".to_string()), expected);
    }

    #[test]
    fn test_fn_returns_null() {
        let code = String::from("
        fn first() {
            return;
        }
        fn second() {
            return first();
        }
        let r = second();
        ");
        test_code(code, Value::Null);

        let code = String::from("
        fn first() {
            return null;
        }
        fn second() {
            return first();
        }
        let r = second();
        ");
        test_code(code, Value::Null);
    }

    #[test]
    fn test_fn_return_as_arg() {
        let code = String::from("
        fn first() {
            return \"Hello, World!\";
        }
        fn second(arg) {
            return arg;
        }
        let r = second(first());
        ");
        test_code(code, Value::Literal("Hello, World!".to_string()));
    }

    #[test]
    fn test_fn_returns1() {
        let code = String::from("
        fn returns_a_value() {
            return \"Hello, World\";
        }
        let r = returns_a_value();
        ");
        test_code(code, Value::Literal(String::from("Hello, World")));
    }

    #[test]
    fn test_fn_returns2() {
        let code = String::from("
        fn sum(arg1, arg2) {
            return arg1 + arg2;
        }
        let r = sum(5,8);
        ");
        test_code(code, Value::Int(13));

        let code = String::from("
        fn sum(arg1, arg2) {
            let result = arg1 + arg2;
            return result;
        }
        let r = sum(5,8);
        ");
        test_code(code, Value::Int(13));
    }

    #[test]
    fn test_fn_returns3() {
        let code = String::from("
        fn first_func(arg) {
            return arg + 5;
        }
        fn returns_a_value(arg2) {
            return first_func(arg2) + first_func(10);
        }
        let r = returns_a_value(25);
        ");
        test_code(code, Value::Int(45));
    }

    #[test]
    fn test_fn_returns4() {
        let code = String::from("
        let g = 0;
        fn does_a_thing() {
            return g + 1;
        }
        g = does_a_thing();
        g = does_a_thing();
        g = does_a_thing();
        g = does_a_thing();
        g = does_a_thing();
        let r = does_a_thing();
        ");
        test_code(code, Value::Int(6));
    }

}
