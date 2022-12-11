use crate::parser::{Parser, ParserToken};
use crate::interpreter::{Interpreter};

pub fn import_libs(interpreter: &mut Interpreter) {
    interpreter.declare_function(&"print".to_string(), &vec![
        ParserToken::CallNative(print)
    ]);
}

fn print(interpreter: *mut Interpreter) {
    let val = unsafe { interpreter.as_mut().unwrap().pop() };
    println!("{}", val.to_string());
}
