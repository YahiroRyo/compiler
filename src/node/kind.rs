#[derive(Clone)]
pub enum NodeKind {
  ADD,
  SUB,
  MUL,
  DIV,
  EQ,
  NE,
  LE,
  LT,
  ASSIGN,
  LVAR(usize),
  NUM(i64),
}