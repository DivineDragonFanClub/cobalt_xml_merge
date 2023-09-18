use crate::{XmlLine, params::ParamMerger};

#[derive(Eq, Clone, Debug)]
pub struct Line<'xml> {
    data: XmlLine<'xml>,
    pub deleted: bool,
    checksum: u32,
    pub front: Vec<Line<'xml>>,
    pub back: Vec<Line<'xml>>,
    replace: Option<ParamMerger>
}

impl<'xml> Line<'xml> {
    // line with no content, set to deleted by default so it is ignored
    pub fn empty() -> Self {
        Self {
            data: "".into(),
            deleted: true,
            checksum: 0,
            front: Vec::new(),
            back: Vec::new(),
            replace: None
        }
    }
    /// trims whitespace from the input
    pub fn new(original: XmlLine<'xml>) -> Self {
        let checksum = crc32fast::hash(original.as_bytes());
        Self {
            data: original,
            deleted: false,
            checksum,
            front: Vec::new(),
            back: Vec::new(),
            replace: None
        }
    }

    /// Converts the line into a hunk of lines.
    /// 
    /// `[front3, front2, front1, original, back3, back2, back1]`
    /// 
    /// meaning both appending and prepending retains the mod loading order
    pub fn to_hunk(self) -> Vec<XmlLine<'xml>> {
        let mut hunk = Vec::with_capacity(
            self.front.len() + self.back.len() + 1
        );
        for line in self.front.into_iter().rev() {
            if line.deleted { continue; }
            hunk.push(line.data);
        }
        if !self.deleted {
            if let Some(replace) = self.replace {
                let new = replace.to_string();
                hunk.push(new.into());
            } else {
                hunk.push(self.data);
            }
        }
        for line in self.back.into_iter().rev() {
            if line.deleted { continue; }
            hunk.push(line.data);
        }
        hunk
    }

    pub fn insert_above(&mut self, lines: &[Line<'xml>]) {
        for line in lines.iter().rev() {
            self.front.push(line.clone());
        }
    }
    
    pub fn insert_below(&mut self, lines: &[Line<'xml>]) {
        for line in lines.iter().rev() {
            self.back.push(line.clone());
        }
    }

    pub fn patch_params(&mut self, patch: &Line) {
        self.replace.get_or_insert(ParamMerger::new(&self.data));
        self.replace.as_mut().map(|replace| replace.patch(&patch.data));
    }
}

impl std::hash::Hash for Line<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.checksum);
    }
}

impl PartialEq for Line<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.checksum == other.checksum { return true }
        
        if self.data.len() != other.data.len() { return false };
    
        return self.data == other.data;
    }
}