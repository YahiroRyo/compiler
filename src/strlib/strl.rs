use crate::strlib::code::Code;

pub fn error_msg(msg: &str) {
  println!("{}", msg);
}
pub fn strtoi(code: &mut Code) -> i64 {
  let mut r = String::new();
  while !code.is_out() && code.c().is_digit(10) {
    r.push(code.next());
  }
  r.to_string().parse::<i64>().unwrap()
}