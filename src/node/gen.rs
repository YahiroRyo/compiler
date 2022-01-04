use crate::node::node::NodeArray;
use crate::node::kind::NodeKind as NodeKind;

impl NodeArray {
  pub fn gen(&mut self, idx: usize) {
    match self.nodes[idx].kind {
      NodeKind::NUM(n) => {
        println!("  push {}", n);
        return;
      },
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
      _ => ()
    }
    println!("  push rax");
  }
}