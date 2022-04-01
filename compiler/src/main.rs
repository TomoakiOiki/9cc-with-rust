mod asm;
mod error;
mod parse;
mod token;

use std::collections::LinkedList;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("引数の個数が正しくありません");
        std::process::exit(1);
    }
    let input = args[1].clone();
    let token: LinkedList<token::Token> = token::tokenize(&input);
    let mut node = parse::expr(&mut token.into_iter().peekable());

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    asm::gen(&mut node);
    println!("  pop rax");
    println!("  ret");
}
