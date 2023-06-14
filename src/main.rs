mod chunk;
mod compiler;
mod scanner;
mod value;
mod vm;

use anyhow::Result;
use std::{
    env,
    fs::File,
    io::{self, BufRead, Read, Write},
    process, str,
};

use chunk::{Chunk, OpCode};
use vm::InterpretResult;

#[macro_use]
extern crate num_derive;

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[..] {
        [_] => repl().unwrap(),
        [_, path] => run_file(path),
        _ => {
            eprintln!("Usage: rlox [path]");
            process::exit(64);
        }
    }
}

fn repl() -> Result<()> {
    loop {
        print!("> ");
        io::stdout().flush()?;

        if let Some(Ok(line)) = io::stdin().lock().lines().next() {
            match vm::interpret(&line) {
                InterpretResult::CompileError => eprintln!("Compile error"),
                InterpretResult::RuntimeError => eprintln!("Runtime error"),
                InterpretResult::Ok => (),
            }
        } else {
            println!("");
            break;
        }
    }
    Ok(())
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

    let result = vm::interpret(source);
    match result {
        InterpretResult::CompileError => process::exit(65),
        InterpretResult::RuntimeError => process::exit(70),
        InterpretResult::Ok => (),
    }
}
