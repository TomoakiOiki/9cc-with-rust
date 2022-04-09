mod asm;
mod error;
mod parse;
mod token;

use std::cell::RefCell;
use std::collections::LinkedList;

thread_local! {
    static TOKEN: RefCell<LinkedList<token::Token>> = {
        let v: LinkedList<token::Token> = LinkedList::new();
        RefCell::new(v)
    };
    static CODE: RefCell<Vec<parse::Node>> = {
        let v: Vec<parse::Node> = Vec::new();
        RefCell::new(v)
    };
    static LOCALS: RefCell<Vec<token::LVar>> = {
        let v: Vec<token::LVar> = Vec::new();
        RefCell::new(v)
    };
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("引数の個数が正しくありません");
        std::process::exit(1);
    }
    let input = args[1].clone();
    token::tokenize(&input);
    // println!("{:?}", token);
    parse::program();
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    // プロローグ
    // 変数26個分の領域を確保する
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    CODE.with(|c| {
        for line in c.borrow_mut().iter() {
            asm::gen(&line);
            println!("  pop rax");
        }
    });
    // エピローグ
    // 最後の式の結果がRAXに残っているのでそれが返り値になる
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}
