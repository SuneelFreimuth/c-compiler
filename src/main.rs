use std::fs::File;
use std::io::Read;
use std::path::Path;

mod lex;

fn main() {
    let return_2 = read_file("return_2.c");
    match lex::lex(&return_2) {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        },
        Err(err) => {
            println!("Error lexing file: {}", err);
        }
    }
}

fn read_file(path: &str) -> String {
    let path = Path::new(path);
    let mut file = File::open(path).expect("could not open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("error dumping file to string");
    s
}