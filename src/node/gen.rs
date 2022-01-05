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
  pub fn gen(&mut self, idx: usize, cnt: &mut i64) {
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
        self.gen(self.nodes[idx].rhs.unwrap(), cnt);

        println!("  pop rdi");
        println!("  pop rax");
        println!("  mov [rax], rdi");
        println!("  push rdi");
        return;
      },
      NodeKind::RETURN => {
        self.gen(self.nodes[idx].lhs.unwrap(), cnt);
        println!("  pop rax");
        println!("  mov rsp, rbp");
        println!("  pop rbp");
        println!("  ret");
        return;
      },
      NodeKind::IF => {
        *cnt += 1;
        let tmp_cnt = cnt.clone();
        let if_idx = self.nodes[idx].lhs.unwrap();
        self.gen(self.nodes[if_idx].lhs.unwrap(), cnt);
        println!("  pop rax");
        println!("  cmp rax, 0");
        if self.nodes[idx].rhs != None {
          println!("  je .Lelse{}", tmp_cnt);
          self.gen(self.nodes[if_idx].rhs.unwrap(), cnt);
          println!("  jmp .Lend{}", tmp_cnt);
          println!(".Lelse{}:", tmp_cnt);
          let else_idx = self.nodes[idx].rhs.unwrap();
          self.gen(self.nodes[else_idx].lhs.unwrap(), cnt);
        } else {
          println!("  je .Lend{}", tmp_cnt);
          self.gen(self.nodes[if_idx].rhs.unwrap(), cnt);
        }
        println!(".Lend{}:", tmp_cnt);
        return;
      },
      NodeKind::WHILE => {
        *cnt += 1;
        let tmp_cnt = cnt.clone();
        println!(".Lbegin{}:", tmp_cnt);
        self.gen(self.nodes[idx].lhs.unwrap(), cnt);
        println!("  pop rax");
        println!("  cmp rax, 0");
        println!("  je .Lend{}", tmp_cnt);
        self.gen(self.nodes[idx].rhs.unwrap(), cnt);
        println!("  jmp .Lbegin{}", tmp_cnt);
        println!(".Lend{}:", tmp_cnt);
        return;
      },
      NodeKind::FOR => {
        *cnt += 1;
        let tmp_cnt = cnt.clone();
        let lhs = self.nodes[idx].lhs.unwrap();
        let rhs = self.nodes[idx].rhs.unwrap();
        if self.nodes[lhs].lhs != None {
          self.gen(self.nodes[lhs].lhs.unwrap(), cnt);
        }
        println!(".Lbegin{}:", tmp_cnt);
        if self.nodes[lhs].rhs != None {
          self.gen(self.nodes[lhs].rhs.unwrap(), cnt);
        }
        println!("  pop rax");
        println!("  cmp rax, 0");
        println!("  je .Lend{}", tmp_cnt);
        self.gen(self.nodes[rhs].rhs.unwrap(), cnt);
        if self.nodes[rhs].lhs != None {
          self.gen(self.nodes[rhs].lhs.unwrap(), cnt);
        }
        println!("  jmp .Lbegin{}", tmp_cnt);
        println!(".Lend{}:", tmp_cnt);
        return;
      },
      NodeKind::BLOCK (from, to) => {
        for index in from..to+1 {
          self.gen(index, cnt);
        }
        return;
      },
      NodeKind::ELSE => return,
      NodeKind::NONE => return,
      _ => ()
    }
    self.gen(self.nodes[idx].lhs.unwrap(), cnt);
    self.gen(self.nodes[idx].rhs.unwrap(), cnt);

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