use differ::{Tag, Differ};

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
        let differ = Differ::new(&self.base, &patch);
        for span in differ.spans() {
            match span.tag {
                Tag::Insert => {
                    let line = self.merge_arena.get_mut(span.a_start).expect("out of bounds");
                    line.front.extend_from_slice(&patch[span.b_start..span.b_end]);
                },
                Tag::Delete => {
                    self.merge_arena[span.a_start..span.a_end].iter_mut().for_each(|line| line.deleted = true);
                },
                Tag::Replace => {
                    self.merge_arena[span.a_start..span.a_end].iter_mut().for_each(|line| line.deleted = true);
                    let line = self.merge_arena.get_mut(span.a_start).expect("out of bounds");
                    line.back.extend_from_slice(&patch[span.b_start..span.b_end]);
                },
                _ => {}
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