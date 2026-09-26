use std::{iter::Peekable, str::CharIndices};

use crate::token::{Token, TokenData};

pub struct Lexer<'l> {
    pub tokens: Vec<TokenData>,
    source: Peekable<CharIndices<'l>>,
    line: usize,
    last_pos: usize,
}

struct LexError {
    msg: String,
    line: usize,
    pos: usize,
}

impl<'l> Lexer<'l> {
    pub fn new(source: &'l str) -> Self {
        Self {
            tokens: Vec::new(),
            source: source.char_indices().peekable(),
            line: 1,
            last_pos: source.len(),
        }
    }

    pub fn tokenize(&mut self) {
        while self.source.peek().is_some() {
            if let Err(e) = self.match_token() {
                eprintln!(
                    "[LEXER ERROR] [Line: {} | Pos: {}]: {}",
                    e.line, e.pos, e.msg
                );
            }
        }
        self.add_token(Token::Eof, self.last_pos);
    }

    fn match_token(&mut self) -> Result<(), LexError> {
        match self.source.next() {
            Some(c) => match c.1 {
                '+' => self.add_token(Token::Plus, c.0),
                '-' => self.add_token(Token::Minus, c.0),
                '%' => self.add_token(Token::Modulo, c.0),
                '(' => self.add_token(Token::LeftParen, c.0),
                ')' => self.add_token(Token::RightParen, c.0),
                '{' => self.add_token(Token::LeftCurlyBr, c.0),
                '}' => self.add_token(Token::RightCurlyBr, c.0),
                '[' => self.add_token(Token::LeftBr, c.0),
                ']' => self.add_token(Token::RightBr, c.0),
                '?' => self.add_token(Token::Question, c.0),
                ':' => self.add_token(Token::Colon, c.0),
                ';' => self.add_token(Token::SemiColon, c.0),
                '*' => {
                    if let Some(&next) = self.source.peek() {
                        if next.1 == '*' {
                            self.source.next();
                            self.add_token(Token::StarStar, next.0);
                        } else {
                            self.add_token(Token::Star, c.0);
                        }
                    } else {
                        self.add_token(Token::Star, c.0);
                    }
                }
                '<' => self.add_token(Token::Less, c.0),
                '>' => self.add_token(Token::Greater, c.0),
                '&' => {
                    if let Some(&next) = self.source.peek() {
                        if next.1 == '&' {
                            self.source.next();
                            self.add_token(Token::And, next.0);
                        } else {
                            self.add_token(Token::BitAnd, c.0);
                        }
                    } else {
                        self.add_token(Token::BitAnd, c.0);
                    }
                }
                '|' => {
                    if let Some(&next) = self.source.peek() {
                        if next.1 == '|' {
                            self.source.next();
                            self.add_token(Token::Or, next.0);
                        } else {
                            self.add_token(Token::BitOr, c.0);
                        }
                    } else {
                        self.add_token(Token::BitOr, c.0);
                    }
                }
                '/' => {
                    if let Some(&next) = self.source.peek() {
                        if next.1 == '/' {
                            self.source.next();
                            self.add_token(Token::DoubleForwardSlash, next.0);
                        } else {
                            self.add_token(Token::ForwardSlash, c.0);
                        }
                    } else {
                        self.add_token(Token::ForwardSlash, c.0);
                    }
                }
                '=' => {
                    if let Some(&next) = self.source.peek() {
                        if next.1 == '=' {
                            self.source.next();
                            self.add_token(Token::EqualEqual, next.0);
                        } else {
                            self.add_token(Token::Equal, c.0);
                        }
                    } else {
                        self.add_token(Token::Equal, c.0);
                    }
                }
                '"' => self.lex_str(c)?,
                'a'..='z' | 'A'..='Z' => self.lex_ident(c)?,
                '0'..='9' => self.lex_num(c)?,
                _ => {
                    if c.1 == '\n' {
                        self.line += 1;
                    } else if c.1.is_whitespace() {
                        return Ok(());
                    } else {
                        self.add_token(Token::Invalid(c.1), c.0);
                    }
                }
            },
            None => {}
        }

        Ok(())
    }

    fn lex_ident(&mut self, first_char: (usize, char)) -> Result<(), LexError> {
        let mut ident_token = String::new();
        ident_token.push(first_char.1);

        let mut is_letter_start = false;
        let mut is_good = true;

        while let Some(&token) = self.source.peek() {
            match token.1 {
                '_' => {
                    ident_token.push(token.1);
                    self.source.next();
                }
                'a'..='z' | 'A'..='Z' => {
                    if !is_letter_start {
                        is_letter_start = true;
                    }
                    ident_token.push(token.1);
                    self.source.next();
                }
                '0'..='9' => {
                    if !is_letter_start {
                        is_good = false
                    }
                    ident_token.push(token.1);
                    self.source.next();
                }
                _ => {
                    // TODO: support non-ascii character
                    break;
                }
            }
        }

        match is_good {
            true => {
                self.add_token(Token::Identifier(ident_token), first_char.0);
                Ok(())
            }
            false => {
                return Err(LexError {
                    msg: format!("identifier start with a number: '{}'", ident_token),
                    line: self.line,
                    pos: first_char.0,
                });
            }
        }
    }

    fn lex_num(&mut self, first_char: (usize, char)) -> Result<(), LexError> {
        let mut num_token = String::new();
        num_token.push(first_char.1);

        let mut is_dot = false;
        let mut is_good = true;

        while let Some(&token) = self.source.peek() {
            match token.1 {
                '0'..='9' => {
                    num_token.push(token.1);
                    self.source.next();
                }
                '.' => {
                    if is_dot {
                        is_good = false;
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

        match is_good {
            true => {
                self.add_token(Token::Number(num_token), first_char.0);
                Ok(())
            }
            false => {
                return Err(LexError {
                    msg: format!(
                        "number contains more than one decimal point: '{}'",
                        num_token
                    ),
                    line: self.line,
                    pos: first_char.0,
                });
            }
        }
    }

    fn lex_str(&mut self, first_char: (usize, char)) -> Result<(), LexError> {
        let mut str_token = String::new();
        str_token.push(first_char.1);

        let mut is_good = false;

        while let Some(token) = self.source.next() {
            match token.1 {
                '"' => {
                    is_good = true;
                    break;
                }
                _ => {
                    str_token.push(token.1);
                }
            }
        }

        match is_good {
            true => {
                self.add_token(Token::String(str_token), first_char.0);
                Ok(())
            }
            false => {
                return Err(LexError {
                    msg: format!("string doesn't end with a quotation: ({})", str_token),
                    line: self.line,
                    pos: first_char.0,
                });
            }
        }
    }

    fn add_token(&mut self, token: Token, pos: usize) {
        self.tokens.push(TokenData {
            token,
            line: self.line,
            pos,
        });
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
            assert_eq!(lexer.tokens[i].token, good_tokens[i]);
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
            assert_eq!(lexer.tokens[i].token, good_tokens[i]);
        }
    }
}
