mod chunk;
mod compiler;
mod scanner;
mod value;
mod vm;

use std::{
    env,
    fs::File,
    io::{self, BufRead, Read},
    process, str,
};

use chunk::{Chunk, OpCode};
use vm::{InterpretResult, VM};

#[macro_use]
extern crate num_derive;

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[..] {
        [] => repl(),
        [path] => run_file(path),
        _ => {
            eprintln!("Usage: rlox [path]");
            process::exit(64);
        }
    }
}

fn repl() {
    loop {
        print!("> ");

        if let Some(Ok(line)) = io::stdin().lock().lines().next() {
            let _ = VM::interpret(&line);
        } else {
            println!("");
            break;
        }
    }
}

fn run_file(path: &str) {
    let mut f = File::open(path).unwrap_or_else(|_| {
        eprintln!("Could not open file {}.", path);
        process::exit(74);
    });
    let mut buffer = vec![];
    f.read_to_end(&mut buffer).unwrap_or_else(|_| {
        eprintln!("Could not read file {}", path);
        process::exit(74);
    });
    let source = str::from_utf8(&buffer).unwrap_or_else(|_| {
        eprintln!("Invalid source string");
        process::exit(74);
    });

    let result = VM::interpret(source);
    match result {
        InterpretResult::CompileError => process::exit(65),
        InterpretResult::RuntimeError => process::exit(70),
        InterpretResult::Ok => (),
    }
}
