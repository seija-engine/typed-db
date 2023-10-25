use crate::{ArcType, parser::utils};

use super::{lex_string::LexString, errors::ParserError};

pub struct Parser<'a> {
    source:LexString<'a>
}

impl<'a> Parser<'a> {
    pub fn new(code_string:&'a str) -> Parser<'a> {
        Parser {
            source:LexString::new(code_string)
        }
    }

    pub fn parse(&mut self) -> Result<ArcType,ParserError> {
        self.source.skip_whitespace();
        if let Some(chr) = self.source.next() {
            let ret = match chr {
                '@' => self.parse_meta(),
                 _  => todo!()
            };
        }
        todo!()
    }

    fn parse_meta(&mut self) -> Result<ArcType,ParserError> {
        self.source.skip_whitespace();
        if !self.source.lookahead(1).map(utils::is_sym_char_start).unwrap_or(false) {
            let next_char = self.source.lookahead(1).unwrap_or(' ');
            return Err(ParserError::ErrorMetaStart(self.source.pos(), next_char));
        }
        todo!()
    }

    

}