use std::hash::Hasher;

use differ::{Tag, Differ};

use crate::line::Line;
use crate::*;

pub struct Merger<'xml> {
    base: Vec<XmlLine<'xml>>,
}

impl<'xml> Merger<'xml> {

    pub fn new(base: &'xml str) -> Self {
        let base = tag_spans(base, base.len() / 1_000);
        //Self::from(base)
        todo!();
    }

    /// joins the lines with line breaks
    pub fn finalize_string(self) -> String {
        todo!();
    }
}

struct XmlLine<'xml> {
    pub tag: &'xml str,
    pub crc32: u32,
}

impl<'xml> XmlLine<'xml> {
    fn new(tag: &'xml str) -> Self {
        let mut hahser = crc32fast::Hasher::new();
        Self {
            tag,
            crc32: crc32fast::hash(tag.as_bytes()),
        }
    }
}

fn tag_spans<'a>(string: &'a str, size_hint: usize) -> Vec<XmlLine<'a>> {
    let mut v = Vec::with_capacity(size_hint); // hint size to avoid some reallocations
    for line in string.lines() {
        if line.is_empty() { continue };
        v.push(XmlLine::new(line));
    }
    v
}