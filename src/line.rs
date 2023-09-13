use crate::XmlLine;

#[derive(Eq, Clone)]
pub struct Line<'xml> {
    data: XmlLine<'xml>,
    pub deleted: bool,
    checksum: u32,
    pub front: Vec<Line<'xml>>,
    pub back: Vec<Line<'xml>>
}

impl<'xml> Line<'xml> {
    // line with no content, set to deleted by default so it is ignored
    pub fn empty() -> Self {
        Self {
            data: "".into(),
            deleted: true,
            checksum: 0,
            front: Vec::new(),
            back: Vec::new()
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
            back: Vec::new()
        }
    }

    /// Converts the line into a hunk of lines.
    /// 
    /// `[front1, front2, front3, original, back1, back2, back3]`
    /// 
    /// meaning both appending and prepending retains the mod loading order
    pub fn to_hunk(self) -> Vec<XmlLine<'xml>> {
        let mut hunk = Vec::with_capacity(
            self.front.len() + self.back.len() + 1
        );
        for line in self.front.into_iter() {
            if line.deleted { continue; }
            hunk.push(line.data);
        }
        if !self.deleted {
            hunk.push(self.data);
        }
        for line in self.back.into_iter() {
            if line.deleted { continue; }
            hunk.push(line.data);
        }
        hunk
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