mod tokenizer;
mod utils;

use std::{collections::LinkedList, process};

use crate::tokenizer::token;

fn output_head_asm(token: LinkedList<token::Token>) -> LinkedList<token::Token> {
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    let (num, new_token) = token::expect_number(token);
    println!("  mov rax, {}", num);
    new_token
}

fn output_asm(token: LinkedList<token::Token>){
    let mut iter = token.into_iter().peekable();

    loop {
        match iter.peek() {
            Some(val) => {
                match val.token_type {
                    token::TokenType::RESERVED => {
                        let s: &str = &val.str.clone();
                        match s {
                            "+" => {
                                iter.next();
                                let num = iter.peek().unwrap().val;
                                println!("  add rax, {}", num);
                            },
                            "-" => {
                                iter.next();
                                let num = iter.peek().unwrap().val;
                                println!("  sub rax, {}",num);
                            },
                            _ => {
                                println!("Unexpected token");
                                process::exit(1);
                            }
                        }
                    },
                    token::TokenType::NUM => {
                        // Do nothing
                    },
                    token::TokenType::EOF => {
                        println!("  ret");
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
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("引数の個数が正しくありません");
        std::process::exit(1);
    }

    let token: LinkedList<token::Token> = token::tokenize(&args[1]);
    let token = output_head_asm(token);
    output_asm(token);
}
