use crate::ast::AST;
use crate::scanner::{Scanner, Token, TokenType};

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn parse_str(src: &str) -> ParseResult<AST> {
        let tokens = Scanner::scan_str(src);
        Parser::parse_tokens(tokens)
    }

    pub fn parse_tokens(tokens: Vec<Token>) -> ParseResult<AST> {
        let mut parser = Parser { current: 0, tokens };
        parser.parse_term()
    }

    // <term> ::= grouping
    fn parse_term(&mut self) -> ParseResult<AST> {
        self.parse_grouping()
    }

    // <grouping> ::= "(" <grouping> ")" | <application>
    fn parse_grouping(&mut self) -> ParseResult<AST> {
        if self.matches(TokenType::LeftParen) {
            self.consume(TokenType::LeftParen)?;
            let term = self.parse_grouping();
            self.consume(TokenType::RightParen)?;

            term
        } else {
            self.parse_application()
        }
    }

    // <application> ::= <atom>  <term>?
    fn parse_application(&mut self) -> ParseResult<AST> {
        let func = self.parse_atom()?;

        if self.matches(TokenType::EOF) || self.matches(TokenType::RightParen) {
            Ok(func)
        } else {
            let arg = self.parse_grouping()?;
            Ok(AST::Application {
                function: Box::new(func),
                argument: Box::new(arg),
            })
        }
    }

    // <atom> ::= <function> | <variable>
    fn parse_atom(&mut self) -> ParseResult<AST> {
        if self.matches(TokenType::Lambda) {
            self.parse_function()
        } else if self.matches(TokenType::Var) {
            self.parse_variable()
        } else {
            Err(ParserError::ExpectedAtom)
        }
    }

    // <function> ::= λ<parameter>.<body>
    fn parse_function(&mut self) -> ParseResult<AST> {
        self.consume(TokenType::Lambda)?;
        let parameter = self.consume(TokenType::Var)?.identifier.unwrap();
        self.consume(TokenType::Dot)?;
        let body = self.parse_grouping()?;

        Ok(AST::Function {
            parameter,
            body: Box::new(body),
        })
    }

    // <variable>
    fn parse_variable(&mut self) -> ParseResult<AST> {
        let tok = self.consume(TokenType::Var)?;
        let identifier = tok.identifier.unwrap();

        Ok(AST::Variable(identifier))
    }

    fn matches(&self, ttype: TokenType) -> bool {
        self.tokens[self.current].ttype == ttype
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.current].clone();
        self.current += 1;
        token
    }

    fn consume(&mut self, ttype: TokenType) -> ParseResult<Token> {
        if self.matches(ttype) {
            Ok(self.advance())
        } else {
            Err(ParserError::Expected(ttype))
        }
    }
}

#[derive(Debug)]
pub enum ParserError {
    Expected(TokenType),
    ExpectedAtom,
}

pub type ParseResult<T> = Result<T, ParserError>;
