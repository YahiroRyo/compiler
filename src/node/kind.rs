#[derive(Clone)]
pub enum NodeKind {
  ADD,
  SUB,
  MUL,
  DIV,
  NUM(i64),
}