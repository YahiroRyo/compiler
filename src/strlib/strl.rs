use crate::strlib::code::Code;
use std::process;

pub fn error_msg(msg: &str) {
  println!("{}", msg);
  process::exit(0x0100);
}
pub fn strtoi(code: &mut Code) -> i64 {
  let mut r = String::new();
  while !code.is_out() && code.c().is_digit(10) {
    r.push(code.next());
  }
  r.to_string().parse::<i64>().unwrap()
}
pub fn is_alnum(c: char) -> bool {
  (c >= 'a' && c <= 'z') ||
  (c >= 'A' && c <= 'Z') ||
  (c >= '0' && c <= '9') ||
  (c == '_')
}