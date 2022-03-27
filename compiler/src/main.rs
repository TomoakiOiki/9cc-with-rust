mod utils;
mod tokenizer;

use tokenizer::token;
use tokenizer::token::TokenType;
use utils::linked_list;


fn main() {
    // コマンド引数を受け取る
    let args: Vec<String> = std::env::args().collect();

    // コマンド引数が2つでなければエラー
    if args.len() < 2 {
        println!("引数の個数が正しくありません");
        std::process::exit(1);
    }

    let token_list: linked_list::LinkedList<token::Token> = token::tokenize(&args[1]);
    let mut current_token = token_list.get_head();

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    loop {
        println!("{:?}", current_token);
        match current_token {
            Some(ref mut token) => {
                let token_type = token.get_body().token_type;
                match token_type {
                    TokenType::RESERVED => {
                        let operator = token.get_body().value.as_ref().unwrap()[0];
                        let next_num = token.read_next().unwrap().get_body().num.unwrap();
                        match operator {
                            '+' => {
                                println!("  add rax, {}", next_num);
                            },
                            '-' => {
                                println!("  sub rax, {}", next_num);
                            },
                            _ => {
                                println!("Unexpected token");
                                std::process::exit(1);
                            }
                        }
                    },
                    TokenType::NUM => {
                        let num = token.get_body().num.unwrap();
                        println!("  mov rax, {}", num);
                    },
                    TokenType::WHITESPACE | TokenType::HEAD => {
                        // 何もしない
                    },
                    TokenType::EOF => {
                        break;
                    }
                }
                let next_token = token.read_next();
                if next_token.is_none() {
                    break;
                }
                current_token = next_token;
            }
            None => {
                break;
            }
        }
    }
    println!("  ret");
}
