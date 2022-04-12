use crate::parse;
use crate::parse::NodeType;

pub fn gen_lval(node: &parse::Node) {
    if !matches!(node.node_type, NodeType::LVAR) {
        panic!("代入の左辺値が変数ではありません");
    }

    println!("  mov rax, rbp");
    println!("  sub rax, {}", node.offset);
    println!("  push rax");
}

pub fn gen(node: &parse::Node) {
    match node.node_type {
        NodeType::NUM => {
            println!("  push {}", node.val);
            return;
        }
        NodeType::LVAR => {
            gen_lval(node);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            return;
        }
        _ => {}
    }

    let lhs = &**node.lhs.as_ref().unwrap();
    let rhs = &**node.rhs.as_ref().unwrap();

    match node.node_type {
        NodeType::ASSIGN => {
            gen_lval(lhs);
            gen(rhs);
            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
            return;
        }
        _ => {}
    }

    gen(lhs);
    gen(rhs);

    println!("  pop rdi");
    println!("  pop rax");

    match node.node_type {
        NodeType::ADD => {
            println!("  add rax, rdi");
        }
        NodeType::SUB => {
            println!("  sub rax, rdi");
        }
        NodeType::MUL => {
            println!("  imul rax, rdi");
        }
        NodeType::DIV => {
            println!("  cqo");
            println!("  idiv rdi");
        }
        NodeType::EQ => {
            println!("  cmp rax, rdi");
            println!("  sete al");
            println!("  movzb rax, al");
        }
        NodeType::NE => {
            println!("  cmp rax, rdi");
            println!("  setne al");
            println!("  movzb rax, al");
        }
        NodeType::LT => {
            println!("  cmp rax, rdi");
            println!("  setl al");
            println!("  movzb rax, al");
        }
        NodeType::LE => {
            println!("  cmp rax, rdi");
            println!("  setle al");
            println!("  movzb rax, al");
        }
        _ => {}
    }

    println!("  push rax");
}
