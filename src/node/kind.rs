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
  RETURN,
  NUM(i64),
  LVAR(usize),
}