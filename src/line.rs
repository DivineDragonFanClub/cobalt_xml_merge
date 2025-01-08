use crate::*;

#[derive(Eq, Clone, Debug)]
pub struct Line<'xml> {
    data: XmlLine<'xml>,
    pub deleted: bool,
    pub front: Vec<Line<'xml>>,
    pub back: Vec<Line<'xml>>,
}

impl<'xml> Line<'xml> {
    // line with no content, set to deleted by default so it is ignored
    pub fn empty() -> Self {
        Self {
            data: "".into(),
            deleted: true,
            front: Vec::new(),
            back: Vec::new(),
        }
    }
    /// trims whitespace from the input
    pub fn new(original: XmlLine<'xml>) -> Self {
        Self {
            data: original,
            deleted: false,
            front: Vec::new(),
            back: Vec::new(),
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
            hunk.push(self.data);
        }
        for line in self.back.into_iter().rev() {
            if line.deleted { continue; }
            hunk.push(line.data);
        }
        hunk
    }

    /// Accounts for formatting inconsistencies (e.g. like Astra-XMLs).<br/>
    /// ...var="" /><br/>
    /// ...var=""/><br/>
    /// Whitespace-sensitivity between attributes remains, but people seem to be consistent with it.
    #[inline]
    pub fn get_trimmed(&'xml self) -> &'xml str {
        self.data.trim_end_matches("/>").trim()
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
}

impl std::hash::Hash for Line<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.get_trimmed().as_bytes());
    }
}

impl PartialEq for Line<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.get_trimmed() == other.get_trimmed() { return true }
        
        return match compare_non_whitespace(&self.data, &other.data) {
            CompareResult::Equal => true,
            _ => false,
        };
    }
}