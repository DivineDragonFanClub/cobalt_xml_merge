use std::borrow::Cow;

pub use merger::Merger;
pub use line::Line;

#[cfg(test)]
mod tests;
mod line;
mod merger;

type XmlLine<'a> = Cow<'a, str>; // can also use cow

#[cfg(test)]
pub fn read_fs_into_strs<'a>(path: impl AsRef<str>) -> Vec<String> {
    let file = std::fs::read_to_string(path.as_ref()).unwrap();
    slice(&file, |s| s.to_owned())
}


/// Merges a base string with a patch string.
/// 
/// Trims whitespace on each line.
pub fn merge_all<T: AsRef<str>>(base: T, patches: &[T]) -> String {
    let mut merger = Merger::new(base.as_ref());
    for patch in patches {
        merger.patch(patch.as_ref());
    }
    merger.finalize_string()
}

#[inline]
fn slice<'a, T>(s: &'a str, map: impl Fn(&'a str) -> T) -> Vec<T> {
    let mut v = Vec::with_capacity(1024); // hint size to avoid some reallocations
    let mut last_pos = 0;
    for pos in memchr::memchr_iter(b'>', s.as_bytes()) {
        let s = &s[last_pos..=pos];
        last_pos = pos + 1;
        let s = s.trim_start_matches(|c| c != '<');
        if s.is_empty() { continue };
        if s.starts_with("<!") { continue }; // skip comments
        if s.starts_with("<") {
            v.push(map(s));
        };
    }
    v
}

/// Converts a string into a vector of lines. Trims whitespace from each line.
pub fn str_to_lines(s: &'_ str) -> Vec<Line<'_>> {
    slice(s, |s| Line::new(Cow::Borrowed(s)))
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