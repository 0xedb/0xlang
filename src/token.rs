mod kind;
pub mod literal;

use std::collections::HashMap;

pub use kind::Kind;

#[derive(Debug, Default)]
pub struct Token<'a> {
    pub kind: kind::Kind,
    pub literal: &'a str,
}

impl<'a> Token<'a> {
    pub fn lookup_keyword(k: &str) -> Kind { 
        let mp: HashMap<&str, Kind> = HashMap::from([
            (literal::LET, Kind::LET),
            (literal::FUNCTION, Kind::FUNCTION),
        ]);

        if let Some(val) = mp.get(k) {
            return *val;
        }
        Kind::IDENT
    }
}
