
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
    Float(f64),
    Literal(String),
    Boolean(bool),
    Array(Vec<Value>),
    Ptr(*mut u32),
    Null,
}

impl Value {
    /**
     * Parses Ints, Floats, Bools, Null, etc.
     * Doesn't parse string literals!
     */
    pub fn parse(s: &String) -> Result<Value, ValueE> {
        if let Ok(i) = s.parse::<i64>() {
            return Ok(Value::Int(i));
        }
        if let Ok(f) = s.parse::<f64>() {
            return Ok(Value::Float(f));
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
            Value::Ptr(ptr) => {
                return !ptr.is_null();
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
            "<" => {
                return self.less_than(other);
            }
            ">" => {
                return self.greater_than(other);
            }
            "==" => {
                return Ok(Value::Boolean(*self == other));
            },
            "!=" => {
                return Ok(Value::Boolean(*self != other));
            }
            "&&" => {
                return Ok(Value::Boolean(self.is_true() && other.is_true()));
            },
            "||" => {
                return Ok(Value::Boolean(self.is_true() || other.is_true()));
            }
            _ => {
                panic!("Invalid operation");
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Int(i) => { return i.to_string(); },
            Value::Float(f) => { return f.to_string(); },
            Value::Literal(s) => { return s.clone(); },
            Value::Boolean(b) => { return if *b { "true".to_string() } else { "false".to_string() } }
            Value::Null => { return "null".to_string(); },
            Value::Ptr(ptr) => { return format!("{:?}", ptr); },
            Value::Array(array) => {
                if array.len() == 0 {
                    return "[]".to_string();
                }

                let mut str = String::from("[");
                let mut iter = array.iter().peekable();
                loop {
                    let element = iter.next().expect("this should be quaranteed be valid");
                    str += &element.to_string();

                    if let None = iter.peek() {
                        break;
                    }
                    str.push(',');
                }
                str.push(']');
                return str;
            }
        }
    }

    pub fn literal(&self) -> String {
        if let Value::Literal(s) = self {
            return s.clone();
        }
        panic!("Expected a String Literal got {:?} instead!", self);
    }

    pub fn int(&self) -> i64 {
        if let Value::Int(i) = self {
            return i.clone();
        }
        panic!("Expected an Int got {:?} instead!", self);
    }

    pub fn ptr(&self) -> *mut u32 {
        if let Value::Ptr(ptr) = self {
            return ptr.clone();
        }
        panic!("Expected a Pointer got {:?} instead!", self);
    }
}

pub trait ValueAdder<T> {
    fn add(&self, rhs: T) -> Result<Value, ValueE>;
    fn sub(&self, rhs: T) -> Result<Value, ValueE>;
    fn mul(&self, rhs: T) -> Result<Value, ValueE>;
    fn div(&self, rhs: T) -> Result<Value, ValueE>;
    fn less_than(&self, rhs: T) -> Result<Value, ValueE>;
    fn greater_than(&self, rhs: T) -> Result<Value, ValueE>;
    fn modulo(&self, rhs: T) -> Result<Value, ValueE>;
}

// Value + Value
impl ValueAdder<Value> for Value {
    fn add(&self, rhs: Value) -> Result<Value, ValueE> {
        if let Value::Int(value) = rhs {
            return self.add(value);
        }
        if let Value::Float(value) = rhs {
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
        if let Value::Float(value) = rhs {
            return self.sub(value);
        }
        if let Value::Literal(value) = rhs {
            return self.sub(value);
        }
        return Err(ValueE::TypeMismatch);
    }

    fn mul(&self, rhs: Value) -> Result<Value, ValueE> {
        if let Value::Int(value) = rhs {
            return self.mul(value);
        }
        if let Value::Float(value) = rhs {
            return self.mul(value);
        }
        if let Value::Literal(value) = rhs {
            return self.mul(value);
        }
        return Err(ValueE::TypeMismatch);
    }

    fn div(&self, rhs: Value) -> Result<Value, ValueE> {
        if let Value::Int(value) = rhs {
            return self.div(value);
        }
        if let Value::Float(value) = rhs {
            return self.div(value);
        }
        if let Value::Literal(value) = rhs {
            return self.div(value);
        }
        return Err(ValueE::TypeMismatch);
    }

    fn less_than(&self, rhs: Value) -> Result<Value, ValueE> {
        if let Value::Int(value) = rhs {
            return self.less_than(value);
        }
        if let Value::Float(value) = rhs {
            return self.less_than(value);
        }
        if let Value::Literal(value) = rhs {
            return self.less_than(value);
        }
        return Err(ValueE::TypeMismatch);
    }

    fn greater_than(&self, rhs: Value) -> Result<Value, ValueE> {
        if let Value::Int(value) = rhs {
            return self.greater_than(value);
        }
        if let Value::Float(value) = rhs {
            return self.greater_than(value);
        }
        if let Value::Literal(value) = rhs {
            return self.greater_than(value);
        }
        return Err(ValueE::TypeMismatch);
    }

    fn modulo(&self, rhs: Value) -> Result<Value, ValueE> {
        if let Value::Int(value) = rhs {
            return self.modulo(value);
        }
        if let Value::Float(value) = rhs {
            return self.modulo(value);
        }
        if let Value::Literal(value) = rhs {
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
        if let Value::Float(lhs) = self {
            return Ok(Value::Float(lhs.clone() + (rhs as f64)));
        }
        return Err(ValueE::TypeMismatch);
    }
    
    fn sub(&self, rhs: i64) -> Result<Value, ValueE> {
        if let Value::Int(lhs) = self {
            return Ok(Value::Int(lhs.clone() - rhs));
        }
        if let Value::Float(lhs) = self {
            return Ok(Value::Float(lhs.clone() - (rhs as f64)));
        }
        return Err(ValueE::TypeMismatch);
    }

    fn mul(&self, rhs: i64) -> Result<Value, ValueE> {
        if let Value::Int(lhs) = self {
            return Ok(Value::Int(lhs.clone() * rhs));
        }
        if let Value::Float(lhs) = self {
            return Ok(Value::Float(lhs.clone() * (rhs as f64)));
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
        if let Value::Float(lhs) = self {
            if rhs == 0 {
                return Err(ValueE::DivisionByZero);
            }
            return Ok(Value::Float(lhs.clone() / (rhs as f64)));
        }
        return Err(ValueE::TypeMismatch);
    }

    fn less_than(&self, rhs: i64) -> Result<Value, ValueE> {
        if let Value::Int(lhs) = self {
            return Ok(Value::Boolean(lhs.clone() < rhs));
        }
        if let Value::Float(lhs) = self {
            return Ok(Value::Boolean(lhs.clone() < (rhs as f64)));
        }
        return Err(ValueE::TypeMismatch);
    }

    fn greater_than(&self, rhs: i64) -> Result<Value, ValueE> {
        if let Value::Int(lhs) = self {
            return Ok(Value::Boolean(lhs.clone() > rhs));
        }
        if let Value::Float(lhs) = self {
            return Ok(Value::Boolean(lhs.clone() > (rhs as f64)));
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
        if let Value::Float(lhs) = self {
            if rhs == 0 {
                return Err(ValueE::DivisionByZero);
            }
            return Ok(Value::Float(lhs.clone() % (rhs as f64)));
        }
        return Err(ValueE::TypeMismatch);
    }
}

// Value + f64
impl ValueAdder<f64> for Value {
    fn add(&self, rhs: f64) -> Result<Value, ValueE> {
        if let Value::Int(lhs) = self {
            return Ok(Value::Int(lhs.clone() + (rhs as i64)));
        }
        if let Value::Float(lhs) = self {
            return Ok(Value::Float(lhs.clone() + rhs));
        }
        return Err(ValueE::TypeMismatch);
    }
    
    fn sub(&self, rhs: f64) -> Result<Value, ValueE> {
        if let Value::Int(lhs) = self {
            return Ok(Value::Int(lhs.clone() - (rhs as i64)));
        }
        if let Value::Float(lhs) = self {
            return Ok(Value::Float(lhs.clone() - rhs));
        }
        return Err(ValueE::TypeMismatch);
    }

    fn mul(&self, rhs: f64) -> Result<Value, ValueE> {
        if let Value::Int(lhs) = self {
            return Ok(Value::Int(lhs.clone() * (rhs as i64)));
        }
        if let Value::Float(lhs) = self {
            return Ok(Value::Float(lhs.clone() * rhs));
        }
        return Err(ValueE::TypeMismatch);
    }

    fn div(&self, rhs: f64) -> Result<Value, ValueE> {
        if let Value::Int(lhs) = self {
            if rhs == 0.0 {
                return Err(ValueE::DivisionByZero);
            }
            return Ok(Value::Int(lhs.clone() / (rhs as i64)));
        }
        if let Value::Float(lhs) = self {
            if rhs == 0.0 {
                return Err(ValueE::DivisionByZero);
            }
            return Ok(Value::Float(lhs.clone() / rhs));
        }
        return Err(ValueE::TypeMismatch);
    }

    fn less_than(&self, rhs: f64) -> Result<Value, ValueE> {
        if let Value::Int(lhs) = self {
            return Ok(Value::Boolean((lhs.clone() as f64) < rhs));
        }
        if let Value::Float(lhs) = self {
            return Ok(Value::Boolean(lhs.clone() < rhs));
        }
        return Err(ValueE::TypeMismatch);
    }

    fn greater_than(&self, rhs: f64) -> Result<Value, ValueE> {
        if let Value::Int(lhs) = self {
            return Ok(Value::Boolean((lhs.clone() as f64) > rhs));
        }
        if let Value::Float(lhs) = self {
            return Ok(Value::Boolean(lhs.clone() > rhs));
        }
        return Err(ValueE::TypeMismatch);
    }

    fn modulo(&self, rhs: f64) -> Result<Value, ValueE> {
        if let Value::Int(lhs) = self {
            if rhs == 0.0 {
                return Err(ValueE::DivisionByZero);
            }
            return Ok(Value::Int(lhs.clone() % (rhs as i64)));
        }
        if let Value::Float(lhs) = self {
            if rhs == 0.0 {
                return Err(ValueE::DivisionByZero);
            }
            return Ok(Value::Float(lhs.clone() % rhs));
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

    fn less_than(&self, _rhs: String) -> Result<Value, ValueE> {
        return Err(ValueE::UnkownOperation);
    }

    fn greater_than(&self, _rhs: String) -> Result<Value, ValueE> {
        return Err(ValueE::UnkownOperation);
    }

    fn modulo(&self, _rhs: String) -> Result<Value, ValueE> {
        return Err(ValueE::UnkownOperation);
    }
}

