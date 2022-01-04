use crate::token::kind::TokenKind as TokenKind;
use crate::strlib::strl::{strtoi, error_msg};
use crate::strlib::code::Code;

pub struct TokenArray {
  pub tokens: Vec<TokenKind>,
  pub idx: usize,
}

impl TokenArray {
  fn new_token(&mut self, kind: TokenKind) {
    self.tokens.push(kind)
  }
  fn kind(&mut self) -> &TokenKind {
    &self.tokens[self.idx]
  }
  fn next(&mut self) -> &TokenKind {
    let tmp = &self.tokens[self.idx];
    self.idx += 1;
    tmp
  }
  pub fn is_eof(&mut self) -> bool {
    match self.kind() {
      TokenKind::EOF => true,
      _ => false
    }
  }
  pub fn consume(&mut self, s: &str) -> bool {
    match self.kind() {
      TokenKind::RESERVED (re) => {
        if re == s {
          self.idx += 1;
          return true;
        }
      },
      _ => ()
    }
    false
  }
  pub fn consume_ident(&mut self) -> (bool, String) {
    match &self.tokens[self.idx] {
      TokenKind::IDENT (s) => {
        self.idx += 1;
        (true, s.to_string())
      },
      _ => {
        (false, String::new())
      }
    }
  }
  pub fn expect_number(&mut self) -> i64 {
    match *self.kind() {
      TokenKind::NUM (n) => {
        self.idx += 1;
        n
      },
      _ => {
        error_msg("数ではありません。");
        0
      }
    }
  }
  pub fn expect(&mut self, s: &str) {
    match self.kind() {
      TokenKind::RESERVED (re) => {
        if re == s {
          self.idx += 1;
        } else {
          println!("{}", s);
          println!("{}", re);
          error_msg(&format!("{}ではありません。", s).to_string());
        }
      },
      _ => {
        error_msg(&format!("{}ではありません。", s).to_string());
      }
    }
  }
}

pub fn tokenize(code: &mut Code) -> TokenArray {
  let mut ret = TokenArray {
    tokens: Vec::new(),
    idx: 0,
  };

  while !code.is_out() {
    if code.c() == ' ' || code.c() == '\n' {
      code.idx += 1;
      continue;
    }
    
    let (is_reserved, reserved) = code.is_reserved();
    if is_reserved {
      ret.new_token(TokenKind::RESERVED(reserved.clone()));
      code.idx += reserved.len();
      continue;
    }

    if code.c() >= 'a' && code.c() <= 'z' {
      ret.new_token(TokenKind::IDENT(String::from(code.next())));
      continue;
    }

    if code.c().is_digit(10) {
      ret.new_token(TokenKind::NUM(strtoi(code)));
      continue;
    }

    error_msg("トークナイズできません。");
  }
  ret.new_token(TokenKind::EOF);
  ret
}