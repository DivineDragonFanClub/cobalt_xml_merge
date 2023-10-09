#![allow(dead_code)]

mod asset_table;

mod theoretical;

// mod field_merge;

mod chapter;

mod shop;

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
fn compare_non_whitespace(lhs: impl AsRef<str>, rhs: impl AsRef<str>) -> CompareResult {
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