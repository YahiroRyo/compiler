use crate::token::kind::TokenKind;
use crate::token::token::TokenArray;

pub struct LVar {
  pub name: String,
  pub offset: usize,
}
pub struct LVarArray {
  pub lvars: Vec<LVar>
}

impl LVarArray {
  pub fn find_lvar(&mut self, tokens: &mut TokenArray) -> (bool, usize) {
    match tokens.kind() {
      TokenKind::IDENT (s) => {
        let mut is_exist = false;
        let mut offset = 0;
        for lvar in &self.lvars {
          if lvar.name == *s {
            is_exist = true;
            offset = lvar.offset;
          }
        }
        return (is_exist, offset);
      },
      _ => ()
    }
    (false, 0)
  }
  pub fn new_lvar(&mut self, name: String, offset: usize) {
    self.lvars.push(LVar {
      name: name,
      offset: offset,
    });
  }
}