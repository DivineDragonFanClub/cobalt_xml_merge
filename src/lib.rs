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
    v
}

/// Converts a string into a vector of lines. Trims whitespace from each line.
pub fn str_to_lines(s: &'_ str) -> Vec<Line<'_>> {
    slice(s, |s| Line::new(Cow::Borrowed(s)))
}









