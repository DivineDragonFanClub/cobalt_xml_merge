use std::{collections::{VecDeque, HashSet}, rc::Rc};

use differ::{Differ, Tag};


#[cfg(test)]
mod tests;

type Line = Rc<str>;

pub fn read_into_lines(path: &str) -> Vec<Line> {
    let file = std::fs::read_to_string(path).unwrap();
    file.lines().map(|s| Rc::from(s)).collect::<Vec<_>>()
}

pub fn two_d_array_merge(base: &str, patches: &[&str]) -> Vec<Line> {
    let base = read_into_lines(base);
    let mut lines_2d = base.iter().map(|line| VecDeque::from([line.clone()])).collect::<Vec<_>>();
    lines_2d.push(VecDeque::new()); // end of file appended lines will be stored here
    let mut is_deleted = HashSet::new(); // keep track of deleted indices

    for path in patches {
        let patch = read_into_lines(path);
        let differ = Differ::new(&base, &patch);
        for span in differ.spans() {
            apply_diff(span, &mut lines_2d, &patch, &mut is_deleted);
        }
    }

    lines_2d.into_iter().flatten().collect()
}

fn apply_diff(span: differ::Span, lines_2d: &mut Vec<VecDeque<Line>>, rhs: &[Line], is_deleted: &mut HashSet<usize>) {
    match span.tag {
        Tag::Insert => insert(lines_2d, &span, rhs),
        Tag::Delete => delete(&span, is_deleted, lines_2d),
        Tag::Replace => replace(span, is_deleted, lines_2d, rhs),
        _ => {}
    }

    return;

    /// deletes original lines within a span
    fn delete(span: &differ::Span, is_deleted: &mut HashSet<usize>, lines_2d: &mut [VecDeque<Line>]) {
        for i in span.a_start..span.a_end {
            if is_deleted.get(&i).is_some() { continue; }
            let entry = lines_2d.get_mut(i).expect("out of bounds");
            entry.pop_back();
            is_deleted.insert(i);
        }
    }

    /// the lowest entry will be the first mod in the list
    /// 
    /// insertion order is:
    /// 
    /// `[mod3, mod2, mod1, original]`
    fn insert(lines_2d: &mut [VecDeque<Line>], span: &differ::Span, rhs: &[Line]) {
        let entry = lines_2d.get_mut(span.a_start).expect("out of bounds");
        for e in rhs[span.b_start..span.b_end].iter().rev() {
            entry.push_front(e.clone());
        }
    }

    /// as is, mods modifying the same line will duplicate
    /// 
    /// the lowest entry will be from the last mod in the list
    /// insertion order is:
    /// 
    /// `[original, mod1, mod2, mod3]`
    fn replace(span: differ::Span, is_deleted: &mut HashSet<usize>, lines_2d: &mut [VecDeque<Line>], rhs: &[Line]) {
        delete(&span, is_deleted, lines_2d);

        // append replacing lines after insertions
        let entry = lines_2d.get_mut(span.a_start).expect("out of bounds");
        for e in rhs[span.b_start..span.b_end].iter() {
            entry.push_back(e.clone());
        }
    }
}








