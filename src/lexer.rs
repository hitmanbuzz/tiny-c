use std::{iter::Peekable, str::Chars};

use crate::{error::LexerError, token::Token};

pub struct Lexer<'l> {
    pub tokens: Vec<Token>,
    pub errors: Vec<LexerError>,
    source: Peekable<Chars<'l>>,
}

impl<'l> Lexer<'l> {
    pub fn new(source: &'l str) -> Self {
        Self {
            tokens: Vec::new(),
            errors: Vec::new(),
            source: source.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) {
        while self.source.peek().is_some() {
            self.match_token();
        }
        self.add_token(Token::Eof);
    }

    fn match_token(&mut self) {
        match self.source.next() {
            Some(c) => match c {
                '+' => self.add_token(Token::Plus),
                '-' => self.add_token(Token::Minus),
                '%' => self.add_token(Token::Modulo),
                '(' => self.add_token(Token::LeftParen),
                ')' => self.add_token(Token::RightParen),
                '{' => self.add_token(Token::LeftCurlyBr),
                '}' => self.add_token(Token::RightCurlyBr),
                '[' => self.add_token(Token::LeftBr),
                ']' => self.add_token(Token::RightBr),
                '?' => self.add_token(Token::Question),
                ':' => self.add_token(Token::Colon),
                ';' => self.add_token(Token::SemiColon),
                '*' => {
                    if let Some(&next) = self.source.peek() {
                        if next == '*' {
                            self.source.next();
                            self.add_token(Token::StarStar);
                        } else {
                            self.add_token(Token::Star);
                        }
                    } else {
                        self.add_token(Token::Star);
                    }
                }
                '/' => {
                    if let Some(&next) = self.source.peek() {
                        if next == '/' {
                            self.source.next();
                            self.add_token(Token::DoubleForwardSlash);
                        } else {
                            self.add_token(Token::ForwardSlash);
                        }
                    } else {
                        self.add_token(Token::ForwardSlash);
                    }
                }
                '=' => {
                    if let Some(&next) = self.source.peek() {
                        if next == '=' {
                            self.source.next();
                            self.add_token(Token::EqualEqual);
                        } else {
                            self.add_token(Token::Equal);
                        }
                    } else {
                        self.add_token(Token::Equal);
                    }
                }
                '"' => self.lex_str(c),
                'a'..='z' | 'A'..='Z' => self.lex_ident(c),
                '0'..='9' => self.lex_num(c),
                _ => {
                    if c.is_whitespace() {
                        return;
                    } else {
                        self.add_token(Token::Invalid(c));
                    }
                }
            },
            None => {}
        }
    }

    fn lex_ident(&mut self, first_char: char) {
        let mut ident_token = String::new();
        ident_token.push(first_char);

        let mut is_letter_start = false;
        let mut is_bad = false;

        while let Some(&token) = self.source.peek() {
            match token {
                '_' => {
                    ident_token.push(token);
                    self.source.next();
                }
                'a'..='z' | 'A'..='Z' => {
                    if !is_letter_start {
                        is_letter_start = true;
                    }
                    ident_token.push(token);
                    self.source.next();
                }
                '0'..='9' => {
                    if !is_letter_start {
                        is_bad = true;
                    }
                    ident_token.push(token);
                    self.source.next();
                }
                _ => {
                    // TODO: support non-ascii character
                    break;
                }
            }
        }

        match is_bad {
            true => self.add_err(LexerError::IdentStartWithNum(ident_token)),
            false => self.add_token(Token::Identifier(ident_token)),
        }
    }

    fn lex_num(&mut self, first_char: char) {
        let mut num_token = String::new();
        num_token.push(first_char);

        let mut is_dot = false;
        let mut is_bad = false;

        while let Some(&token) = self.source.peek() {
            match token {
                '0'..='9' => {
                    num_token.push(token);
                    self.source.next();
                }
                '.' => {
                    if is_dot {
                        is_bad = true;
                    } else {
                        is_dot = true;
                    }
                    num_token.push('.');
                    self.source.next();
                }
                _ => {
                    break;
                }
            }
        }

        match is_bad {
            true => self.add_err(LexerError::NumHasDots(num_token)),
            false => self.add_token(Token::Number(num_token)),
        }
    }

    fn lex_str(&mut self, first_char: char) {
        let mut str_token = String::new();
        str_token.push(first_char);

        let mut is_good = false;

        while let Some(token) = self.source.next() {
            match token {
                '"' => {
                    is_good = true;
                    break;
                }
                _ => {
                    str_token.push(token);
                }
            }
        }

        match is_good {
            true => self.add_token(Token::String(str_token)),
            false => self.add_err(LexerError::BadString(str_token)),
        }
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn add_err(&mut self, err: LexerError) {
        self.errors.push(err);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char() {
        let source = "+({)}[-/][*";
        let mut lexer = Lexer::new(source);
        lexer.tokenize();

        assert!(lexer.tokens.len() > 0, "should have 12 tokens but got 0");

        let good_tokens: Vec<Token> = vec![
            Token::Plus,
            Token::LeftParen,
            Token::LeftCurlyBr,
            Token::RightParen,
            Token::RightCurlyBr,
            Token::LeftBr,
            Token::Minus,
            Token::ForwardSlash,
            Token::RightBr,
            Token::LeftBr,
            Token::Star,
            Token::Eof,
        ];

        assert_eq!(
            good_tokens.len(),
            lexer.tokens.len(),
            "should have same 12 tokens"
        );

        for i in 0..lexer.tokens.len() {
            assert_eq!(lexer.tokens[i], good_tokens[i]);
        }
    }

    #[test]
    fn test_func_main() {
        let source = "
            int main() {
                return 69;
            }
        ";
        let mut lexer = Lexer::new(source);
        lexer.tokenize();

        assert!(lexer.tokens.len() > 0, "should have 10 tokens but got 0");

        let good_tokens: Vec<Token> = vec![
            Token::Identifier("int".to_string()),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftCurlyBr,
            Token::Identifier("return".to_string()),
            Token::Number("69".to_string()),
            Token::SemiColon,
            Token::RightCurlyBr,
            Token::Eof,
        ];

        assert_eq!(
            good_tokens.len(),
            lexer.tokens.len(),
            "should have same 10 tokens"
        );

        for i in 0..lexer.tokens.len() {
            assert_eq!(lexer.tokens[i], good_tokens[i]);
        }
    }
}
