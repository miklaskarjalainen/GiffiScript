use crate::lexer::{Lexer};
use crate::parser::{Parser, ParserToken};
use crate::interpreter::{Interpreter};
use crate::value::ValueAdder;

pub fn import_libs(interpreter: &mut Interpreter) {
    interpreter.declare_function(&"sum".to_string(), &vec![
        ParserToken::CallNative(sum)
    ]);
}

fn sum(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut() }.unwrap();
    let arg2 = machine.pop();
    let arg1 = machine.pop();
    machine.push(arg1.add(arg2).unwrap());
}
