
#[derive(Debug)]
pub enum ValueE {
    ParsingError,
    TypeMismatch
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Litreal(String),
    Boolean(bool),
    Null,
}

impl Value {
    pub fn parse(s: &String) -> Result<Value, ValueE> {
        if let Ok(i) = s.parse::<i64>() {
            return Ok(Value::Int(i));
        }
        if s == "true" {
            return Ok(Value::Boolean(true));
        }
        if s == "false" {
            return Ok(Value::Boolean(false));
        }
        Err(ValueE::ParsingError)
    }

    pub fn do_operation(&self, op: &String, other: Value) -> Result<Value, ValueE> {
        match op.as_str() {
            "+" => {
                return self.add(other);
            }
            "-" => {
                return self.sub(other);
            }
            "*" => {
                return self.mul(other);
            }
            "/" => {
                return self.div(other);
            }
            _ => {
                panic!("Invalid operation");
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Int(i) => { return i.to_string(); },
            Value::Litreal(s) => { return s.clone(); },
            Value::Boolean(b) => { return if *b { "true".to_string() } else { "false".to_string() } }
            Value::Null => { return "null".to_string(); },
        }
    }
}

pub trait ValueAdder<T> {
    fn add(&self, rhs: T) -> Result<Value, ValueE>;
    fn sub(&self, rhs: T) -> Result<Value, ValueE>;
    fn mul(&self, rhs: T) -> Result<Value, ValueE>;
    fn div(&self, rhs: T) -> Result<Value, ValueE>;
}

// Value + Value
impl ValueAdder<Value> for Value {
    fn add(&self, rhs: Value) -> Result<Value, ValueE> {
        if let Value::Int(value) = rhs {
            return self.add(value);
        }
        return Err(ValueE::TypeMismatch);
    }
    
    fn sub(&self, rhs: Value) -> Result<Value, ValueE> {
        if let Value::Int(value) = rhs {
            return self.sub(value);
        }
        return Err(ValueE::TypeMismatch);
    }

    fn mul(&self, rhs: Value) -> Result<Value, ValueE> {
        if let Value::Int(value) = rhs {
            return self.mul(value);
        }
        return Err(ValueE::TypeMismatch);
    }

    fn div(&self, rhs: Value) -> Result<Value, ValueE> {
        if let Value::Int(value) = rhs {
            return self.div(value);
        }
        return Err(ValueE::TypeMismatch);
    }
}

// Value + i64
impl ValueAdder<i64> for Value {
    fn add(&self, rhs: i64) -> Result<Value, ValueE> {
        if let Value::Int(lhs) = self {
            return Ok(Value::Int(lhs.clone() + rhs));
        }
        return Err(ValueE::TypeMismatch);
    }
    
    fn sub(&self, rhs: i64) -> Result<Value, ValueE> {
        if let Value::Int(lhs) = self {
            return Ok(Value::Int(lhs.clone() - rhs));
        }
        return Err(ValueE::TypeMismatch);
    }

    fn mul(&self, rhs: i64) -> Result<Value, ValueE> {
        if let Value::Int(lhs) = self {
            return Ok(Value::Int(lhs.clone() * rhs));
        }
        return Err(ValueE::TypeMismatch);
    }

    fn div(&self, rhs: i64) -> Result<Value, ValueE> {
        if let Value::Int(lhs) = self {
            return Ok(Value::Int(lhs.clone() / rhs));
        }
        return Err(ValueE::TypeMismatch);
    }
}
