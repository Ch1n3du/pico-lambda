// ---------------------------- The Scanner -----------------------------
pub struct Scanner {
    source: Vec<char>,
    current: usize,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(src: &str) -> Scanner {
        Scanner {
            source: src.chars().collect::<Vec<char>>(),
            current: 0,
            tokens: Vec::new(),
        }
    }

    pub fn scan_str(src: &str) -> Vec<Token> {
        let mut scanny = Scanner::new(src);
        scanny.scan_tokens()
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.scan_token();
        }
        self.add_token(TokenType::EOF, None);
        self.tokens.to_vec()
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            'λ' | '\\' => self.add_token(TokenType::Lambda, None),
            '.' => self.add_token(TokenType::Dot, None),
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            c if c.is_alphabetic() => self.add_token(TokenType::Var, Some(c)),
            _ => (),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let chary = self.source[self.current];
        self.current += 1;
        chary
    }

    fn add_token(&mut self, ttype: TokenType, identifier: Option<char>) {
        self.tokens.push(Token { ttype, identifier })
    }
}

#[derive(Clone, PartialEq)]
pub struct Token {
    pub ttype: TokenType,
    pub identifier: Option<char>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Lambda,
    Var,
    Dot,
    LeftParen,
    RightParen,
    EOF,
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenType::*;

        match self.ttype {
            Lambda => write!(f, "'λ'"),
            LeftParen => write!(f, "'('"),
            RightParen => write!(f, "')'"),
            Dot => write!(f, "'.'"),
            Var => write!(f, "'{}'", self.identifier.unwrap()),
            EOF => write!(f, "'EOF'"),
        }
    }
}
