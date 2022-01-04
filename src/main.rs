mod strlib;
mod middleware;
mod token;
mod node;

use std::env;
use strlib::code::Code;
use token::token::{tokenize, TokenArray};
use middleware::filter::middleware;

fn main() {
	let args: Vec<String> = env::args().collect();
	middleware(args.clone());
	let mut code: Code = Code::new(args[1].chars().collect());
	let mut token: TokenArray = tokenize(&mut code);

	println!(".intel_syntax noprefix");
	println!(".globl main");
	println!("main:");
	println!("  mov rax, {}", token.expect_number());

	while !token.is_eof() {
		if token.consume("+") {
			println!("  add rax, {}", token.expect_number());
			continue;
		}
		
		token.expect("-");
		println!("  sub rax, {}", token.expect_number());
	}
	println!("  ret");
}
