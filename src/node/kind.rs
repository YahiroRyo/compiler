#[derive(Clone)]
pub struct Range {
  pub from: usize,
  pub to: usize,
}
#[derive(Clone)]
pub struct Func {
  pub range: Range,
  pub name: String,
}

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
  IF,
  ELSE,
  WHILE,
  FOR,
  NONE,
  ASSIGN,
  RETURN,
  NUM(i64),
  LVAR(usize),
  FUNC(Func),
  BLOCK(Range),
}