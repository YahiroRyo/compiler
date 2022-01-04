use crate::node::kind::NodeKind;
use crate::lvar::lvar::LVarArray;
use crate::node::node::{NodeArray};
use crate::token::token::TokenArray;

pub struct ParseArgs {
  pub tokens: TokenArray,
  pub lvars: LVarArray,
}

// stmt       = expr ";"
// expr       = assign
// assign     = equality ("=" assign)?
// equality   = relational ("==" relational | "!=" relational)*
// relational = add ("<" add | "<=" add | ">" add | ">=" add)*
// add        = mul ("+" mul | "-" mul)*
// mul        = unary ("*" unary | "/" unary)*
// unary      = ("+" | "-")? primary
// primary    = num | ident | "(" expr ")"
impl NodeArray {
  pub fn stmt(&mut self, args: &mut ParseArgs) -> usize {
    let idx = self.expr(args);
    args.tokens.expect(";");
    idx
  }
  fn expr(&mut self, args: &mut ParseArgs) -> usize {
    self.assign(args)
  }
  fn assign(&mut self, args: &mut ParseArgs) -> usize {
    let mut idx = self.equality(args);
    if args.tokens.consume("=") {
      let rhs = self.assign(args);
      idx = self.new_node_usize(NodeKind::ASSIGN, idx, rhs);
    }
    idx
  }
  fn equality(&mut self, args: &mut ParseArgs) -> usize {
    let mut idx = self.relational(args);
    loop {
      if args.tokens.consume("==") {
        let rhs = self.relational(args);
        idx = self.new_node_usize(NodeKind::EQ, idx, rhs);
      } else if args.tokens.consume("!=") {
        let rhs = self.relational(args);
        idx = self.new_node_usize(NodeKind::NE, idx, rhs);
      } else {
        return idx;
      }
    }
  }
  fn relational(&mut self, args: &mut ParseArgs) -> usize {
    let mut idx = self.add(args);

    loop {
      if args.tokens.consume("<") {
        let rhs = self.add(args);
        idx = self.new_node_usize(NodeKind::LT, idx, rhs);
      } else if args.tokens.consume("<=") {
        let rhs = self.add(args);
        idx = self.new_node_usize(NodeKind::LE, idx, rhs);
      } else if args.tokens.consume(">") {
        let lhs = self.add(args);
        idx = self.new_node_usize(NodeKind::LT, lhs, idx);
      } else if args.tokens.consume(">=") {
        let lhs = self.add(args);
        idx = self.new_node_usize(NodeKind::LE, lhs, idx);
      } else {
        return idx;
      }
    }
  }
  fn add(&mut self, args: &mut ParseArgs) -> usize {
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
    let (is_var, var_name) = args.tokens.consume_ident();
    if is_var {
      let (is_exist, offset) = args.lvars.find_lvar(&mut args.tokens);
      args.tokens.idx += 1;
      if is_exist {
        return self.new_node(NodeKind::LVAR(offset), None, None);
      } else {
        let offset;
        if args.lvars.lvars.len() == 0 {
          offset = 8;
        } else {
          offset = args.lvars.lvars[args.lvars.lvars.len() - 1].offset + 8;
        }
        args.lvars.new_lvar(var_name, offset);
        return self.new_node(NodeKind::LVAR(offset), None, None);
      }
    }

    self.new_node_num(args.tokens.expect_number())
  }
}
