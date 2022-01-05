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
  // from, to blockの範囲 idx
  BLOCK(usize, usize),
}