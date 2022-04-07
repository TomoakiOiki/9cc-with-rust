use crate::parse;
use crate::parse::NodeType;

pub fn gen_lval(node: &parse::Node) {
    if !matches!(node.node_type, NodeType::ND_LVAR) {
        panic!("代入の左辺値が変数ではありません");
    }

    let offset = (node.name.as_bytes()[0] - 'a' as u8 + 1) * 8;

    println!("  mov rax, rbp");
    println!("  sub rax, {}", offset);
    println!("  push rax");
}

pub fn gen(node: &parse::Node) {
    let lhs = &**node.lhs.as_ref().unwrap();
    let rhs = &**node.rhs.as_ref().unwrap();

    match node.node_type {
        NodeType::ND_NUM => {
            println!("  push {}", node.val);
            return;
        }
        NodeType::ND_LVAR => {
            gen_lval(node);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            return;
        }
        NodeType::ND_ASSIGN => {
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
        NodeType::ND_ADD => {
            println!("  add rax, rdi");
        }
        NodeType::ND_SUB => {
            println!("  sub rax, rdi");
        }
        NodeType::ND_MUL => {
            println!("  imul rax, rdi");
        }
        NodeType::ND_DIV => {
            println!("  cqo");
            println!("  idiv rdi");
        }
        NodeType::ND_EQ => {
            println!("  cmp rax, rdi");
            println!("  sete al");
            println!("  movzb rax, al");
        }
        NodeType::ND_NE => {
            println!("  cmp rax, rdi");
            println!("  setne al");
            println!("  movzb rax, al");
        }
        NodeType::ND_LT => {
            println!("  cmp rax, rdi");
            println!("  setl al");
            println!("  movzb rax, al");
        }
        NodeType::ND_LE => {
            println!("  cmp rax, rdi");
            println!("  setle al");
            println!("  movzb rax, al");
        }
        _ => {}
    }

    println!("  push rax");
}
