mod tokenizer;

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
    if token::at_eof(token.clone()) {
        println!("  ret");
        return;
    }
    
    let mut iter = token.into_iter().peekable();

    loop {
        match iter.peek() {
            Some(val) => {
                match val.token_type {
                    token::TokenType::RESERVED => {
                        let s = val.str.clone();
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
                        println!("  {}", s);
                    },
                    token::TokenType::NUM => {
                        // Do nothing
                    },
                    token::TokenType::EOF => {
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
    // コマンド引数を受け取る
    let args: Vec<String> = std::env::args().collect();

    // コマンド引数が2つでなければエラー
    if args.len() < 2 {
        println!("引数の個数が正しくありません");
        std::process::exit(1);
    }

    let token: LinkedList<token::Token> = token::tokenize(&args[1]);
    let token = output_head_asm(token);
    output_asm(token);
}
