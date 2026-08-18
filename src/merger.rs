use imara_diff::intern::{
    InternedInput,
    Interner,
    Token,
};

use crate::{
    line::{
        Line,
        Tag,
    },
    *,
};

pub struct Merger<'xml> {
    merge_arena: Vec<Line<'xml>>,
    input: InternedInput<Tag<'xml>>,
    base_tokens: Token,
}

impl<'xml> Merger<'xml> {
    pub fn new(base: &'xml str) -> Self {
        let base = str_to_lines(base);
        Self::from(base)
    }

    pub fn from(base: Vec<Line<'xml>>) -> Self {
        let mut merge_arena = base.clone();
        merge_arena.push(Line::empty()); // end of file appended lines will be stored here

        let mut input = InternedInput {
            before: Vec::with_capacity(base.len()),
            after: Vec::with_capacity(base.len()),
            interner: Interner::new(base.len() * 2),
        };
        input.update_before(base.iter().map(Line::tag));
        let base_tokens = Token(input.interner.num_tokens());

        Self {
            merge_arena,
            input,
            base_tokens,
        }
    }

    pub fn patch(&mut self, patch: &'xml str) {
        let patch = str_to_lines(patch);
        self.patch_lines(patch);
    }

    pub fn patch_lines(&mut self, patch: Vec<Line<'xml>>) {
        let mut diffkinds = vec![];

        self.input.update_after(patch.iter().map(Line::tag));
        imara_diff::diff(
            imara_diff::Algorithm::Myers,
            &self.input,
            |before: std::ops::Range<u32>, after: std::ops::Range<u32>| {
                let diffkind = match (before.len(), after.len()) {
                    (0, 0) => unreachable!(),
                    (0, _) => {
                        DiffKind::Insert {
                            before: before.start,
                            after,
                        }
                    },
                    (_, 0) => DiffKind::Delete { before },
                    _ => DiffKind::Replace { before, after },
                };
                diffkinds.push(diffkind);
            },
        );
        self.input.interner.erase_tokens_after(self.base_tokens);

        for diffkind in diffkinds {
            fn range_into(range: std::ops::Range<u32>) -> std::ops::Range<usize> {
                range.start as usize..range.end as usize
            }
            match diffkind {
                DiffKind::Insert { before, after } => {
                    let line = &mut self.merge_arena[before as usize];
                    line.insert_above(&patch[range_into(after)]);
                },
                DiffKind::Delete { before } => {
                    self.merge_arena[range_into(before)]
                        .iter_mut()
                        .for_each(|line| line.deleted = true);
                },
                DiffKind::Replace { before, after } => {
                    let line_idx = before.start as usize;
                    self.merge_arena[range_into(before)]
                        .iter_mut()
                        .for_each(|line| line.deleted = true);
                    let line = &mut self.merge_arena[line_idx];
                    line.insert_below_overwrite(&patch[range_into(after)]);
                },
            }
        }
    }

    pub fn finalize(self) -> Vec<XmlLine<'xml>> {
        let mut out = Vec::with_capacity(self.merge_arena.iter().map(Line::hunk_len).sum());
        for line in self.merge_arena {
            line.write_hunk(&mut out);
        }
        out
    }

    /// joins the lines with line breaks
    pub fn finalize_string(self) -> String {
        self.finalize().join("\n")
    }
}

#[derive(Debug)]
enum DiffKind {
    Insert {
        before: u32,
        after: std::ops::Range<u32>,
    },
    Delete {
        before: std::ops::Range<u32>,
    },
    Replace {
        before: std::ops::Range<u32>,
        after: std::ops::Range<u32>,
    },
}
