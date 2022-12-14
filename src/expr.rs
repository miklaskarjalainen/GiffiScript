use std::collections::VecDeque;
use crate::lexer::{LexerToken};
use crate::parser::{ParserToken};

const MAX_PRECEDENCE:u8 = 7;

#[derive(Debug)]
pub struct AstExpr {
    pub token: ParserToken,
    pub rhs: Option<Box<AstExpr>>,
    pub lhs: Option<Box<AstExpr>>,
}

impl AstExpr {
    pub fn new(token: ParserToken, lhs: Option<Box<AstExpr>>, rhs: Option<Box<AstExpr>>) -> AstExpr {
        AstExpr {
            token: token,
            lhs: lhs,
            rhs: rhs,
        }
    }

    /**
     * Returns an evaluated vector
     * (40 + 40) * 2 -> (40, 40, '+', 2, '*')
     */
    pub fn evaluate(mut expr: &mut Vec<ParserToken>) -> Vec<ParserToken> {
        if expr.len() == 0 {
            return vec![];
        }
        // Make array is already evaluated
        if let Some(ParserToken::MakeArray(_)) = expr.last() {
            return expr.to_vec();
        }

        // Turns the expressions to a tree
        let ast = AstExpr::to_ast(&mut expr, 0);
        // Turns the tree into a stack like vector.
        ast.to_tokens()
    }

    pub fn to_tokens(&self) -> Vec<ParserToken> {
        let mut v:Vec<ParserToken> = vec![];

        if let Some(rhs) = &self.lhs {
            v.append(&mut rhs.to_tokens())
        }

        if let Some(lhs) = &self.rhs {
            v.append(&mut lhs.to_tokens())
        }

        v.push(self.token.clone());
        return v;
    }

    // https://en.wikipedia.org/wiki/Operator-precedence_parser
    fn parse_primary(input: &mut Vec<ParserToken>) -> AstExpr {
        let token = input.pop().expect("failed to pop");
        if let ParserToken::Push(_) = &token {
            return AstExpr::new(token, None, None);
        }
        else if let ParserToken::GetVariable(_) = &token {
            return AstExpr::new(token, None, None);
        }
        else if let ParserToken::Call(_, _) = &token {
            return AstExpr::new(token, None, None);
        }
        else if let ParserToken::GetArrayElement(_) = &token {
            return AstExpr::new(token, None, None);
        }
        else if let ParserToken::GetVariableArrayElement(_, _) = &token {
            return AstExpr::new(token, None, None);
        }
        // TODO: do parens
        /*
        else if &ParserToken::Operation("(".to_string()) == &token {
            let gg = AstExpr::parse_primary(input);
            return gg;
        }
        */
        panic!("lol");
    }

    fn to_ast(input: &mut Vec<ParserToken>, prec: u8) -> AstExpr {
        if prec >= MAX_PRECEDENCE {
            return AstExpr::parse_primary(input);
        }

        let lhs = AstExpr::to_ast(input, prec + 1);
        let token_opt = input.pop();

        if let Some(token) = token_opt {
            if AstExpr::get_precedence(&token) == prec {
                let rhs = AstExpr::to_ast(input, prec);
                return AstExpr::new(token, Some(Box::new(lhs)), Some(Box::new(rhs)));
            }
            else {
                input.push(token);
            }
        } 
        
        return lhs;
    }

    fn get_precedence(tk: &ParserToken) -> u8 {
        if let ParserToken::Operation(op) = tk {
            match op.as_str() {
                "&&" | "||" => {
                    return 0u8;
                },
                "<<" | ">>" => {
                    return 1u8;
                },
                "<" | ">" => {
                    return 2u8;
                },
                "==" | "!=" => {
                    return 3u8;
                }
                "+" | "-" => {
                    return 4u8;
                }
                "*" | "/" | "%" => {
                    return 5u8;
                }
                "!" => {
                    return 6u8;
                }
                "(" | ")" => {
                    return MAX_PRECEDENCE;
                }
                _ => {
                    // assert!(false, "unkown operator");
                    return MAX_PRECEDENCE;
                }
            }
        }
        panic!("{:?} is not an Operator", tk);
    }
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use crate::lexer::{LexerToken};
    use crate::parser::{ParserToken};
    use crate::value::Value;
    use super::AstExpr;

    fn test_evaluator(mut to_eval: Vec<ParserToken>) -> Option<Value> {
        let evaluated = AstExpr::evaluate(&mut to_eval);
        
        let mut stack = vec![];
        for tk in evaluated {
            match tk {
                ParserToken::Push(val) => {
                    stack.push(val.clone());
                }
                ParserToken::Operation(op) => {
                    let arg1 = stack.pop().expect("couldn't grab an argument for an operation");
                    let arg2 = stack.pop().expect("couldn't grab an argument for an operation");
                    let r = arg1.do_operation(&op, arg2).expect("error during an operation");
                    stack.push(r);
                }
                _ => {
                    panic!("Invalid parser token from evaluation");
                }
            }
        }
        stack.pop()
    }
    
    // TODO: Parens '(' ')'
    #[test]
    fn test_operator_precedence() {
        // 1+2*3 == 7
        let mut first = vec![];
        first.push(ParserToken::Push(Value::Int(1)));
        first.push(ParserToken::Operation("+".to_string()));
        first.push(ParserToken::Push(Value::Int(2)));
        first.push(ParserToken::Operation("*".to_string()));
        first.push(ParserToken::Push(Value::Int(3)));
        assert_eq!(test_evaluator(first).expect("error"), Value::Int(7));

        // 8/4/2 == 1
        let mut second = vec![];
        second.push(ParserToken::Push(Value::Int(8)));
        second.push(ParserToken::Operation("/".to_string()));
        second.push(ParserToken::Push(Value::Int(4)));
        second.push(ParserToken::Operation("/".to_string()));
        second.push(ParserToken::Push(Value::Int(2)));
        assert_eq!(test_evaluator(second).expect("error"), Value::Int(1));
    }

}
