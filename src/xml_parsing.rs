#[derive(logos::Logos, Debug, PartialEq, Hash)]
#[logos(skip r"[ \r\t\f]+")] // Ignore this regex pattern between tokens
pub enum XmlToken<'a> {
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
    #[regex(r"[a-zA-Z][a-zA-Z0-9_:]*", |lex| lex.slice())]
    Identifier(&'a str), 

    /// Token for the equal sign between attribute name and value
    #[token("=")]
    Equals,

    /// Token for attribute values enclosed in double quotes
    #[regex(r#""[^"]*""#, |lex| lex.slice())]
    AttributeValue(&'a str),
}

impl std::fmt::Display for XmlToken<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XmlToken::TagStart => write!(f, "<"),
            XmlToken::TagEnd => write!(f, ">"),
            XmlToken::ClosingTagStart => write!(f, "</"),
            XmlToken::CommentStart => write!(f, "<!"),
            XmlToken::SelfClosing => write!(f, "/>"),
            XmlToken::Identifier(s) => write!(f, "{}", s),
            XmlToken::Equals => write!(f, "="),
            XmlToken::AttributeValue(s) => write!(f, "{} ", s),
        }
    }
}

pub fn next_token<'a>(lexer: &mut logos::Lexer<'a, XmlToken<'a>>) -> Option<XmlToken<'a>> {
    while let Some(res) = lexer.next() {
        if let Ok(token) = res {
            return Some(token);
        };
    }
    return None;
}