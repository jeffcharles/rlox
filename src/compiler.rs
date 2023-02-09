use crate::{
    chunk::Chunk,
    scanner::{Scanner, Token, TokenType},
};
use anyhow::{bail, Result};

struct Parser<'a> {
    scanner: Scanner<'a>,
    current: Option<Token<'a>>,
    previous: Option<Token<'a>>,
    had_error: bool,
    panic_mode: bool,
}

impl<'a> Parser<'a> {
    fn new(scanner: Scanner) -> Parser {
        Parser {
            scanner,
            current: None,
            previous: None,
            had_error: false,
            panic_mode: false,
        }
    }

    fn advance(&mut self) {
        self.previous = self.current;

        loop {
            self.current = Some(self.scanner.scan_token());
            if self.current.unwrap().ty != TokenType::Error {
                break;
            }
            self.error_at_current(self.current.unwrap().start);
        }
    }

    fn consume(&mut self, ty: TokenType, message: &str) {
        if self.current.unwrap().ty == ty {
            self.advance();
        } else {
            self.error_at_current(message);
        }
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.current.unwrap(), message);
    }

    fn error(&mut self, message: &str) {
        self.error_at(&self.previous.unwrap(), message);
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        eprint!("[line {}] Error", token.line);

        match token.ty {
            TokenType::EOF => eprint!(" at end"),
            TokenType::Error => (),
            _ => eprint!(" at '{}.{}", token.start.len(), token.start),
        }

        eprintln!(": {message}");
        self.had_error = true;
    }
}

pub fn compile(source: &str) -> Result<Chunk> {
    let scanner = Scanner::new(&source);
    let mut parser = Parser::new(scanner);

    parser.had_error = false;
    parser.panic_mode = false;

    parser.advance();
    // expression();
    // consume(TokenType::EOF, "Expect end of expression");
    if parser.had_error {
        bail!("Parser had error");
    } else {
        todo!()
    }
}
