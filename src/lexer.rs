use crate::token::{self, literal, Kind};

#[derive(Debug)]
pub struct Lexer<'a> {
    ch: &'a str,
    input: &'a str,
    position: usize,
    read_position: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut lex = Lexer {
            input,
            ch: Default::default(),
            position: Default::default(),
            read_position: Default::default(),
        };

        lex.read_char();

        lex
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = literal::EOF;
        } else {
            self.ch = &self.input[self.read_position..self.read_position + 1];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> token::Token {
        let mut tok = token::Token {
            ..Default::default()
        };

        self.eat_whitespace();
        println!("{self:?}");
        match self.ch {
            literal::ASSIGN => tok = Lexer::new_token(Kind::ASSIGN, self.ch),
            literal::SEMICOLON => tok = Lexer::new_token(Kind::SEMICOLON, self.ch),
            literal::LPAREN => tok = Lexer::new_token(Kind::LPAREN, self.ch),
            literal::RPAREN => tok = Lexer::new_token(Kind::RPAREN, self.ch),
            literal::COMMA => tok = Lexer::new_token(Kind::COMMA, self.ch),
            literal::PLUS => tok = Lexer::new_token(Kind::PLUS, self.ch),
            literal::LBRACE => tok = Lexer::new_token(Kind::LBRACE, self.ch),
            literal::RBRACE => tok = Lexer::new_token(Kind::RBRACE, self.ch),
            literal::EOF => tok = Lexer::new_token(Kind::EOF, self.ch),
            _ => {
                if Lexer::is_letter(self.ch) {
                    tok.literal = self.read_identifier();
                    tok.kind = token::Token::lookup_keyword(tok.literal);
                    return tok;
                } else if Lexer::is_Digit(self.ch) {
                    return Lexer::new_token(Kind::INT, self.read_digit());
                } else {
                    tok = Lexer::new_token(Kind::ILLEGAL, self.ch)
                }
            }
        }

        self.read_char();

        tok
    }

    fn eat_whitespace(&mut self) {
        while self.ch.to {
            self.read_char();
        }
    }

    fn is_letter(l: &str) -> bool {
        l.chars()
            .any(|x| (x >= 'a' && x <= 'z') || (x >= 'A' && x <= 'Z'))
    }

    fn is_Digit(l: &str) -> bool {
        l.chars().any(|x| (x >= '0' && x <= '9'))
    }

    fn read_identifier(&mut self) -> &str {
        let pos = self.position;

        while Lexer::is_letter(&self.input[self.position..self.position + 1]) {
            self.read_char();
        }
        &self.input[pos..self.position]
    }

    fn read_digit(&mut self) -> &str {
        let pos = self.position;

        while Lexer::is_Digit(&self.input[self.position..self.position + 1]) {
            self.read_char();
        }
        &self.input[pos..self.position]
    }

    fn new_token(kind: Kind, ch: &str) -> token::Token {
        token::Token { kind, literal: ch }
    }
}

#[cfg(test)]
mod tests {

    use crate::token::{self, literal, Token};
    use std::fmt::Debug;

    use super::*;

    fn lexer_error_str<T: Debug>(i: usize, wanted: T, got: T) -> String {
        String::from(format!(
            "WRONG::: test #{:?} Expected: {:?} but Got: {:?}",
            i, wanted, got
        ))
    }

    #[test]
    fn test_next_token() -> Result<(), String> {
        let input = r#"
            let five = 5;
                let ten = 10;
            let add = fn(x, y) {
            x + y;
            };
            let result = add(five, ten);
        "#;

        let tests = [
            Token {
                kind: Kind::LET,
                literal: "let",
            },
            Token {
                kind: Kind::IDENT,
                literal: "five",
            },
            Token {
                kind: Kind::ASSIGN,
                literal: "=",
            },
            Token {
                kind: Kind::INT,
                literal: "5",
            },
            Token {
                kind: Kind::SEMICOLON,
                literal: ";",
            },
            Token {
                kind: Kind::LET,
                literal: "let",
            },
            Token {
                kind: Kind::IDENT,
                literal: "ten",
            },
            Token {
                kind: Kind::ASSIGN,
                literal: "=",
            },
            Token {
                kind: Kind::INT,
                literal: "10",
            },
            Token {
                kind: Kind::SEMICOLON,
                literal: ";",
            },
            Token {
                kind: Kind::LET,
                literal: "let",
            },
            Token {
                kind: Kind::IDENT,
                literal: "add",
            },
            Token {
                kind: Kind::ASSIGN,
                literal: "=",
            },
            Token {
                kind: Kind::FUNCTION,
                literal: "fn",
            },
            Token {
                kind: Kind::LPAREN,
                literal: "(",
            },
            Token {
                kind: Kind::IDENT,
                literal: "x",
            },
            Token {
                kind: Kind::COMMA,
                literal: ",",
            },
            Token {
                kind: Kind::IDENT,
                literal: "y",
            },
            Token {
                kind: Kind::RPAREN,
                literal: ")",
            },
            Token {
                kind: Kind::LBRACE,
                literal: "{",
            },
            Token {
                kind: Kind::IDENT,
                literal: "x",
            },
            Token {
                kind: Kind::PLUS,
                literal: "+",
            },
            Token {
                kind: Kind::IDENT,
                literal: "y",
            },
            Token {
                kind: Kind::SEMICOLON,
                literal: ";",
            },
            Token {
                kind: Kind::RBRACE,
                literal: "}",
            },
            Token {
                kind: Kind::SEMICOLON,
                literal: ";",
            },
            Token {
                kind: Kind::LET,
                literal: "let",
            },
            Token {
                kind: Kind::IDENT,
                literal: "result",
            },
            Token {
                kind: Kind::ASSIGN,
                literal: "=",
            },
            Token {
                kind: Kind::IDENT,
                literal: "add",
            },
            Token {
                kind: Kind::LPAREN,
                literal: "(",
            },
            Token {
                kind: Kind::IDENT,
                literal: "five",
            },
            Token {
                kind: Kind::COMMA,
                literal: ",",
            },
            Token {
                kind: Kind::IDENT,
                literal: "ten",
            },
            Token {
                kind: Kind::RPAREN,
                literal: ")",
            },
            Token {
                kind: Kind::SEMICOLON,
                literal: ";",
            },
            Token {
                kind: Kind::EOF,
                literal: "",
            },
        ];

        let mut lex = Lexer::new(input);

        for (i, value) in tests.iter().enumerate() {
            let tok = lex.next_token();

            if tok.kind != value.kind {
                return Err(lexer_error_str(i + 1, value.kind, tok.kind));
            }

            if tok.literal != value.literal {
                return Err(lexer_error_str(i + 1, value.literal, tok.literal));
            }
        }

        Ok(())
    }
}
