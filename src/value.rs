
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum VErr {
    ParsingError
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Value {
    Int(i64),
    Null,
}

impl Value {
    pub fn parse(s: &String) -> Result<Value, VErr> {
        if let Ok(i) = s.parse::<i64>() {
            return Ok(Value::Int(i));
        }
        Err(VErr::ParsingError)
    }
}

pub trait ValueAdder<T> {
    fn add(&self, rhs: T) -> Result<Value, String>;
    fn sub(&self, rhs: T) -> Result<Value, String>;
    fn mul(&self, rhs: T) -> Result<Value, String>;
    fn div(&self, rhs: T) -> Result<Value, String>;
}

// Value + Value
impl ValueAdder<Value> for Value {
    fn add(&self, rhs: Value) -> Result<Value, String> {
        if let Value::Int(value) = rhs {
            return self.add(value);
        }
        return Err("Type mismatch".to_string());
    }
    
    fn sub(&self, rhs: Value) -> Result<Value, String> {
        if let Value::Int(value) = rhs {
            return self.sub(value);
        }
        return Err("Type mismatch".to_string());
    }

    fn mul(&self, rhs: Value) -> Result<Value, String> {
        if let Value::Int(value) = rhs {
            return self.mul(value);
        }
        return Err("Type mismatch".to_string());
    }

    fn div(&self, rhs: Value) -> Result<Value, String> {
        if let Value::Int(value) = rhs {
            return self.div(value);
        }
        return Err("Type mismatch".to_string());
    }
}

// Value + i64
impl ValueAdder<i64> for Value {
    fn add(&self, rhs: i64) -> Result<Value, String> {
        if let Value::Int(lhs) = self {
            return Ok(Value::Int(lhs.clone() + rhs));
        }
        return Err("Type mismatch".to_string());
    }
    
    fn sub(&self, rhs: i64) -> Result<Value, String> {
        if let Value::Int(lhs) = self {
            return Ok(Value::Int(lhs.clone() - rhs));
        }
        return Err("Type mismatch".to_string());
    }

    fn mul(&self, rhs: i64) -> Result<Value, String> {
        if let Value::Int(lhs) = self {
            return Ok(Value::Int(lhs.clone() * rhs));
        }
        return Err("Type mismatch".to_string());
    }

    fn div(&self, rhs: i64) -> Result<Value, String> {
        if let Value::Int(lhs) = self {
            return Ok(Value::Int(lhs.clone() / rhs));
        }
        return Err("Type mismatch".to_string());
    }
}
