use crate::node::kind::NodeKind;
use crate::lvar::lvar::LVarArray;
use crate::node::node::{NodeArray};
use crate::token::token::TokenArray;
use crate::node::kind::{Func, Range};

pub struct ParseArgs {
  pub tokens: TokenArray,
  pub lvars: LVarArray,
}

// stmt       = expr ";"
//            | "{" stmt* "}"
//            | "if" "(" expr ")" stmt ("else" stmt)?
//            | "while" "(" expr ")" stmt
//            | "for" "(" expr? ";" expr? ";" expr? ")" stmt
//            | "return" expr ";"
// expr       = assign
// assign     = equality ("=" assign)?
// equality   = relational ("==" relational | "!=" relational)*
// relational = add ("<" add | "<=" add | ">" add | ">=" add)*
// add        = mul ("+" mul | "-" mul)*
// mul        = unary ("*" unary | "/" unary)*
// unary      = ("+" | "-")? primary
// primary    = num
//            | ident ("(" expr?* ")")?
//            | "(" expr ")"
impl NodeArray {
  pub fn stmt(&mut self, args: &mut ParseArgs) -> usize {
    let idx;

    if args.tokens.consume("{") {
      let from: usize = self.stmt(args);
      let mut to: usize = from;
      while !args.tokens.consume("}") {
        to = self.stmt(args);
      }
      return self.new_node(NodeKind::BLOCK(Range{
        from,
        to
      }), None, None);
    }

    if args.tokens.consume("if") {
      args.tokens.expect("(");
      let if_lhs = self.expr(args);
      args.tokens.expect(")");
      let if_rhs = self.stmt(args);
      let lhs = self.new_node_usize(NodeKind::NONE, if_lhs, if_rhs);
      if args.tokens.consume("else") {
        let else_lhs = self.stmt(args);
        let rhs = self.new_node_usize(NodeKind::ELSE, else_lhs, 0);
        return self.new_node_usize(NodeKind::IF, lhs, rhs);
      }
      return self.new_node(NodeKind::IF, Some(lhs), None);
    }

    if args.tokens.consume("while") {
      args.tokens.expect("(");
      let lhs = self.expr(args);
      args.tokens.expect(")");
      let rhs = self.stmt(args);
      return self.new_node_usize(NodeKind::WHILE, lhs, rhs);
    }

    if args.tokens.consume("while") {
      args.tokens.expect("(");
      let lhs = self.expr(args);
      args.tokens.expect(")");
      let rhs = self.stmt(args);
      return self.new_node_usize(NodeKind::WHILE, lhs, rhs);
    }

    if args.tokens.consume("for") {
      args.tokens.expect("(");
      let mut lhs: Option<usize> = None;
      let mut mhs: Option<usize> = None;
      let mut rhs: Option<usize> = None;
      let stmt: usize;
      if !args.tokens.consume(";") {
        lhs = Some(self.expr(args));
        args.tokens.idx += 1;
      }
      if !args.tokens.consume(";") {
        mhs = Some(self.expr(args));
        args.tokens.idx += 1;
      }
      if !args.tokens.consume(")") {
        rhs = Some(self.expr(args));
      }
      args.tokens.expect(")");
      stmt = self.stmt(args);

      let while_lhs = self.new_node(NodeKind::NONE, lhs, mhs);
      let while_rhs = self.new_node(NodeKind::NONE, rhs, Some(stmt));
      return self.new_node_usize(NodeKind::FOR, while_lhs, while_rhs);
    }
    
    if args.tokens.consume_return() {
      let lhs = self.expr(args);
      idx = self.new_node(NodeKind::RETURN, Some(lhs), None);
    } else {
      idx = self.expr(args);
    }
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
      // function
      if args.tokens.consume("(") {
        let from = self.expr(args);
        let mut to = from;
        args.tokens.consume(",");
        while !args.tokens.consume(")") {
          to = self.expr(args);
          args.tokens.consume(",");
        }
        return self.new_node(NodeKind::FUNC(Func {
          range: Range {
            from,
            to,
          },
          name: var_name,
        }), None, None);
      } else {
      // var
        args.tokens.idx -= 1;
        let (is_exist, offset) = args.lvars.find_lvar(&mut args.tokens);
        args.tokens.idx += 1;
        if is_exist {
          return self.new_node(NodeKind::LVAR(offset), None, None);
        } else {
          args.tokens.expect("=");
          args.tokens.idx -= 1;

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
    }

    self.new_node_num(args.tokens.expect_number())
  }
}
