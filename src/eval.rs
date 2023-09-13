use crate::ast::{self, *};
use std::{collections::HashMap, fmt};

pub type Stack = Vec<Term>;
pub type Ctx = HashMap<String, Term>;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn eval(&mut self, ast: &str) {
        let file: File = serde_json::from_str(ast).expect("Invalid AST format.");
        self.eval_expr(&file.expression, &mut Ctx::new(), &mut Stack::new());
    }

    pub fn eval_expr(&mut self, expr: &Term, ctx: &mut Ctx, call_stack: &mut Stack) -> Value {
        match expr {
            Term::Let(l) => {
                ctx.insert(l.name.text.to_owned(), *l.value.clone());
                return self.eval_expr(&l.next, ctx, call_stack);
            }
            Term::Function(f) => {
                let mut local_ctx = ctx.clone();
                for (param, arg) in f.parameters.iter().zip(call_stack.iter()) {
                    local_ctx.insert(param.text.to_owned(), arg.clone());
                }
                return self.eval_expr(&f.value, &mut local_ctx, call_stack);
            }
            Term::Call(c) => {
                let mut stack = Stack::new();
                for arg in &c.arguments {
                    let evaluated_arg = self.eval_expr(arg, ctx, &mut stack).to_term();
                    stack.push(evaluated_arg);
                }
                self.eval_expr(&c.callee, ctx, &mut stack)
            }
            Term::Var(v) => {
                let variable = ctx
                    .get(&v.text)
                    .expect(&format!("Undefined variable `{}`", v.text))
                    .clone();
                return self.eval_expr(&variable, ctx, call_stack);
            }
            Term::Tuple(t) => {
                return Value::Tuple(
                    Box::new(self.eval_expr(&t.first, ctx, call_stack)),
                    Box::new(self.eval_expr(&t.second, ctx, call_stack)),
                )
            }
            Term::First(f) => {
                let v = self.eval_expr(&f.value, ctx, call_stack);
                match v {
                    Value::Tuple(a, _) => return *a,
                    _ => panic!("Called `first` on a non tuple value"),
                }
            }
            Term::Second(s) => {
                let v = self.eval_expr(&s.value, ctx, call_stack);
                match v {
                    Value::Tuple(_, b) => return *b,
                    _ => panic!("Called `second` on a non tuple value"),
                }
            }
            Term::If(i) => {
                if self.eval_expr(&i.condition, ctx, call_stack).to_bool() {
                    return self.eval_expr(&i.then, ctx, call_stack);
                }
                return self.eval_expr(&i.otherwise, ctx, call_stack);
            }
            Term::Print(p) => {
                println!("{}", self.eval_expr(&p.value, ctx, call_stack));
                Value::Nil
            }
            Term::Binary(b) => match b.op {
                BinaryOp::Add => {
                    let lhs = self.eval_expr(&b.lhs, ctx, call_stack);
                    let rhs = self.eval_expr(&b.rhs, ctx, call_stack);

                    if lhs.is_string() || rhs.is_string() {
                        return Value::Str(format!("{}{}", lhs.to_string(), rhs.to_string()));
                    } else if lhs.is_int() && rhs.is_int() {
                        return Value::Int(lhs.to_int() + rhs.to_int());
                    } else {
                        panic!("Invalid add operation")
                    }
                }
                BinaryOp::Mul => {
                    let lhs = self.eval_expr(&b.lhs, ctx, call_stack);
                    let rhs = self.eval_expr(&b.rhs, ctx, call_stack);
                    if lhs.is_int() && rhs.is_int() {
                        return Value::Int(lhs.to_int() * rhs.to_int());
                    } else {
                        panic!("Invalid mul operation")
                    }
                }
                BinaryOp::Div => {
                    let lhs = self.eval_expr(&b.lhs, ctx, call_stack);
                    let rhs = self.eval_expr(&b.rhs, ctx, call_stack);
                    if lhs.is_int() && rhs.is_int() {
                        return Value::Int(lhs.to_int() / rhs.to_int());
                    } else {
                        panic!("Invalid div operation")
                    }
                }
                BinaryOp::Rem => {
                    let lhs = self.eval_expr(&b.lhs, ctx, call_stack);
                    let rhs = self.eval_expr(&b.rhs, ctx, call_stack);
                    if lhs.is_int() && rhs.is_int() {
                        return Value::Int(lhs.to_int() % rhs.to_int());
                    } else {
                        panic!("Invalid rem operation")
                    }
                }
                BinaryOp::Sub => {
                    let lhs = self.eval_expr(&b.lhs, ctx, call_stack);
                    let rhs = self.eval_expr(&b.rhs, ctx, call_stack);
                    if lhs.is_int() && rhs.is_int() {
                        return Value::Int(lhs.to_int() - rhs.to_int());
                    } else {
                        panic!("Invalid sub operation")
                    }
                }
                BinaryOp::Eq => Value::Bool(
                    self.eval_expr(&b.lhs, ctx, call_stack)
                        == self.eval_expr(&b.rhs, ctx, call_stack),
                ),
                BinaryOp::Neq => Value::Bool(
                    self.eval_expr(&b.lhs, ctx, call_stack)
                        != self.eval_expr(&b.rhs, ctx, call_stack),
                ),
                BinaryOp::Gt => Value::Bool(
                    self.eval_expr(&b.lhs, ctx, call_stack)
                        > self.eval_expr(&b.rhs, ctx, call_stack),
                ),
                BinaryOp::Gte => Value::Bool(
                    self.eval_expr(&b.lhs, ctx, call_stack)
                        >= self.eval_expr(&b.rhs, ctx, call_stack),
                ),
                BinaryOp::Lt => Value::Bool(
                    self.eval_expr(&b.lhs, ctx, call_stack)
                        < self.eval_expr(&b.rhs, ctx, call_stack),
                ),
                BinaryOp::Lte => Value::Bool(
                    self.eval_expr(&b.lhs, ctx, call_stack)
                        <= self.eval_expr(&b.rhs, ctx, call_stack),
                ),
                BinaryOp::And => Value::Bool(
                    self.eval_expr(&b.lhs, ctx, call_stack).to_bool()
                        && self.eval_expr(&b.rhs, ctx, call_stack).to_bool(),
                ),
                BinaryOp::Or => Value::Bool(
                    self.eval_expr(&b.lhs, ctx, call_stack).to_bool()
                        || self.eval_expr(&b.rhs, ctx, call_stack).to_bool(),
                ),
            },
            Term::Int(i) => return Value::Int(i.value),
            Term::Str(s) => return Value::Str(s.value.to_owned()),
            Term::Bool(b) => return Value::Bool(b.value),
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self {
            Self::Int(i) => write!(f, "{}", i),
            Self::Str(s) => write!(f, "{}", s),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Tuple(v1, v2) => write!(f, "({}, {})", *v1, *v2),
            Self::Nil => write!(f, "nil"),
        }
    }
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

    pub fn to_term(&self) -> Term {
        match self {
            Value::Int(i) => {
                return Term::Int(ast::Int {
                    value: *i,
                    location: Location {
                        start: 0,
                        end: 0,
                        filename: String::new(),
                    },
                });
            }
            Value::Str(s) => {
                return Term::Str(ast::Str {
                    value: s.to_string(),
                    location: Location {
                        start: 0,
                        end: 0,
                        filename: String::new(),
                    },
                })
            }
            Value::Bool(b) => {
                return Term::Bool(ast::Bool {
                    value: *b,
                    location: Location {
                        start: 0,
                        end: 0,
                        filename: String::new(),
                    },
                })
            }
            _ => panic!("Could not build Term from Value"),
        }
    }
}
