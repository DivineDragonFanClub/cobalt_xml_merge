use crate::*;

#[derive(Eq, Clone, Debug)]
pub struct Line<'xml> {
    data: XmlLine<'xml>,
    pub deleted: bool,
    crc32: u32,
    pub front: Option<Vec<Line<'xml>>>,
    pub back: Option<Vec<Line<'xml>>>,
}

impl<'xml> Line<'xml> {
    // line with no content, set to deleted by default so it is ignored
    pub fn empty() -> Self {
        Self {
            data: "",
            deleted: true,
            crc32: 0,
            front: None,
            back: None,
        }
    }

    /// trims whitespace from the input
    pub fn new(original: XmlLine<'xml>) -> Self {
        let crc32 = crc32fast::hash(original.trim_end_matches("/>").trim().as_bytes());
        Self {
            data: original,
            deleted: false,
            crc32,
            front: None,
            back: None,
        }
    }

    /// Converts the line into a hunk of lines.
    ///
    /// `[front3, front2, front1, original, back3, back2, back1]`
    ///
    /// meaning both appending and prepending retains the mod loading order
    pub fn to_hunk(self) -> Vec<XmlLine<'xml>> {
        let size_hint = {
            let mut size = 0;
            if let Some(v) = self.front.as_ref() {
                size += v.len();
            }
            if let Some(v) = self.back.as_ref() {
                size += v.len();
            }
            if !self.deleted {
                size += 1;
            }
            size
        };
        let mut hunk = Vec::with_capacity(size_hint);
        if let Some(v) = self.front {
            for line in v.into_iter().rev() {
                if line.deleted {
                    continue;
                }
                hunk.push(line.data);
            }
        }
        if !self.deleted {
            hunk.push(self.data);
        }
        if let Some(v) = self.back {
            for line in v.into_iter().rev() {
                if line.deleted {
                    continue;
                }
                hunk.push(line.data);
            }
        }
        hunk
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
}

impl std::hash::Hash for Line<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.crc32);
    }
}

impl PartialEq for Line<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.crc32 == other.crc32 {
            return true;
        }

        matches!(compare_non_whitespace(self.data, other.data), CompareResult::Equal)
    }
}
