use crate::parse;

pub fn gen(node: &parse::Node) {
    if matches!(node.node_type, parse::NodeType::ND_NUM) {
        println!("  push {}", node.val);
        return;
    }

    let lhs = &**node.lhs.as_ref().unwrap();
    let rhs = &**node.rhs.as_ref().unwrap();

    gen(lhs);
    gen(rhs);

    println!("  pop rdi");
    println!("  pop rax");

    match node.node_type {
        parse::NodeType::ND_ADD => {
            println!("  add rax, rdi");
        }
        parse::NodeType::ND_SUB => {
            println!("  sub rax, rdi");
        }
        parse::NodeType::ND_MUL => {
            println!("  imul rax, rdi");
        }
        parse::NodeType::ND_DIV => {
            println!("  cqo");
            println!("  idiv rdi");
        }
        parse::NodeType::ND_EQ => {
            println!("  cmp rax, rdi");
            println!("  sete al");
            println!("  movzb rax, al");
        }
        parse::NodeType::ND_NE => {
            println!("  cmp rax, rdi");
            println!("  setne al");
            println!("  movzb rax, al");
        }
        parse::NodeType::ND_LT => {
            println!("  cmp rax, rdi");
            println!("  setl al");
            println!("  movzb rax, al");
        }
        parse::NodeType::ND_LE => {
            println!("  cmp rax, rdi");
            println!("  setle al");
            println!("  movzb rax, al");
        }
        _ => {}
    }

    println!("  push rax");
}
