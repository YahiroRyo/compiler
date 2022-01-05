use crate::build_in_funcs::func::BuildInFuncs;

impl BuildInFuncs {
  pub fn print() {
    Self::to_string();
    Self::write();
    println!("print:");
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");
    println!("  mov rax, rdi");
    println!("  mov rdi, rbp");
    println!("  sub rdi, 8");
    println!("  mov [rdi], rax");
    println!("  call to_string");
    println!("  mov rdi, rax");
    println!(".Lprint:");
    println!("  call write");
    println!("  mov rdi, rax");
    println!("  add rdi, 8");
    println!("  mov rbx, [rdi]");
    println!("  cmp rbx, 0x00");
    println!("  jne .Lprint");
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
  }
  pub fn write() {
    println!("write:");
    println!("  mov rsi, rdi");
    println!("  mov rax, 1");
    println!("  mov rdi, 1");
    println!("  mov rdx, 1");
    println!("  syscall");
    println!("  mov rax, rsi");
    println!("  ret");
  }
  pub fn to_string() {
    println!("to_string:");
    println!("  mov rax, [rdi]");
    println!("  mov rbx, 0x00");
    println!("  mov [rdi], rbx");
    println!("  sub rdi, 8");
    println!("  mov rbx, 0x30");
    println!("  cmp rax, 10");
    println!("  jb .Lendto_string");
    println!("  mov rcx, 10");
    println!("  mov rdx, 0");
    println!(".Lto_string:");
    println!("  div rcx");
    println!("  add rdx, rbx");
    println!("  mov [rdi], rdx");
    println!("  sub rdi, 8");
    println!("  mov rdx, 0");
    println!("  cmp rax, 10");
    println!("  jae .Lto_string");
    println!(".Lendto_string:");
    println!("  add rax, rbx");
    println!("  mov [rdi], rax");
    println!("  mov rax, rdi");
    println!("  ret");
  }
}