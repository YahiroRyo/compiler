#[derive(Clone)]
pub struct Range {
  pub from: Option<usize>,
  pub to: Option<usize>,
}
#[derive(Clone)]
pub struct Call {
  pub range: Range,
  pub name: String,
}
#[derive(Clone)]
pub struct Func {
  pub gens: Vec<usize>,
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
  CALL(Call),
  FUNC(Func),
  BLOCK(Range),
}