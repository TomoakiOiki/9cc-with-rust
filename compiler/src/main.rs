mod token;
mod error;

use std::{collections::LinkedList};

use token::at_eof;


fn output_asm(input: String, token: LinkedList<token::Token>){
    let mut iter = token.into_iter().peekable();
    let mut asm: Vec<String> = Vec::new();
    asm.push(".intel_syntax noprefix".to_string());
    asm.push(".global main".to_string());
    asm.push("main:".to_string());
    asm.push(format!("  mov rax, {}", token::expect_number(input.clone(), &mut iter)));

    while !at_eof(&mut iter) {
        if token::consume('+',&mut iter) {
            let num = token::expect_number(input.clone(), &mut iter);
            asm.push(format!("  add rax, {}", num));
            continue;
        }

        token::expect('-', &mut iter);
        asm.push(format!("  sub rax, {}", token::expect_number(input.clone(), &mut iter)));
    }

    asm.push("  ret".to_string());

    for line in asm {
        println!("{}", line);
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("引数の個数が正しくありません");
        std::process::exit(1);
    }
    let input = args[1].clone();
    let token: LinkedList<token::Token> = token::tokenize(&input);
    output_asm(input, token);
}
