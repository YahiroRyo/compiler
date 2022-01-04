#[derive(Debug)]
pub enum TokenKind {
  RESERVED(String),
  IDENT(String),
  NUM(i64),
  RETURN,
  EOF,
}