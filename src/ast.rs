use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct File {
    pub name: String,
    pub expression: Term,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Location {
    pub start: i32,
    pub end: i32,
    pub filename: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Int {
    pub value: i32,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Bool {
    pub value: bool,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Str {
    pub value: String,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Binary {
    pub lhs: Box<Term>,
    pub op: BinaryOp,
    pub rhs: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Call {
    pub callee: Box<Term>,
    pub arguments: Vec<Box<Term>>,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Function {
    pub parameters: Vec<Var>,
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Var {
    pub text: String,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Parameter {
    pub text: String,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Let {
    pub name: Parameter,
    pub value: Box<Term>,
    pub next: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone)]
pub struct If {
    pub condition: Box<Term>,
    pub then: Box<Term>,
    pub otherwise: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Print {
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone)]
pub struct First {
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Second {
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tuple {
    pub first: Box<Term>,
    pub second: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Deserialize, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum Term {
    Int(Int),
    Str(Str),
    Bool(Bool),
    Binary(Binary),
    Call(Call),
    Function(Function),
    Let(Let),
    If(If),
    Print(Print),
    First(First),
    Second(Second),
    Tuple(Tuple),
    Var(Var),
}
