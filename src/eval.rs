use crate::ast::*;
use std::collections::HashMap;

pub struct Interpreter {
    pub fn_ctx: HashMap<String, Function>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            fn_ctx: HashMap::new(),
        }
    }

    pub fn eval(&mut self, ast: &str) {
        let file: File = serde_json::from_str(ast).expect("Invalid AST format.");
        self.eval_expr(file.expression);
    }

    pub fn eval_expr(&mut self, expr: Term) -> Value {
        match expr {
            Term::Tuple(t) => {
                return Value::Tuple(
                    Box::new(self.eval_expr(*t.first)),
                    Box::new(self.eval_expr(*t.second)),
                )
            }
            Term::First(f) => {
                let v = self.eval_expr(*f.value);
                match v {
                    Value::Tuple(a, _) => return *a,
                    _ => panic!("Called `first` on a non tuple value"),
                }
            }
            Term::Second(s) => {
                let v = self.eval_expr(*s.value);
                match v {
                    Value::Tuple(_, b) => return *b,
                    _ => panic!("Called `second` on a non tuple value"),
                }
            }

            Term::If(i) => {
                if self.eval_expr(*i.condition).to_bool() {
                    return self.eval_expr(*i.then);
                }
                return self.eval_expr(*i.otherwise);
            }
            Term::Print(p) => {
                print!("{:?}", self.eval_expr(*p.value));
                Value::Nil
            }
            Term::Binary(b) => match b.op {
                BinaryOp::Add => {
                    let lhs = self.eval_expr(*b.lhs);
                    let rhs = self.eval_expr(*b.rhs);

                    if lhs.is_string() || rhs.is_string() {
                        return Value::Str(format!("{}{}", lhs.to_string(), rhs.to_string()));
                    } else if lhs.is_int() && rhs.is_int() {
                        return Value::Int(lhs.to_int() + rhs.to_int());
                    } else {
                        panic!("Invalid add operation")
                    }
                }
                BinaryOp::Mul => {
                    let lhs = self.eval_expr(*b.lhs);
                    let rhs = self.eval_expr(*b.rhs);
                    if lhs.is_int() && rhs.is_int() {
                        return Value::Int(lhs.to_int() * rhs.to_int());
                    } else {
                        panic!("Invalid mul operation")
                    }
                }
                BinaryOp::Div => {
                    let lhs = self.eval_expr(*b.lhs);
                    let rhs = self.eval_expr(*b.rhs);
                    if lhs.is_int() && rhs.is_int() {
                        return Value::Int(lhs.to_int() / rhs.to_int());
                    } else {
                        panic!("Invalid div operation")
                    }
                }
                BinaryOp::Rem => {
                    let lhs = self.eval_expr(*b.lhs);
                    let rhs = self.eval_expr(*b.rhs);
                    if lhs.is_int() && rhs.is_int() {
                        return Value::Int(lhs.to_int() % rhs.to_int());
                    } else {
                        panic!("Invalid rem operation")
                    }
                }
                BinaryOp::Sub => {
                    let lhs = self.eval_expr(*b.lhs);
                    let rhs = self.eval_expr(*b.rhs);
                    if lhs.is_int() && rhs.is_int() {
                        return Value::Int(lhs.to_int() - lhs.to_int());
                    } else {
                        panic!("Invalid sub operation")
                    }
                }
                BinaryOp::Eq => Value::Bool(self.eval_expr(*b.lhs) == self.eval_expr(*b.rhs)),
                BinaryOp::Neq => Value::Bool(self.eval_expr(*b.lhs) != self.eval_expr(*b.rhs)),
                BinaryOp::Gt => Value::Bool(self.eval_expr(*b.lhs) > self.eval_expr(*b.rhs)),
                BinaryOp::Gte => Value::Bool(self.eval_expr(*b.lhs) >= self.eval_expr(*b.rhs)),
                BinaryOp::Lt => Value::Bool(self.eval_expr(*b.lhs) < self.eval_expr(*b.rhs)),
                BinaryOp::Lte => Value::Bool(self.eval_expr(*b.lhs) <= self.eval_expr(*b.rhs)),
                BinaryOp::And => Value::Bool(
                    self.eval_expr(*b.lhs).to_bool() && self.eval_expr(*b.rhs).to_bool(),
                ),
                BinaryOp::Or => Value::Bool(
                    self.eval_expr(*b.lhs).to_bool() || self.eval_expr(*b.rhs).to_bool(),
                ),
            },
            Term::Int(i) => return Value::Int(i.value),
            Term::Str(s) => return Value::Str(s.value),
            Term::Bool(b) => return Value::Bool(b.value),
            _ => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Value {
    Int(i32),
    Str(String),
    Bool(bool),
    Tuple(Box<Value>, Box<Value>),
    Nil,
}

impl Value {
    pub fn is_int(&self) -> bool {
        match self {
            Value::Int(_) => return true,
            Value::Str(_) => return false,
            Value::Bool(_) => return false,
            Value::Tuple(_, _) => return false,
            Value::Nil => false,
        }
    }

    pub fn to_int(&self) -> i32 {
        match self {
            Value::Int(i) => *i,
            Value::Str(_) => panic!("Could not convert string to int"),
            Value::Bool(_) => panic!("Could not convert bool to int"),
            Value::Tuple(_, _) => panic!("Could not convert tuple to int"),
            Value::Nil => panic!("Could not convert nil to int"),
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Value::Int(i) => *i != 0,
            Value::Str(s) => s != "",
            Value::Bool(b) => *b,
            Value::Tuple(_, _) => false,
            Value::Nil => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Value::Int(_) => false,
            Value::Str(_) => true,
            Value::Bool(_) => false,
            Value::Tuple(_, _) => false,
            Value::Nil => false,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Int(i) => return format!("{}", i),
            Value::Str(s) => return format!("{}", s),
            Value::Bool(b) => return format!("{}", b),
            Value::Tuple(a, b) => return format!("({},{})", a.to_string(), b.to_string()),
            Value::Nil => return format!(""),
        }
    }
}
