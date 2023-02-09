use crate::scanner::{Scanner, TokenType};

fn compile(source: &str) {
    let mut scanner = Scanner::new(&source);
    let mut line = 0;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            print!("{:4}", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }
        println!("{:?} '{}.{}", token.ty, token.start.len(), token.start);

        if matches!(token.ty, TokenType::EOF) {
            break;
        }
    }
}
