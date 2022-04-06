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
    // println!("{:?}", token);
    let mut code: Vec<parse::Node> = vec![];
    parse::program(&mut token.into_iter().peekable(), &mut code);
    println!("{:?}", code);

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    // プロローグ
    // 変数26個分の領域を確保する
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    for line in code {
        asm::gen(&line);
        println!("  pop rax");
    }
    // エピローグ
    // 最後の式の結果がRAXに残っているのでそれが返り値になる
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}
