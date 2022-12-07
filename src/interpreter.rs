use std::collections::{HashMap};
use std::process::exit;

use crate::parser::{ParserToken};
use crate::value::{Value};

pub struct Interpreter {
    funcs: HashMap<String, Vec<ParserToken>>,
    variables: HashMap<String, Value>,
    stack: Vec<Value>
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut int = Interpreter { 
            funcs: HashMap::new(),
            variables: HashMap::new(),
            stack: vec![]
        };

        int.funcs.insert("test".to_string(), vec![
            ParserToken::Push(Value::Int(5)),
            ParserToken::Push(Value::Int(6)),
            ParserToken::Operation("*".to_string())
        ]);
        return int;
    }

    pub fn execute_tokens(&mut self, tokens: &Vec<ParserToken>) {
        for token in tokens {
            if let ParserToken::Push(v) = token {
                self.push(v.clone());
            }
            else if let ParserToken::Pop() = token {
                self.pop();
            }
            else if let ParserToken::Call(func_name, _arg_count) = token {
                println!("Calling function {}", func_name);
                assert!(self.funcs.contains_key(func_name), "No function with this name was found :-(");
                let tks = self.funcs[func_name].clone();
                self.execute_tokens(&tks);
            }
            else if let ParserToken::GetVariable(var_name) = token {
                self.get_variable(var_name);
            }
            else if let ParserToken::DeclareVariable(var_name) = token {
                self.declare_variable(var_name);
            }
            else if let ParserToken::DeclareFunction(fn_name, fn_body) = token {
                self.declare_function(fn_name, fn_body);
            }
            else if let ParserToken::Operation(op) = &token {
                self.op(op);
            }
        }
    }

    fn declare_variable(&mut self, var_name: &String) {
        assert!(self.stack.len() > 0);

        if self.variables.contains_key(var_name) {
            panic!("A variable named {} already exsts!", var_name);
        }
        let val = self.pop();
        self.variables.insert(var_name.clone(), val);

        println!("Variable {} declared!", var_name);
    }

    fn declare_function(&mut self, fn_name: &String, fn_body: &Vec<ParserToken>) {
        if self.funcs.contains_key(fn_name) {
            panic!("A function named {} already exsts!", fn_name);
        }
        self.funcs.insert(fn_name.clone(), fn_body.clone());
        println!("Function {} declared!", fn_name);
    }

    /**
     * Gets pushed onto stack
     */
    fn get_variable(&mut self, var_name: &String) {
        if !self.variables.contains_key(var_name) {
            panic!("No variable called {}", var_name);
        }
        let val = self.variables.get(var_name).unwrap().clone();
        self.push(val);
    }

    fn op(&mut self, op: &String) {
        assert!(self.stack.len() >= 2, "not enough arguments to do an operation");

        let lhs = self.pop();
        let rhs = self.pop();
        let r = lhs.do_operation(op, rhs);

        if r.is_err() {
            println!("Error when executing an operator {:?}", r.unwrap_err());
            exit(1);
        }
        let ur = r.unwrap();

        self.push(ur.clone());
        println!("Operation Result: {:?}", ur);
    }

    fn push(&mut self, val: Value) {
        println!("Pushed {:?}", val);
        self.stack.push(val);
    }

    fn pop(&mut self) -> Value {
        assert!(self.stack.len() >= 1, "not enough arguments to pop");
        return self.stack.pop().unwrap();
    }


}

