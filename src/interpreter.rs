use std::collections::{HashMap, VecDeque};
use std::process::exit;

use crate::parser::{ParserToken};
use crate::value::{Value};

type Variables = HashMap<String, Value>;
pub struct Interpreter {
    funcs: HashMap<String, Vec<ParserToken>>,
    variables: VecDeque<Variables>,
    stack: Vec<Value>
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut int = Interpreter { 
            funcs: HashMap::new(),
            variables: VecDeque::new(),
            stack: vec![]
        };
        // Global Variable Scope
        int.start_scope();
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
            else if let ParserToken::Call(func_name) = token {
                self.call_function(func_name)
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
            else if let ParserToken::Return() = &token {
                self.return_function();
                break;
            }
        }
    }

    fn call_function(&mut self, fn_name: &String) {
        self.start_scope();
        
        if fn_name == "print" {
            println!("| PRINT: {:?} |", self.pop());
            return;
        }
        let tks = self.funcs.get(fn_name).expect("No function found!").clone();
        self.execute_tokens(&tks);
        self.end_scope();
    }

    fn return_function(&mut self) {
        // TODO: return a value to the stack.
        // self.end_scope();
    }

    fn declare_variable(&mut self, var_name: &String) {
        assert!(self.stack.len() > 0);

        let val = self.pop();
        let scope = self.get_scope();
        if scope.contains_key(var_name) {
            panic!("A variable named {} already exsts!", var_name);
        }
        scope.insert(var_name.clone(), val.clone());
    }

    fn declare_function(&mut self, fn_name: &String, fn_body: &Vec<ParserToken>) {
        if self.funcs.contains_key(fn_name) {
            panic!("A function named {} already exsts!", fn_name);
        }
        self.funcs.insert(fn_name.clone(), fn_body.clone());
    }

    /**
     * Gets pushed onto stack
     */
    fn get_variable(&mut self, var_name: &String) {
        for idx in 0..self.variables.len() {
            let variables = self.variables.get(idx).unwrap();
            let val = variables.get(var_name);
            if val.is_none() {
                continue;
            }
            self.push(val.unwrap().clone());
            return;
        }
        panic!("No variable called {}", var_name);
    }

    fn get_scope(&mut self) -> &mut Variables {
        self.variables.front_mut().unwrap()
    }

    fn start_scope(&mut self) {
        self.variables.push_front(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.variables.pop_front();
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
    }

    fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> Value {
        assert!(self.stack.len() >= 1, "not enough arguments to pop");
        return self.stack.pop().unwrap();
    }


}

