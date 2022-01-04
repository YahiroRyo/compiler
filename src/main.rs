mod strlib;
mod middleware;
mod token;
mod node;

use std::env;
use strlib::code::Code;
use node::node::NodeArray;
use node::parse::ParseArgs as ParseArgs;
use middleware::filter::middleware;
use token::token::{tokenize, TokenArray};

fn main() {
	let args: Vec<String> = env::args().collect();
	middleware(args.clone());
	let mut code: Code = Code::new(args[1].chars().collect());
	let token: TokenArray = tokenize(&mut code);
	let mut parse_args = ParseArgs { tokens: token };
	let mut nodes: Vec<NodeArray> = Vec::new();
	while !parse_args.tokens.is_eof() {
		let mut node = NodeArray {
			nodes: Vec::new(),
			idx: 0,
		};
		node.stmt(&mut parse_args);
		nodes.push(node);
	}

	println!(".intel_syntax noprefix");
	println!(".globl main");
	println!("main:");
	println!("  push rbp");
	println!("  mov rbp, rsp");
	println!("  sub rsp, 208");

	for mut node in nodes {
		node.gen(node.nodes.len()-1);

		println!("  pop rax");
	}
	
	println!("  mov rsp, rbp");
	println!("  pop rbp");
	println!("  ret");
}
