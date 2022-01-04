mod strlib;
mod middleware;
mod token;
mod node;

use std::env;
use strlib::code::Code;
use node::node::NodeArray;
use node::parse::ParseArgs;
use middleware::filter::middleware;
use token::token::{tokenize, TokenArray};

fn main() {
	let args: Vec<String> = env::args().collect();
	middleware(args.clone());
	let mut code: Code = Code::new(args[1].chars().collect());
	let token: TokenArray = tokenize(&mut code);
	let mut parse_args = ParseArgs { tokens: token };
	let mut nodes = NodeArray {
		nodes: Vec::new(),
		idx: 0,
	};
	nodes.expr(&mut parse_args);

	println!(".intel_syntax noprefix");
	println!(".globl main");
	println!("main:");

	nodes.gen(nodes.nodes.len()-1);
	
	println!("  pop rax");
	println!("  ret");
}
