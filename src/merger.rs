use imara_diff::intern::TokenSource;

use crate::line::Line;
use crate::*;

pub struct Merger<'xml> {
    base: Vec<Line<'xml>>,
    merge_arena: Vec<Line<'xml>>
}

impl<'xml> Merger<'xml> {

    pub fn new(base: &'xml str) -> Self {
        let base = str_to_lines(base);
        Self::from(base)
    }

    pub fn from(base: Vec<Line<'xml>>) -> Self {
        let mut merge_arena = base.clone();
        merge_arena.push(Line::empty()); // end of file appended lines will be stored here
        Self {
            merge_arena,
            base,
        }
    }

    pub fn patch(&mut self, patch: &'xml str) {
        let patch = str_to_lines(patch);
        self.patch_lines(patch);
    }

    pub fn patch_lines(&mut self, patch: Vec<Line<'xml>>) {
        let mut diffkinds = vec![];


        let input = {
            let old = TagSource(self.base.iter());
            let new = TagSource(patch.iter());
            imara_diff::intern::InternedInput::new(old, new)
        };
        imara_diff::diff(imara_diff::Algorithm::Myers, &input, |before: std::ops::Range<u32> , after: std::ops::Range<u32>| {
            let diffkind = match (before.len(), after.len()) {
                (0, 0) => unreachable!(),
                (0, _) => DiffKind::Insert {
                    before: before.start,
                    after,
                },
                (_, 0) => DiffKind::Delete { before },
                _ => DiffKind::Replace { before, after },
            };
            diffkinds.push(diffkind);
        });

        for diffkind in diffkinds {
            fn range_into(range: std::ops::Range<u32>) -> std::ops::Range<usize> {
                range.start as usize..range.end as usize
            }
            match diffkind {
                DiffKind::Insert { before, after } => {
                    let line = &mut self.merge_arena[before as usize];
                    line.insert_above(&patch[range_into(after)]); 
                }
                DiffKind::Delete { before } => {
                    self.merge_arena[range_into(before)].iter_mut().for_each(|line| line.deleted = true);
                },
                DiffKind::Replace { before, after } => {
                    let line_idx = before.start as usize;
                    self.merge_arena[range_into(before)].iter_mut().for_each(|line| line.deleted = true);
                    let line = &mut self.merge_arena[line_idx];
                    line.insert_below(&patch[range_into(after)]);
                },
            }
        }
    }

    pub fn finalize(self) -> Vec<XmlLine<'xml>> {
        self.merge_arena.into_iter()
            .flat_map(|line| line.to_hunk())
            .collect()
    }

    /// joins the lines with line breaks
    pub fn finalize_string(self) -> String {
        self.finalize().join("\n")
    }
}

struct TagSource<'a, T: Sized>(std::slice::Iter<'a, T>);
impl<'xml, T: Sized + Eq + std::hash::Hash> TokenSource for TagSource<'xml, T> {
    type Token = &'xml T;

    type Tokenizer = std::slice::Iter<'xml, T>;

    fn tokenize(&self) -> Self::Tokenizer {
        std::slice::Iter::clone(&self.0)
    }

    fn estimate_tokens(&self) -> u32 {
        self.0.len() as u32
    }
}

#[derive(Debug)]
enum DiffKind {
    Insert { before: u32, after: std::ops::Range<u32> },
    Delete { before: std::ops::Range<u32> },
    Replace { before: std::ops::Range<u32>, after: std::ops::Range<u32> },
}