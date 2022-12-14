use std::collections::hash_map::RandomState;

use crate::lexer::{Lexer};
use crate::parser::{Parser, ParserToken};
use crate::interpreter::{Interpreter};
use crate::value::{ValueAdder, Value};

pub fn import_libs(interpreter: &mut Interpreter) {
    interpreter.declare_function(&"sum".to_string(), &vec![
        ParserToken::CallNative(sum)
    ]);
    interpreter.declare_function(&"abs".to_string(), &vec![
        ParserToken::CallNative(abs)
    ]);
    interpreter.declare_function(&"max".to_string(), &vec![
        ParserToken::CallNative(max)
    ]);
    interpreter.declare_function(&"min".to_string(), &vec![
        ParserToken::CallNative(min)
    ]);
    interpreter.declare_function(&"rand_rangei".to_string(), &vec![
        ParserToken::CallNative(rand_rangei)
    ]);
    interpreter.declare_function(&"rand_rangef".to_string(), &vec![
        ParserToken::CallNative(rand_rangef)
    ]);

    interpreter.declare_function(&"sin".to_string(), &vec![
        ParserToken::CallNative(sin)
    ]);
    interpreter.declare_function(&"tan".to_string(), &vec![
        ParserToken::CallNative(tan)
    ]);
    interpreter.declare_function(&"cos".to_string(), &vec![
        ParserToken::CallNative(cos)
    ]);
    interpreter.declare_function(&"sinh".to_string(), &vec![
        ParserToken::CallNative(sinh)
    ]);
    interpreter.declare_function(&"tanh".to_string(), &vec![
        ParserToken::CallNative(tanh)
    ]);
    interpreter.declare_function(&"cosh".to_string(), &vec![
        ParserToken::CallNative(cosh)
    ]);
}

fn sum(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut() }.unwrap();
    let arg2 = machine.pop();
    let arg1 = machine.pop();
    machine.push(arg1.add(arg2).unwrap());
}

fn abs(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut() }.unwrap();
    let arg = machine.pop().int();
    machine.push(Value::Int(arg.abs()));
}

fn max(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut() }.unwrap();
    let arg1 = machine.pop().int();
    let arg2 = machine.pop().int();
    let r = arg1.max(arg2);
    machine.push(Value::Int(r));
}

fn min(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut() }.unwrap();
    let arg1 = machine.pop().int();
    let arg2 = machine.pop().int();
    let r = arg1.min(arg2);
    machine.push(Value::Int(r));
}

fn sin(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut() }.unwrap();
    let arg = machine.pop().float();
    machine.push(Value::Float(arg.sin()));
}

fn tan(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut() }.unwrap();
    let arg = machine.pop().float();
    machine.push(Value::Float(arg.tan()));
}

fn cos(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut() }.unwrap();
    let arg = machine.pop().float();
    machine.push(Value::Float(arg.cos()));
}

fn sinh(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut() }.unwrap();
    let arg = machine.pop().float();
    machine.push(Value::Float(arg.sinh()));
}

fn tanh(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut() }.unwrap();
    let arg = machine.pop().float();
    machine.push(Value::Float(arg.tanh()));
}

fn cosh(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut() }.unwrap();
    let arg = machine.pop().float();
    machine.push(Value::Float(arg.cosh()));
}

fn rand_rangei(interpreter: *mut Interpreter) {
    use rand::Rng;

    let machine = unsafe { interpreter.as_mut() }.unwrap();
    let arg2 = machine.pop().int();
    let arg1 = machine.pop().int();
    let r = rand::thread_rng().gen_range(arg1..arg2);
    machine.push(Value::Int(r));
}

fn rand_rangef(interpreter: *mut Interpreter) {
    use rand::Rng;

    let machine = unsafe { interpreter.as_mut() }.unwrap();
    let arg2 = machine.pop().float();
    let arg1 = machine.pop().float();
    let r = rand::thread_rng().gen_range(arg1..arg2);
    machine.push(Value::Float(r));
}


