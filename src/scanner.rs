use std::str::Chars;

pub struct Scanner<'a> {
    start: Chars<'a>,
    current: usize,
    line: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Error,
    EOF,
}

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub ty: TokenType,
    pub start: &'a str,
    pub line: u32,
}

impl<'a> Default for Token<'a> {
    fn default() -> Self {
        Self {
            ty: TokenType::Error,
            start: Default::default(),
            line: Default::default(),
        }
    }
}

impl<'a> Token<'a> {
    pub fn new(ty: TokenType, start: &'a str, line: u32) -> Token<'a> {
        Token { ty, start, line }
    }

    pub fn error(message: &'static str, line: u32) -> Token<'a> {
        Token {
            ty: TokenType::Error,
            start: message,
            line,
        }
    }
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            start: source.chars(),
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token<'b>(&'b mut self) -> Token<'a> {
        self.skip_whitespace();
        self.start = self.start.as_str()[self.current..].chars();
        self.current = 0;

        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        let c = self.advance();
        if Self::is_alpha(c) {
            return self.identifier();
        }
        if Self::is_digit(c) {
            return self.number();
        }

        match c {
            '(' => return self.make_token(TokenType::LeftParen),
            ')' => return self.make_token(TokenType::RightParen),
            '{' => return self.make_token(TokenType::LeftBrace),
            '}' => return self.make_token(TokenType::RightBrace),
            ';' => return self.make_token(TokenType::Semicolon),
            ',' => return self.make_token(TokenType::Comma),
            '.' => return self.make_token(TokenType::Dot),
            '-' => return self.make_token(TokenType::Minus),
            '+' => return self.make_token(TokenType::Plus),
            '/' => return self.make_token(TokenType::Slash),
            '*' => return self.make_token(TokenType::Star),
            '!' => {
                let ty = if self.matches('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                return self.make_token(ty);
            }
            '=' => {
                let ty = if self.matches('=') {
                    eprintln!("Matches");
                    TokenType::EqualEqual
                } else {
                    eprintln!("Does not match");
                    TokenType::Equal
                };
                return self.make_token(ty);
            }
            '<' => {
                let ty = if self.matches('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                return self.make_token(ty);
            }
            '>' => {
                let ty = if self.matches('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                return self.make_token(ty);
            }
            '\"' => return self.string(),
            _ => (),
        }
        Token::error("Unexpected character.", self.line)
    }

    fn make_token(&self, ty: TokenType) -> Token<'a> {
        Token::new(ty, &self.start.as_str()[..self.current], self.line)
    }

    fn is_at_end(&self) -> bool {
        self.peek().is_none()
    }

    fn advance(&mut self) -> char {
        let mut char_indices = self.start.as_str()[self.current..].char_indices();
        let (_, c) = char_indices.next().unwrap();
        let i = char_indices.next().map_or_else(|| c.len_utf8(), |(i, _)| i);
        self.current += i;
        c
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.peek().map_or(false, |c| c != expected) {
            return false;
        }
        let (i, _) = self.start.as_str()[self.current..]
            .char_indices()
            .next()
            .unwrap();
        self.current += i;
        true
    }

    fn peek(&self) -> Option<char> {
        self.start.as_str()[self.current..].chars().next()
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some(c) => match c {
                    ' ' | '\r' | '\t' => {
                        self.advance();
                    }
                    '\n' => {
                        self.line += 1;
                        self.advance();
                    }
                    '/' if self.peek_next() == Some('/') => {
                        while self.peek().map_or(false, |c| c != '\n') {
                            self.advance();
                        }
                    }
                    _ => return,
                },
                None => return,
            };
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        let mut chars = self.start.as_str()[self.current..].chars();
        chars.next().unwrap();
        chars.next()
    }

    fn string(&mut self) -> Token<'a> {
        while self.peek().map_or(false, |c| c != '"') {
            if self.peek().unwrap() == '\n' {
                self.line += 1;
                self.advance();
            }
        }

        if self.is_at_end() {
            return Token::error("Unterminated string", self.line);
        }

        // The closing quote
        self.advance();
        self.make_token(TokenType::String)
    }

    fn is_digit(c: char) -> bool {
        c.is_digit(10)
    }

    fn number(&mut self) -> Token<'a> {
        while self.peek().map_or(false, Self::is_digit) {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == Some('.') && self.peek_next().map_or(false, Self::is_digit) {
            // Consume the "."
            self.advance();

            while Self::is_digit(self.peek().unwrap()) {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn is_alpha(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn identifier(&mut self) -> Token<'a> {
        while self
            .peek()
            .map_or(false, |c| Self::is_alpha(c) || Self::is_digit(c))
        {
            self.advance();
        }
        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        match &self.start.as_str()[..self.current] {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.scan_token();
        Some(token)
    }
}
