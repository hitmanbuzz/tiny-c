use crate::token::Token;

pub struct Lexer<'l> {
    pub tokens: Vec<Token>,
    pub errors: Vec<LexerError>,
    pub source: &'l str,
    pub idx: usize,
}

#[allow(non_camel_case_types)]
enum LexerError {
    IDENT_START_WITH_NUM(String),
    NUM_HAS_MORE_DOTS(String),
    BAD_STRING(String),
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
    }

    fn match_token(&mut self, token: char) {
        // i am pretty sure this is guaranteed not to fail
        match token {
            '+' => {
                self.add_token(Token::PLUS);
                self.offset(1);
            }
            '-' => {
                self.add_token(Token::MINUS);
                self.offset(1);
            }
            '\\' => {
                self.add_token(Token::BACK_SLASH);
                self.offset(1);
            }
            '%' => {
                self.add_token(Token::MODULO);
                self.offset(1);
            }
            '(' => {
                self.add_token(Token::LEFT_PAREN);
                self.offset(1);
            }
            ')' => {
                self.add_token(Token::RIGHT_PAREN);
                self.offset(1);
            }
            '{' => {
                self.add_token(Token::LEFT_CURLY_BR);
                self.offset(1);
            }
            '}' => {
                self.add_token(Token::RIGHT_CURLY_BR);
                self.offset(1);
            }
            '[' => {
                self.add_token(Token::LEFT_BR);
                self.offset(1);
            }
            ']' => {
                self.add_token(Token::RIGHT_BR);
                self.offset(1);
            }
            '?' => {
                self.add_token(Token::QUESTION);
                self.offset(1);
            }
            ':' => {
                self.add_token(Token::COLON);
                self.offset(1);
            }
            ';' => {
                self.add_token(Token::SEMI_COLON);
                self.offset(1);
            }
            '*' => {
                if let Some(next) = self.peek(self.idx + 1) {
                    if next == '*' {
                        self.add_token(Token::DOUBLE_STAR);
                        self.offset(2);
                    } else {
                        self.add_token(Token::STAR);
                        self.offset(1);
                    }
                }
                self.offset(1);
            }
            '/' => {
                if let Some(next) = self.peek(self.idx + 1) {
                    if next == '/' {
                        self.add_token(Token::DOUBLE_FORWARD_SLASH);
                        self.offset(2);
                    } else {
                        self.add_token(Token::INVALID('/'));
                        self.offset(1);
                    }
                }
            }
            '=' => {
                if let Some(next) = self.peek(self.idx + 1) {
                    if next == '=' {
                        self.add_token(Token::EQUAL_EQUAL);
                        self.offset(2);
                    } else {
                        self.add_token(Token::EQUAL);
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
                    self.add_token(Token::INVALID(token));
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
            true => self.add_err(LexerError::IDENT_START_WITH_NUM(ident_token)),
            false => self.add_token(Token::IDENTIFIER(ident_token)),
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
            true => self.add_err(LexerError::NUM_HAS_MORE_DOTS(num_token)),
            false => self.add_token(Token::NUMBER(num_token)),
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
            true => self.add_token(Token::STRING(str_token)),
            false => self.add_err(LexerError::BAD_STRING(str_token)),
        }
    }

    // actually return a character
    fn peek(&self, idx: usize) -> Option<char> {
        if let Some(c) = self.source.get(idx..=idx) {
            return c.chars().next();
        }
        return None;
    }

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
            "should have 9 tokens but got 0 instead"
        );

        let good_tokens: Vec<Token> = vec![
            Token::IDENTIFIER("int".to_string()),
            Token::IDENTIFIER("main".to_string()),
            Token::LEFT_PAREN,
            Token::RIGHT_PAREN,
            Token::LEFT_CURLY_BR,
            Token::IDENTIFIER("return".to_string()),
            Token::NUMBER("0".to_string()),
            Token::SEMI_COLON,
            Token::RIGHT_CURLY_BR,
        ];

        for i in 0..lexer.tokens.len() {
            assert_eq!(lexer.tokens[i], good_tokens[i]);
        }
    }
}
