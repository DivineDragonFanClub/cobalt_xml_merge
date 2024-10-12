use std::{borrow::Cow, rc::Rc};

#[derive(logos::Logos, Debug, PartialEq)]
#[logos(skip r"[ \r\t\f]+")] // Ignore this regex pattern between tokens
pub enum XmlToken {
    #[token("<")]
    TagStart,

    #[token(">")]
    TagEnd,

    #[token("</")]
    ClosingTagStart,

    #[token("<!")]
    CommentStart,

    #[token("/>")]
    SelfClosing,

    /// Token for attribute names (identifiers), the tagname and attribute are found here
    #[regex(r"[a-zA-Z][a-zA-Z0-9_:]*", |lex| to_u8_range(lex.span()))]
    Identifier(std::ops::Range<u8>), 

    /// Token for the equal sign between attribute name and value
    #[token("=")]
    Equals,

    /// Token for attribute values enclosed in double quotes
    #[regex(r#""[^"]*""#, |lex| to_u8_range(lex.span()))]
    AttributeValue(std::ops::Range<u8>),
}

fn to_u8_range(range: std::ops::Range<usize>) -> std::ops::Range<u8> {
    range.start as u8..range.end as u8
}

pub fn next_token<'a>(lexer: &mut logos::Lexer<'a, XmlToken>) -> Option<XmlToken> {
    while let Some(res) = lexer.next() {
        if let Ok(token) = res {
            return Some(token);
        };
    }
    return None;
}