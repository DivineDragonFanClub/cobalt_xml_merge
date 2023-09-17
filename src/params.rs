use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ParamMerger {
    default: Vec<(Rc<str>, Rc<str>)>,
    merged: Vec<(Rc<str>, Rc<str>)>,
}

impl ParamMerger { 
    pub fn new(line: &str) -> Self {
        let doc = roxmltree::Document::parse(line).unwrap_or_else(|e| {
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
        Self {
            default,
            merged,
        }
    }

    pub fn patch(&mut self, line: &str) {
        let doc = roxmltree::Document::parse(line).unwrap();
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
        output.push_str("<Param ");
        for (name, value) in &self.merged {
            output.push_str(&format!("{}=\"{}\" ", name, value));
        }
        output.push_str("/>");
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