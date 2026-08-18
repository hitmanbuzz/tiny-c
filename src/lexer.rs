use crate::{error::LexerError, token::Token};

pub struct Lexer<'l> {
    pub tokens: Vec<Token>,
    pub errors: Vec<LexerError>,
    source: &'l str,
    idx: usize,
}

impl<'l> Lexer<'l> {
    pub fn new(source: &'l str) -> Self {
        Self {
            tokens: Vec::new(),
            errors: Vec::new(),
            source: source,
            idx: 0,
        }
    }

    pub fn tokenize(&mut self) {
        while let Some(token) = self.peek(self.idx) {
            self.match_token(token);
        }
        self.add_token(Token::Eof);
    }

    fn match_token(&mut self, token: char) {
        // i am pretty sure this is guaranteed not to fail
        match token {
            '+' => {
                self.add_token(Token::Plus);
                self.offset(1);
            }
            '-' => {
                self.add_token(Token::Minus);
                self.offset(1);
            }
            '\\' => {
                self.add_token(Token::BackSlash);
                self.offset(1);
            }
            '%' => {
                self.add_token(Token::Modulo);
                self.offset(1);
            }
            '(' => {
                self.add_token(Token::LeftParen);
                self.offset(1);
            }
            ')' => {
                self.add_token(Token::RightParen);
                self.offset(1);
            }
            '{' => {
                self.add_token(Token::LeftCurlyBr);
                self.offset(1);
            }
            '}' => {
                self.add_token(Token::RightCurlyBr);
                self.offset(1);
            }
            '[' => {
                self.add_token(Token::LeftBr);
                self.offset(1);
            }
            ']' => {
                self.add_token(Token::RightBr);
                self.offset(1);
            }
            '?' => {
                self.add_token(Token::Question);
                self.offset(1);
            }
            ':' => {
                self.add_token(Token::Colon);
                self.offset(1);
            }
            ';' => {
                self.add_token(Token::SemiColon);
                self.offset(1);
            }
            '*' => {
                if let Some(next) = self.peek(self.idx + 1) {
                    if next == '*' {
                        self.add_token(Token::StarStar);
                        self.offset(2);
                    } else {
                        self.add_token(Token::Star);
                        self.offset(1);
                    }
                }
                self.offset(1);
            }
            '/' => {
                if let Some(next) = self.peek(self.idx + 1) {
                    if next == '/' {
                        self.add_token(Token::DoubleForwardSlash);
                        self.offset(2);
                    } else {
                        self.add_token(Token::Invalid('/'));
                        self.offset(1);
                    }
                }
            }
            '=' => {
                if let Some(next) = self.peek(self.idx + 1) {
                    if next == '=' {
                        self.add_token(Token::EqualEqual);
                        self.offset(2);
                    } else {
                        self.add_token(Token::Equal);
                        self.offset(1);
                    }
                }
            }
            '"' => self.lex_str(),
            'a'..='z' | 'A'..='Z' => self.lex_ident(),
            '0'..='9' => self.lex_num(),
            _ => {
                if token.is_whitespace() {
                    self.offset(1);
                } else {
                    self.add_token(Token::Invalid(token));
                    self.offset(1);
                }
            }
        }
    }

    fn lex_ident(&mut self) {
        let mut ident_token = String::new();
        let mut is_letter_start = false;
        let mut is_bad = false;

        while let Some(token) = self.peek(self.idx) {
            match token {
                'a'..='z' | 'A'..='Z' => {
                    if !is_letter_start {
                        is_letter_start = true;
                    }
                    ident_token.push(token);
                    self.offset(1);
                }
                '0'..='9' => {
                    if !is_letter_start {
                        is_bad = true;
                    }
                    ident_token.push(token);
                    self.offset(1);
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

    fn lex_num(&mut self) {
        let mut num_token = String::new();
        let mut is_dot = false;
        let mut is_bad = false;

        while let Some(token) = self.peek(self.idx) {
            match token {
                '0'..='9' => {
                    num_token.push(token);
                    self.offset(1);
                }
                '.' => {
                    if is_dot {
                        is_bad = true;
                    } else {
                        is_dot = true;
                    }
                    num_token.push('.');
                    self.offset(1);
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

    fn lex_str(&mut self) {
        let mut str_token = String::new();
        let mut counter = 0;
        let mut is_good = false;

        while let Some(token) = self.peek(self.idx) {
            match token {
                '"' => {
                    counter += 1;
                    self.offset(1);
                    if counter == 2 {
                        is_good = true;
                        break;
                    }
                }
                _ => {
                    str_token.push(token);
                    self.offset(1);
                }
            }
        }

        match is_good {
            true => self.add_token(Token::String(str_token)),
            false => self.add_err(LexerError::BadString(str_token)),
        }
    }

    // actually return a character
    fn peek(&self, idx: usize) -> Option<char> {
        if let Some(c) = self.source.get(idx..=idx) {
            return c.chars().next();
        }
        return None;
    }

    // idk whether to use this or not
    #[allow(dead_code)]
    fn next(&mut self) -> Option<char> {
        let curr_idx = self.idx;
        let curr = self.peek(curr_idx);
        if curr.is_some() {
            self.offset(1);
        }
        return curr;
    }

    fn offset(&mut self, offset_value: usize) {
        self.idx += offset_value;
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
    fn test_func_main() {
        let source = "
            int main() {
                return 0;
            }
        ";
        let mut lexer = Lexer::new(source);
        lexer.tokenize();

        assert!(
            lexer.tokens.len() > 0,
            "should have 10 tokens but got 0 instead"
        );

        let good_tokens: Vec<Token> = vec![
            Token::Identifier("int".to_string()),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftCurlyBr,
            Token::Identifier("return".to_string()),
            Token::Number("0".to_string()),
            Token::SemiColon,
            Token::RightCurlyBr,
            Token::Eof,
        ];

        assert_eq!(
            lexer.tokens.len(),
            good_tokens.len(),
            "should have same 10 tokens"
        );

        for i in 0..lexer.tokens.len() {
            assert_eq!(lexer.tokens[i], good_tokens[i]);
        }
    }
}
