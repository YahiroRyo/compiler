use crate::node::kind::NodeKind as NodeKind;
use crate::node::node::{NodeArray};
use crate::token::token::TokenArray;

pub struct ParseArgs {
  pub tokens: TokenArray
}

// expr       = equality
// equality   = relational ("==" relational | "!=" relational)*
// relational = add ("<" add | "<=" add | ">" add | ">=" add)*
// add        = mul ("+" mul | "-" mul)*
// mul        = unary ("*" unary | "/" unary)*
// unary      = ("+" | "-")? primary
// primary    = num | "(" expr ")"
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
    let mut index = self.unary(args);
    loop {
      if args.tokens.consume("*") {
        let rhs = self.unary(args);
        index = self.new_node_usize(NodeKind::MUL, index, rhs);
      } else if args.tokens.consume("/") {
        let rhs = self.unary(args);
        index = self.new_node_usize(NodeKind::DIV, index, rhs);
      } else {
        return index;
      }
    }
  }
  fn unary(&mut self, args: &mut ParseArgs) -> usize {
    if args.tokens.consume("+") {
      return self.primary(args);
    } else if args.tokens.consume("-") {
      let lhs = self.new_node_num(0);
      let rhs = self.primary(args);
      return self.new_node_usize(NodeKind::SUB, lhs, rhs);
    }
    self.primary(args)
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
