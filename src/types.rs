#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Num(f64),
    Bool(bool),
    Nil,
}
