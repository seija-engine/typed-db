use super::lex_string::TextPos;

pub enum ParserError {
    ErrorMetaStart(TextPos,char)
}