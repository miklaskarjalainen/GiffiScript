
#[derive(Debug)]
pub enum ValueE {
    ParsingError,
    TypeMismatch,
    UnkownOperation,
    DivisionByZero,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Literal(String),
    Boolean(bool),
    Null,
}

impl Value {
    /**
     * Parses Ints, Bools, Null, etc.
     * Doesn't parse string literals!
     */
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
        if s == "null" {
            return Ok(Value::Null);
        }
        Err(ValueE::ParsingError)
    }

    pub fn is_true(&self) -> bool {
        match self {
            Value::Int(value) => {
                return value != &0;
            }
            Value::Literal(literal) => {
                return !literal.is_empty();
            }
            Value::Boolean(value) => {
                return value.clone();
            }
            Value::Null => {
                return false;
            }
            _ => {
                panic!("Not implemented yet!");
            }
        }
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
            "%" => {
                return self.modulo(other);
            }
            "==" => {
                return Ok(Value::Boolean(*self == other));
            },
            "!=" => {
                return Ok(Value::Boolean(*self != other));
            }
            _ => {
                panic!("Invalid operation");
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Int(i) => { return i.to_string(); },
            Value::Literal(s) => { return s.clone(); },
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
    fn modulo(&self, rhs: T) -> Result<Value, ValueE>;
}

// Value + Value
impl ValueAdder<Value> for Value {
    fn add(&self, rhs: Value) -> Result<Value, ValueE> {
        if let Value::Int(value) = rhs {
            return self.add(value);
        }
        if let Value::Literal(value) = rhs {
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

    fn modulo(&self, rhs: Value) -> Result<Value, ValueE> {
        if let Value::Int(value) = rhs {
            return self.modulo(value);
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
            if rhs == 0 {
                return Err(ValueE::DivisionByZero);
            }
            return Ok(Value::Int(lhs.clone() / rhs));
        }
        return Err(ValueE::TypeMismatch);
    }

    fn modulo(&self, rhs: i64) -> Result<Value, ValueE> {
        if let Value::Int(lhs) = self {
            if rhs == 0 {
                return Err(ValueE::DivisionByZero);
            }
            return Ok(Value::Int(lhs.clone() % rhs));
        }
        return Err(ValueE::TypeMismatch);
    }
}

// Value + String
impl ValueAdder<String> for Value {
    fn add(&self, rhs: String) -> Result<Value, ValueE> {
        if let Value::Literal(lhs) = self {
            return Ok(Value::Literal(lhs.clone() + &rhs));
        }
        return Err(ValueE::UnkownOperation);
    }
    
    fn sub(&self, _rhs: String) -> Result<Value, ValueE> {
        return Err(ValueE::UnkownOperation);
    }

    fn mul(&self, _rhs: String) -> Result<Value, ValueE> {
        return Err(ValueE::UnkownOperation);
    }

    fn div(&self, _rhs: String) -> Result<Value, ValueE> {
        return Err(ValueE::UnkownOperation);
    }

    fn modulo(&self, _rhs: String) -> Result<Value, ValueE> {
        return Err(ValueE::UnkownOperation);
    }
}

