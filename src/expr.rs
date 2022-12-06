use crate::lexer::{LexerToken};
use crate::parser::{ParserToken};

#[derive(Debug)]
pub struct AstExpr {
    pub token: LexerToken,
    pub rhs: Option<Box<AstExpr>>,
    pub lhs: Option<Box<AstExpr>>,
}

impl AstExpr {
    pub fn new(token: LexerToken, lhs: Option<Box<AstExpr>>, rhs: Option<Box<AstExpr>>) -> AstExpr {
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
    pub fn evaluate(mut expr: &mut Vec<LexerToken>) -> Vec<ParserToken> {
        // Turns the expressions to a tree
        let ast = AstExpr::to_ast(&mut expr, 0);
        
        // Turns the tree into a stack like vector.
        ast.to_tokens()
    }

    pub fn to_tokens(&self) -> Vec<ParserToken> {
        let mut v:Vec<ParserToken> = vec![];

        if let Some(lhs) = &self.lhs {
            v.append(&mut lhs.to_tokens())
        }

        if let Some(rhs) = &self.rhs {
            v.append(&mut rhs.to_tokens())
        }

        let tk = self.token.clone();
        if let LexerToken::Identifier(var_name) = tk {
            v.push(ParserToken::GetVariable(var_name));
        }
        else if let LexerToken::Value(value) = tk {
            v.push(ParserToken::Push(value));
        }
        else if let LexerToken::Operator(op) = tk {
            v.push(ParserToken::Operation(op));
        }
        else {
            panic!("should not happen");
        }
        return v;
    }

    // https://en.wikipedia.org/wiki/Operator-precedence_parser
    fn parse_primary(input: &mut Vec<LexerToken>) -> AstExpr {
        let token = input.pop().expect("failed to pop");
        if let LexerToken::Value(_) = &token {
            return AstExpr::new(token, None, None);
        }
        else if let LexerToken::Identifier(_) = &token {
            return AstExpr::new(token, None, None);
        }
        // TODO: do parens
        panic!("lol");
    }

    fn to_ast(input: &mut Vec<LexerToken>, prec: u8) -> AstExpr {
        if prec >= 2 {
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

    fn get_precedence(tk: &LexerToken) -> u8 {
        if let LexerToken::Operator(op) = tk {
            match op.as_str() {
                "+" | "-" => {
                    return 0u8;
                }
                "*" | "/" | "%" => {
                    return 1u8;
                }
                "(" | ")" => {
                    return 2u8;
                }
                _ => {
                    // assert!(false, "unkown operator");
                    return 2u8;
                }
            }
        }
        panic!("{:?} is not an Operator", tk);
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::{Lexer, LexerToken};
    use crate::parser::{ParserToken, Parser};
    use crate::value::Value;
    use super::AstExpr;

    fn test_evaluator(mut to_eval: Vec<LexerToken>) -> Option<Value> {
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
        let first = vec![
            LexerToken::Value(Value::Int(1)),
            LexerToken::Operator("+".to_string()),
            LexerToken::Value(Value::Int(2)),
            LexerToken::Operator("*".to_string()),
            LexerToken::Value(Value::Int(3)),
        ];
        assert_eq!(test_evaluator(first).expect("error"), Value::Int(7));

        // 2*3+1 == 7
        let first = vec![
            LexerToken::Value(Value::Int(2)),
            LexerToken::Operator("*".to_string()),
            LexerToken::Value(Value::Int(3)),
            LexerToken::Operator("+".to_string()),
            LexerToken::Value(Value::Int(1)),
        ];
        assert_eq!(test_evaluator(first).expect("error"), Value::Int(7));
    }


}
