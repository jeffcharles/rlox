pub struct Scanner<'a> {
    start: &'a str,
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
    pub str: &'a str,
    pub line: u32,
}

impl<'a> Default for Token<'a> {
    fn default() -> Self {
        Self {
            ty: TokenType::Error,
            str: Default::default(),
            line: Default::default(),
        }
    }
}

impl<'a> Token<'a> {
    pub fn new(ty: TokenType, str: &'a str, line: u32) -> Token<'a> {
        Token { ty, str, line }
    }

    pub fn error(message: &'static str, line: u32) -> Token<'a> {
        Token {
            ty: TokenType::Error,
            str: message,
            line,
        }
    }
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            start: source,
            current: 0,
            line: 1,
        }
    }

    fn make_token(&self, ty: TokenType) -> Token<'a> {
        Token::new(ty, &self.start[..self.current], self.line)
    }

    fn advance(&mut self) -> Option<char> {
        self.start[self.current..].chars().next().map(|c| {
            self.current += c.len_utf8();
            c
        })
    }

    fn matches(&mut self, expected: char) -> bool {
        match self.peek() {
            Some(c) if c == expected => {
                self.current += c.len_utf8();
                true
            }
            _ => false,
        }
    }

    fn peek(&self) -> Option<char> {
        self.start[self.current..].chars().next()
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
        let mut chars = self.start[self.current..].chars();
        chars.next();
        chars.next()
    }

    fn string(&mut self) -> Token<'a> {
        while self.peek().map_or(false, |c| c != '"') {
            if self.peek().unwrap() == '\n' {
                self.line += 1;
                self.advance();
            }
        }

        // The closing quote
        if let None = self.advance() {
            return Token::error("Unterminated string", self.line);
        }
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
        match &self.start[..self.current] {
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
        self.skip_whitespace();
        self.start = &self.start[self.current..];
        self.current = 0;

        let c = self.advance();
        let c = if let Some(c) = c {
            c
        } else {
            return Some(self.make_token(TokenType::EOF));
        };
        if Self::is_alpha(c) {
            return Some(self.identifier());
        }
        if Self::is_digit(c) {
            return Some(self.number());
        }

        Some(match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                let ty = if self.matches('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.make_token(ty)
            }
            '=' => {
                let ty = if self.matches('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.make_token(ty)
            }
            '<' => {
                let ty = if self.matches('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.make_token(ty)
            }
            '>' => {
                let ty = if self.matches('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.make_token(ty)
            }
            '\"' => self.string(),
            _ => Token::error("Unexpected character.", self.line),
        })
    }
}
