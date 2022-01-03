mod strlib;
mod middleware;

use std::env;
use strlib::code::Code;
use strlib::strl::strtoi;
use middleware::filter::middleware;

fn main() {
	let args: Vec<String> = env::args().collect();
	middleware(args.clone());
	let mut code: Code = Code::new(args[1].chars().collect());

	println!(".intel_syntax noprefix");
	println!(".globl main");
	println!("main:");

	println!("  mov rax, {}", strtoi(&mut code));
	println!("  ret");
}
