#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Kind {
    // terminators + eof
    EOF,
    ILLEGAL,

    // literals + identifiers
    INT,
    IDENT,

    // operators
    PLUS,
    ASSIGN,

    // delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,

    LBRACE,
    RBRACE,

    // keywords
    LET,
    FUNCTION,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::EOF
    }
}
