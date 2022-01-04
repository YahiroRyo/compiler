const RESERVED_TERMS: [&'static str; 6] = [
  "+",
  "-",
  "/",
  "*",
  "(",
  ")",
];

pub struct Code {
  pub chars: Vec<char>,
  pub idx: usize,
}
impl Code {
  pub fn new(chars: Vec<char>) -> Code {
    Code {
      chars: chars,
      idx: 0,
    }
  }
  pub fn next(&mut self) -> char {
    let tmp = self.chars[self.idx];
    self.idx += 1;
    tmp
  }
  pub fn c(&mut self) -> char {
    self.chars[self.idx]
  }
  pub fn is_out(&mut self) -> bool {
    self.idx == self.chars.len()
  }
  pub fn is_reserved(&mut self) -> bool {
    for term in RESERVED_TERMS {
      let mut tmp_char: String = String::new();
      for i in 0..term.len() {
        tmp_char.push(self.chars[self.idx + i]);
      }
      if tmp_char == term {
        return true;
      }
    }
    false
  }
}