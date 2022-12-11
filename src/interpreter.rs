use std::collections::{HashMap, VecDeque};
use std::process::exit;

use crate::parser::{ParserToken};
use crate::value::{Value};

mod math;
mod io;

#[derive(Debug, Clone, PartialEq)]
struct Scope {
    scope_name: String,
    variables: HashMap<String, Value>
}
impl Scope {
    pub fn new(scope_name: String) -> Scope {
        Scope {
            scope_name: scope_name,
            variables: HashMap::new()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Interpreter {
    funcs: HashMap<String, Vec<ParserToken>>,
    variables: VecDeque<Scope>,
    stack: Vec<Value>,
    last_op: *const ParserToken
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut int = Interpreter { 
            funcs: HashMap::new(),
            variables: VecDeque::new(),
            stack: vec![],
            last_op: 0 as *const ParserToken
        };
        int.start_scope("global".to_string());
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
            else if let ParserToken::Call(func_name, arg_tokens) = token {
                self.call_function(func_name, arg_tokens);
            }
            else if let ParserToken::CallNative(native_function) = token {
                native_function(self);
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
                break;
            }
            else if let ParserToken::StoreVariable(var_name) = &token {
                self.store_variable(var_name);
            }
            else if let ParserToken::If(true_body, false_body) = &token {
                self.if_statement(true_body, false_body);
            }
            else if let ParserToken::While(check, body) = &token {
                self.while_loop(check, body);
            }
            else if let ParserToken::Import(library) = &token {
                self.import(library);
            }
            else {
                self.error(panic!("Unimplumented operation: {:?}", token));
            }

            self.last_op = token;
        }
    }

    fn import(&mut self, library: &String) {
        if library == "math" {
            math::import_libs(self);
            return;
        }
        if library == "io" {
            io::import_libs(self);
            return;
        }
    }

    fn while_loop(&mut self, check: &Vec<ParserToken>, body: &Vec<ParserToken>) {
        'while_loop : loop{
            // Evalute
            self.execute_tokens(check);
            let continue_looping = self.pop().is_true();
            if !continue_looping {
                break 'while_loop;
            }

            // Execute the body
            self.start_scope("While loop".to_string());
            self.execute_tokens(body);
            self.end_scope();
        }
    }

    fn if_statement(&mut self, true_body: &Vec<ParserToken>, false_body: &Vec<ParserToken>) {
        let value = self.pop();
        if value.is_true() {
            self.start_scope("If block".to_string());
            self.execute_tokens(true_body);
            self.end_scope();
        }
        else {
            self.start_scope("Else block".to_string());
            self.execute_tokens(false_body);
            self.end_scope();
        }
    }

    fn call_function(&mut self, fn_name: &String, arg_tokens: &Vec<ParserToken>) {
        self.execute_tokens(arg_tokens);

        if fn_name == "panic" {
            self.error("PANIC".to_string());
        }

        self.start_scope(fn_name.clone());
        let tks = self.funcs.get(fn_name).expect("No function found!").clone();
        self.execute_tokens(&tks);
        self.end_scope();
    }

    fn store_variable(&mut self, var_name: &String) {
        let val = self.pop();

        for idx in 0..self.variables.len() {
            let scope = self.variables.get_mut(idx).unwrap();
            let exists = scope.variables.contains_key(var_name);
            if !exists {
                continue;
            }
            *scope.variables.get_mut(var_name).unwrap() = val;
            return;
        }
        panic!("No variable called {}", var_name);
    }

    fn declare_variable(&mut self, var_name: &String) {
        assert!(self.stack.len() > 0);

        let val = self.pop();
        let scope = self.get_scope();
        if scope.variables.contains_key(var_name) {
            panic!("A variable named {} already exsts!", var_name);
        }
        scope.variables.insert(var_name.clone(), val);
    }

    fn declare_function(&mut self, fn_name: &String, fn_body: &Vec<ParserToken>) {
        if self.get_scope_count() > 1 {
            panic!("Function declerations only allowed in the global scope!");
        }

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
            let scope = self.variables.get(idx).unwrap();
            let val = scope.variables.get(var_name);
            if val.is_none() {
                continue;
            }
            self.push(val.unwrap().clone());
            return;
        }
        self.error(format!("No variable called {}", var_name));
    }

    #[must_use]
    pub fn get_variable_value(&mut self, var_name: &String) -> Value {
        self.get_variable(var_name);
        return self.pop();
    }

    fn get_scope_count(&self) -> usize {
        self.variables.len()
    }

    fn get_scope(&mut self) -> &mut Scope {
        self.variables.front_mut().unwrap()
    }

    fn start_scope(&mut self, scope_name: String) {
        self.variables.push_front(
            Scope::new(scope_name)
        );
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

    pub fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> Value {
        if self.stack.len() == 0 {
            self.error(format!("not enough arguments to pop"));
        }
        return self.stack.pop().unwrap();
    }

    fn error(&mut self, error_msg: String) -> ! {
        use colored::Colorize;

        println!("{}", format!("-------VALUE STACK [{}]:------", self.stack.len()).red().bold());

        let mut stack_copy = self.stack.clone();
        stack_copy.reverse();

        for idx in (0..stack_copy.len()).rev() {
            let val = stack_copy.get(idx).unwrap();
            println!("[{}] = {}",
                idx, 
                format!("{:?}", val).green()
            );

        }

        println!("{}", format!("-------VARIABLES:------------").red().bold());
        for scope_idx in (0..self.variables.len()).rev() {
            let identation = self.variables.len()-scope_idx;
            let scope_name = &self.variables[scope_idx].scope_name;
            for _ in 0..identation {
                print!(" ");
            }

            println!("{}:", scope_name);
            for (var_name, value) in &self.variables[scope_idx].variables {
                for _ in 0..identation {
                    print!(" ");
                }
                println!(" {} = {:?}", var_name, value);
            }
        }

        println!("{}", format!("-----------------------------").red().bold());
        let op = unsafe { self.last_op.as_ref() };
        println!("{}", format!("Last operation: {:?}", op.unwrap_or(&ParserToken::Push(Value::Null))).bold().red());
        println!("{}", format!("Interpreter Error: '{}'", error_msg.bold()).red());
        println!("{}", format!("-----------------------------").red().bold());

        exit(-1);
    }

}

