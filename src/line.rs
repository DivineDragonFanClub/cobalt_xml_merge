use crate::*;

#[derive(Eq, Clone, Debug)]
pub struct Line<'xml> {
    data: XmlLine<'xml>,
    pub deleted: bool,
    norm: XmlLine<'xml>,
    pub front: Option<Vec<Line<'xml>>>,
    pub back: Option<Vec<Line<'xml>>>,
}

impl<'xml> Line<'xml> {
    // line with no content, set to deleted by default so it is ignored
    pub fn empty() -> Self {
        Self {
            data: "",
            deleted: true,
            norm: "",
            front: None,
            back: None,
        }
    }

    /// trims whitespace from the input
    pub fn new(original: XmlLine<'xml>) -> Self {
        let norm = original.trim_end_matches("/>").trim();
        Self {
            data: original,
            deleted: false,
            norm,
            front: None,
            back: None,
        }
    }

    pub fn tag(&self) -> Tag<'xml> {
        Tag { norm: self.norm }
    }

    /// Converts the line into a hunk of lines.
    ///
    /// `[front3, front2, front1, original, back3, back2, back1]`
    ///
    /// meaning both appending and prepending retains the mod loading order
    pub fn to_hunk(self) -> Vec<XmlLine<'xml>> {
        let mut hunk = Vec::with_capacity(self.hunk_len());
        self.write_hunk(&mut hunk);
        hunk
    }

    pub fn hunk_len(&self) -> usize {
        let mut size = usize::from(!self.deleted);
        if let Some(v) = self.front.as_ref() {
            size += v.len();
        }
        if let Some(v) = self.back.as_ref() {
            size += v.len();
        }
        size
    }

    pub fn write_hunk(self, out: &mut Vec<XmlLine<'xml>>) {
        if let Some(v) = self.front {
            out.extend(v.into_iter().rev().filter(|l| !l.deleted).map(|l| l.data));
        }
        if !self.deleted {
            out.push(self.data);
        }
        if let Some(v) = self.back {
            out.extend(v.into_iter().rev().filter(|l| !l.deleted).map(|l| l.data));
        }
    }

    pub fn insert_above(&mut self, lines: &[Line<'xml>]) {
        let front = self.front.get_or_insert(Vec::new());
        for line in lines.iter().rev() {
            front.push(line.clone());
        }
    }

    pub fn insert_below(&mut self, lines: &[Line<'xml>]) {
        let back = self.back.get_or_insert(Vec::new());
        for line in lines.iter().rev() {
            back.push(line.clone());
        }
    }

    pub fn insert_below_overwrite(&mut self, lines: &[Line<'xml>]) {
        let same_block = self.block_tag();
        let back = self.back.get_or_insert(Vec::new());
        for line in lines.iter().rev() {
            if same_block.is_some() && line.block_tag() == same_block {
                if let Some(prev) = back.iter_mut().find(|prev| prev.block_tag() == same_block) {
                    *prev = line.clone();
                    continue;
                }
            }
            back.push(line.clone());
        }
    }

    fn block_tag(&self) -> Option<&'xml str> {
        let line = self.data.trim();
        let body = line.strip_prefix('<')?;
        if body.starts_with('?') || body.starts_with('!') || line.ends_with("/>") {
            return None;
        }
        let name_len = body
            .find(|c: char| c.is_whitespace() || c == '>')
            .unwrap_or(body.len());
        Some(&body[..name_len])
    }
}

#[derive(Eq, Clone, Copy, Debug)]
pub struct Tag<'xml> {
    norm: XmlLine<'xml>,
}

impl std::hash::Hash for Tag<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.norm.as_bytes());
    }
}

impl PartialEq for Tag<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.norm == other.norm
    }
}

impl std::hash::Hash for Line<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tag().hash(state);
    }
}

impl PartialEq for Line<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.tag() == other.tag()
    }
}
