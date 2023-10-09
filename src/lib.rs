use std::borrow::Cow;

use anyhow::{Result, Context};
pub use merger::Merger;
pub use line::Line;
use quick_xml::{Reader, events::Event, Writer};

#[cfg(test)]
mod tests;
mod line;
mod merger;
mod atomic_patch;

/// takes in byte position, returns line and column number of the char
pub fn line_col_of(string: &str, byte_position: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    for (i, c) in string.char_indices() {
        if i == byte_position { break; }
        if c == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    return (line, col);
}

pub fn prettify_xml(xml: impl AsRef<str>) -> Result<String> {
    let xml = xml.as_ref();
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut writer = Writer::new_with_indent(Vec::new(), b'\t', 1);

    loop {
        match reader.read_event().with_context(|| {
            let (line, col) = line_col_of(xml,reader.buffer_position());
            let line_content = xml.lines().nth(line - 1).unwrap_or(xml);
            format!("line {line} col {col} content: \n {line_content}")
        })? {
            Event::Eof => break,
            Event::Comment(_) => continue,
            e => {
                writer.write_event(&e).with_context(|| {
                    let (line, col) = line_col_of(xml,reader.buffer_position());
                    format!("line {line} col {col} content: \n {e:?}")
                })?;
            }
        }
    }

    let xml = writer.into_inner();
    
    return Ok(String::from_utf8(xml)?);
}

type XmlLine<'a> = Cow<'a, str>; // can also use cow

#[cfg(test)]
pub fn read_fs_into_strs<'a>(path: impl AsRef<str>) -> Vec<String> {
    let file = std::fs::read_to_string(path.as_ref()).unwrap();
    return slice(&file, |s| s.to_owned());
}


/// Merges a base string with a patch string.
/// 
/// Trims whitespace on each line.
pub fn merge_all(base: impl AsRef<str>, patches: &[impl AsRef<str>]) -> String {
    let mut merger = Merger::new(base.as_ref());
    for patch in patches {
        merger.patch(patch.as_ref());
    }
    return merger.finalize_string();
}

#[inline]
fn slice<'a, T>(s: &'a str, map: impl Fn(&'a str) -> T) -> Vec<T> {
    let lines = s.split_inclusive(">");
    let mut v = Vec::with_capacity(lines.size_hint().0); // hint size to avoid some reallocations
    for slice in lines {
        let s = slice.trim();
        let s = s.trim_start_matches(|c| c != '<');
        if s.is_empty() { continue };
        if s.starts_with("<!") { continue }; // skip comments
        if s.starts_with("<") {
            v.push(map(s));
        };
    }
    return v;
}

/// Converts a string into a vector of lines. Trims whitespace from each line.
pub fn str_to_lines(s: &'_ str) -> Vec<Line<'_>> {
    return slice(s, |s| Line::new(Cow::Borrowed(s)));
}

type CharIndex = (usize, char);
#[derive(Debug)]
pub enum CompareResult {
    Equal,
    LeftOverflow(CharIndex),
    RightOverflow(CharIndex),
    NotEqualAt {left: CharIndex, right: CharIndex},
}

impl CompareResult {
    pub fn test(&self) {
        match self {
            CompareResult::Equal => {},
            _ => panic!("{self:?}"),
        }
    }
}

/// Compares two strings, ignoring whitespace.  
/// 
/// returns the byte position of first mimmatch.
pub fn compare_non_whitespace(lhs: impl AsRef<str>, rhs: impl AsRef<str>) -> CompareResult {
    let mut lhs = lhs.as_ref().char_indices();
    let mut rhs = rhs.as_ref().char_indices();
    loop {
        let l = next(&mut lhs);
        let r = next(&mut rhs);
        match (l, r) {
            (None, None) => return CompareResult::Equal,
            (None, Some(r)) => return CompareResult::RightOverflow(r),
            (Some(l), None) => return CompareResult::LeftOverflow(l),
            (Some(l), Some(r)) => {
                if l.1 != r.1 {
                    return CompareResult::NotEqualAt {left: l, right: r};
                }
            }
        }
    }

    fn next(iter: &mut impl Iterator<Item = (usize, char)>) -> Option<(usize, char)> {
        iter.skip_while(|(_, c)| match c {
                ' ' | '\n' | '\t' | '\r' | '\u{feff}' => {
                    true
                },
                _ => false
            })
            .next()
    }
}