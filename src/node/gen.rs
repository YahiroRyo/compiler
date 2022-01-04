use crate::node::node::NodeArray as NodeArray;
use crate::node::kind::NodeKind as NodeKind;
use crate::strlib::strl::error_msg;

impl NodeArray {
  fn gen_lval(&mut self, idx: usize) {
    match self.nodes[idx].kind {
      NodeKind::LVAR(offset) => {
        println!("  mov rax, rbp");
        println!("  sub rax, {}", offset);
        println!("  push rax");
      },
      _ => error_msg("代入の左辺値が変数ではありません")
    }
  }
  pub fn gen(&mut self, idx: usize) {
    match self.nodes[idx].kind {
      NodeKind::NUM(n) => {
        println!("  push {}", n);
        return;
      },
      NodeKind::LVAR (_) => {
        self.gen_lval(idx);
        println!("  pop rax");
        println!("  mov rax, [rax]");
        println!("  push rax");
        return;
      },
      NodeKind::ASSIGN => {
        self.gen_lval(self.nodes[idx].lhs.unwrap());
        self.gen(self.nodes[idx].rhs.unwrap());

        println!("  pop rdi");
        println!("  pop rax");
        println!("  mov [rax], rdi");
        println!("  push rdi");
        return;
      }
      _ => ()
    }
    self.gen(self.nodes[idx].lhs.unwrap());
    self.gen(self.nodes[idx].rhs.unwrap());

    println!("  pop rdi");
    println!("  pop rax");

    match self.nodes[idx].kind {
      NodeKind::ADD => println!("  add rax, rdi"),
      NodeKind::SUB => println!("  sub rax, rdi"),
      NodeKind::MUL => println!("  imul rax, rdi"),
      NodeKind::DIV => { println!("  cqo"); println!("  idiv rdi") },
      NodeKind::EQ => {
        println!("  cmp rax, rdi");
        println!("  sete al");
        println!("  movzb rax, al");
      },
      NodeKind::NE => {
        println!("  cmp rax, rdi");
        println!("  setne al");
        println!("  movzb rax, al");
      },
      NodeKind::LT => {
        println!("  cmp rax, rdi");
        println!("  setl al");
        println!("  movzb rax, al");
      },
      NodeKind::LE => {
        println!("  cmp rax, rdi");
        println!("  setle al");
        println!("  movzb rax, al");
      },
      _ => ()
    }
    println!("  push rax");
  }
}