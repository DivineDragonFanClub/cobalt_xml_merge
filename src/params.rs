use std::{rc::Rc, borrow::Cow};

#[derive(Debug, Clone)]
pub struct ParamMerger {
    default: Vec<(Rc<str>, Rc<str>)>,
    merged: Vec<(Rc<str>, Rc<str>)>,
    tag_name: Rc<str>,
    tag_type: Tag,
}

impl ParamMerger { 
    pub fn new(line: &str) -> Self {
        let (tag_type, line) = get_tag(line);

        match tag_type {
            Tag::Closing => Self {
                default: vec![],
                merged: vec![],
                tag_name: line.into(),
                tag_type,
            },
            _ => {
                let doc = roxmltree::Document::parse(&line).unwrap_or_else(|e| {
                    println!("error line: {line}");
                    panic!("failed to parse line: {e}");
                });
                let mut default = vec![];
                for attr in doc.root_element().attributes() {
                    let name = attr.name();
                    let value = attr.value();
                    default.push((name.into(), value.into()));
                }
                let merged = default.clone();
                let tag_name = doc.root_element().tag_name().name().into();
                Self {
                    default,
                    merged,
                    tag_name,
                    tag_type,
                }
            }
        }


    }

    pub fn patch(&mut self, line: &str) {
        let (tag_type, line) = get_tag(line);
        match tag_type {
            Tag::Closing => return,
            _ => {}
        }
        let doc = roxmltree::Document::parse(&line).unwrap_or_else(|e| {
            println!("error line: {line}");
            panic!("failed to parse line: {e}");
        });
        for (i, attr) in doc.root_element().attributes().enumerate() {
            // let name = attr.name();
            let value = attr.value();
            let default: &str = &self.default[i].1;

            if default != value {
                self.merged[i].1 = value.into();
            }
        }

    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        match self.tag_type {
            Tag::Closing => output.push_str("</"),
            Tag::Opening => output.push_str("<"),
            Tag::Empty => output.push_str("<"),
        }

        output.push_str(&self.tag_name);
        for (name, value) in &self.merged {
            output.push_str(&format!(" {}=\"{}\"", name, value));
        }

        match self.tag_type {
            Tag::Closing => output.push_str(">"),
            Tag::Opening => output.push_str(">"),
            Tag::Empty => output.push_str(" />"),
        }
        output
    }
}

impl PartialEq for ParamMerger {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for ParamMerger {

}

#[derive(Debug, Clone)]
enum Tag {
    Closing,
    Opening,
    Empty,
}

impl Tag {
    /// make sure input is trimmed
    pub fn from(input: &str) -> Self {
        // </Data>
        if input.starts_with("</"){
            return Self::Closing
        }
        if input.ends_with("/>") {
            return Self::Empty
        }
        return Self::Opening
    }
}

fn get_tag<'a>(line: &'a str) -> (Tag, Cow<'a, str>) {
    let tag = Tag::from(line); 
    match tag {
        // </Data>
        Tag::Closing => {
            // return inbetween </ and >
            return (tag, Cow::Borrowed(line.trim_start_matches("</").trim_end_matches(">").trim()));
        }
        // <Param />
        Tag::Empty => {
            return (tag, Cow::Borrowed(line));
        }
        // <Header>
        Tag::Opening => {
            // into <Header/> so it can be parsed
            // cloning is fine because it's a small string and a rare case
            return (tag, Cow::Owned(line.trim_end_matches(">").to_owned() + "/>"));
        }
    }
}
