use std::process;
use crate::strlib::strl::error_msg;

pub fn error(args: Vec<String>) {
  if args.len() != 2 {
    error_msg("引数の個数に誤りがあります。");
    process::exit(1);
  }
}