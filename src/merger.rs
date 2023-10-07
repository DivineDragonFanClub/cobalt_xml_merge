use std::cmp::Ordering;

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
                    let line = &mut self.merge_arena[span.a_start];
                    line.insert_above(&patch[span.b_start..span.b_end]);
                },
                Tag::Delete => {
                    self.merge_arena[span.a_start..span.a_end].iter_mut().for_each(|line| line.deleted = true);
                },
                Tag::Replace => {
                    self.merge_arena[span.a_start..span.a_end].iter_mut().for_each(|line| line.deleted = true);
                    let line = &mut self.merge_arena[span.a_start];
                    line.insert_below(&patch[span.b_start..span.b_end]);
                    // for (a, b) in (span.a_start..span.a_end).zip(span.b_start..span.b_end) {
                    //     let line = &mut self.merge_arena[a];
                    //     let patch_line = &patch[b];
                    //     line.patch_params(patch_line);
                    // }

                    // // clean up trailing lines from either span
                    // let a_len = span.a_end - span.a_start;
                    // let b_len = span.b_end - span.b_start;

                    // match Ord::cmp(&a_len, &b_len) {
                    //     // append to the end of a
                    //     Ordering::Less => {
                    //         let line = &mut self.merge_arena[span.a_end-1];
                    //         line.insert_below(&patch[span.b_start+a_len..span.b_end]);
                    //     },
                    //     // delete trailing a lines
                    //     Ordering::Greater => {
                    //         self.merge_arena[span.a_start+b_len..span.a_end].iter_mut().for_each(|line| line.deleted = true);
                    //     },
                    //     Ordering::Equal => {}
                    // }

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