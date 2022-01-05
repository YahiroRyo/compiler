use crate::token::kind::TokenKind as TokenKind;
use crate::strlib::strl::{strtoi, is_alnum, error_msg};
use crate::strlib::code::Code;

pub struct TokenArray {
  pub tokens: Vec<TokenKind>,
  pub idx: usize,
}

impl TokenArray {
  fn new_token(&mut self, kind: TokenKind) {
    self.tokens.push(kind)
  }
  pub fn kind(&mut self) -> &TokenKind {
    &self.tokens[self.idx]
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
  pub fn consume_return(&mut self) -> bool {
    match self.kind() {
      TokenKind::RETURN => {
        self.idx += 1;
        true
      },
      _ => false
    }
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

    if code.strcmp("return") && !is_alnum(code.chars[code.idx+6]) {
      ret.new_token(TokenKind::RETURN);
      code.idx += 6;
      continue;
    }
    if code.strcmp("if") {
      ret.new_token(TokenKind::RESERVED(String::from("if")));
      code.idx += 2;
      continue;
    }
    if code.strcmp("else") {
      ret.new_token(TokenKind::RESERVED(String::from("else")));
      code.idx += 4;
      continue;
    }
    if code.strcmp("while") {
      ret.new_token(TokenKind::RESERVED(String::from("while")));
      code.idx += 5;
      continue;
    }
    if code.strcmp("for") {
      ret.new_token(TokenKind::RESERVED(String::from("for")));
      code.idx += 3;
      continue;
    }

    if code.c().is_digit(10) {
      ret.new_token(TokenKind::NUM(strtoi(code)));
      continue;
    }

    if code.c() >= 'a' && code.c() <= 'z' {
      ret.new_token(TokenKind::IDENT(code.take_ident()));
      continue;
    }

    error_msg("トークナイズできません。");
  }
  ret.new_token(TokenKind::EOF);
  ret
}