mod strlib;
mod middleware;
mod token;
mod node;
mod lvar;

use std::env;
use strlib::code::Code;
use node::node::NodeArray;
use node::parse::ParseArgs;
use lvar::lvar::LVarArray;
use token::token::{tokenize, TokenArray};
use middleware::filter::middleware;

fn main() {
	let args: Vec<String> = env::args().collect();
	middleware(args.clone());
	let mut code: Code = Code::new(args[1].chars().collect());
	let token: TokenArray = tokenize(&mut code);
	let mut parse_args = ParseArgs {
		tokens: token,
		lvars: LVarArray {
			lvars: Vec::new()
		}
	};
	let mut nodes: Vec<NodeArray> = Vec::new();
	let mut cnt: i64 = 0;
	
	while !parse_args.tokens.is_eof() {
		let mut node = NodeArray {
			nodes: Vec::new(),
			idx: 0,
		};
		node.func(&mut parse_args);
		nodes.push(node);
	}

	println!(".intel_syntax noprefix");
	println!(".globl main");

	for mut node in nodes {
		node.gen(node.nodes.len()-1, &mut cnt);
	}
}
