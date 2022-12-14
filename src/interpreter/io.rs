use crate::parser::{Parser, ParserToken};
use crate::interpreter::{Interpreter};
use crate::value::{Value};

pub fn import_libs(interpreter: &mut Interpreter) {
    interpreter.declare_function(&"print".to_string(), &vec![
        ParserToken::CallNative(print)
    ]);
    interpreter.declare_function(&"delay_ms".to_string(), &vec![
        ParserToken::CallNative(delay_ms)
    ]);
    interpreter.declare_function(&"delay_s".to_string(), &vec![
        ParserToken::CallNative(delay_s)
    ]);
}

fn print(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut().unwrap() };
    let val = machine.pop();
    println!("{}", val.to_string());
}

fn delay_ms(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut().unwrap() };

    let val = machine.pop();
    if let Value::Int(ms) = val {
        if ms.is_negative() {
            machine.error(format!("Int cannot be a negative value, got {}!", ms));
        }
        use std::{thread, time};
        let millis = time::Duration::from_millis(ms as u64);
        thread::sleep(millis);
    }
}

fn delay_s(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut().unwrap() };

    let val = machine.pop();
    if let Value::Int(ms) = val {
        if ms.is_negative() {
            machine.error(format!("Int cannot be a negative value, got {}!", ms));
        }
        use std::{thread, time};
        let millis = time::Duration::from_secs(ms as u64);
        thread::sleep(millis);
    }
}
