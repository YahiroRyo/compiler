#[derive(Debug)]
pub enum TokenKind {
  RESERVED(String),
  NUM(i64),
  EOF,
}