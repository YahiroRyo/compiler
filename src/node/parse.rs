use crate::node::kind::NodeKind as NodeKind;
use crate::node::node::{NodeArray};
use crate::token::token::TokenArray;

pub struct ParseArgs {
  pub tokens: TokenArray
}

impl NodeArray {
  pub fn expr(&mut self, args: &mut ParseArgs) -> usize {
    let mut index = self.mul(args);
    loop {
      if args.tokens.consume("+") {
        let rhs = self.mul(args);
        index = self.new_node_usize(NodeKind::ADD, index, rhs);
      } else if args.tokens.consume("-") {
        let rhs = self.mul(args);
        index = self.new_node_usize(NodeKind::SUB, index, rhs);
      } else {
        return index;
      }
    }
  }
  fn mul(&mut self, args: &mut ParseArgs) -> usize {
    let mut index = self.primary(args);
    loop {
      if args.tokens.consume("*") {
        let rhs = self.primary(args);
        index = self.new_node_usize(NodeKind::MUL, index, rhs);
      } else if args.tokens.consume("/") {
        let rhs = self.primary(args);
        index = self.new_node_usize(NodeKind::DIV, index, rhs);
      } else {
        return index;
      }
    }
  }
  fn primary(&mut self, args: &mut ParseArgs) -> usize {
    if args.tokens.consume("(") {
      let index = self.expr(args);
      args.tokens.expect(")");
      return index;
    }
    self.new_node_num(args.tokens.expect_number())
  }
}
