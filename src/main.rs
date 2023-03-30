#![allow(warnings)]

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::process::exit;

use clap::{Parser, ValueEnum};

mod lex;
mod compiler;

use CompileError::*;

fn main() {
    // let args = Args::parse();
    // let config = Config { mode: args.mode };
    // for file in args.files {
    //     if let Err(err) = compile(&file, &config) {
    //         fail(format!("Fatal error occurred when compiling {file}:").as_str());
    //         match &err {
    //             CouldNotOpenFile(file) => {
    //                 eprintln!("Could not ")
    //             }
    //         }
    //     }
    // }
    // let num_files = args.files.len();
    // println!("Successfully compiled {num_files} files.")
}

fn fail(message: &str) {
    eprintln!("Fatal error: {message}");
    eprintln!("Compilation terminated.");
    exit(1);
}

// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     #[arg(short, long, default_value_t = Mode::Lex)]
//     mode: Mode,
//     files: Vec<String>,
// }

struct Config {
    mode: Mode,
}

#[derive(Clone, Debug, ValueEnum)]
enum Mode {
    Lex,
    Compile,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Mode::Lex => "Lex",
            Mode::Compile => "Compile"
        };
        write!(f, "{repr}")
    }
}

// fn compile<'a>(file: &'a String, config: &Config) -> Result<(), &'a CompileError<'a>> {
//     let text = read_file(file)
//         .map_err(|_| &CouldNotOpenFile(file))?;
//     match lex::lex(&text) {
//         Ok(tokens) => {
//             for token in tokens {
//                 println!("{:?}", token);
//             }
//         },
//         Err(err) => {
//             return Err(&LexError(&err.to_string()))
//         }
//     }
//     Ok(())
// }

enum CompileError<'a> {
    CouldNotOpenFile(&'a String),
    LexError(&'a String)
}

fn read_file(path: &str) -> io::Result<String> {
    let path = Path::new(path);
    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s).expect("error dumping file to string");
    Ok(s)
}