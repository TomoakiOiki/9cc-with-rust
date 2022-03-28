mod tokenizer;
mod utils;

use std::{collections::LinkedList, process};

use crate::tokenizer::token;

fn output_asm(input: String, token: LinkedList<token::Token>){
    let mut iter = token.into_iter().peekable();
    let mut asm: Vec<String> = Vec::new();
    asm.push(".intel_syntax noprefix".to_string());
    asm.push(".global main".to_string());
    asm.push("main:".to_string());

    loop {
        match iter.peek() {
            Some(val) => {
                match val.token_type {
                    token::TokenType::RESERVED => {
                        let s: &str = &val.str.clone();
                        match s {
                            "+" => {
                                iter.next();
                                let token = *iter.peek().as_ref().unwrap();
                                let num = token::expect_number(input.clone(), token);
                                asm.push(format!("  add rax, {}", num));
                            },
                            "-" => {
                                iter.next();
                                let num = iter.peek().unwrap().val.clone();
                                asm.push(format!("  sub rax, {}", num));
                            },
                            _ => {
                                asm.push("Unexpected token".to_string());
                                process::exit(1);
                            }
                        }
                    },
                    token::TokenType::NUM => {
                        let num = token::expect_number(input.clone(), val);
                        asm.push(format!("  mov rax, {}", num));
                    },
                    token::TokenType::EOF => {
                        asm.push("  ret".to_string());
                        break;
                    }
                }
                iter.next();
            }
            None => {
                break;
            }
        }
    }

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
