use std::process::exit;

pub fn error_at(input: String, pos: usize, msg: String){
    println!("{}", input);
    for _ in 0..pos {
        print!(" ");
    }
    println!("^ {}",msg);
    exit(1);
}
