use std::collections::{VecDeque, HashSet};

use differ::{Differ, Tag};


#[cfg(test)]
mod tests;


fn read_into_lines(path: &str) -> Vec<String> {
    let file = std::fs::read_to_string(path).unwrap();
    file.lines().map(|s| s.to_owned()).collect::<Vec<_>>()
}

pub fn two_d_array_merge(base: &str, patches: &[&str]) -> Vec<String> {
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

fn apply_diff(span: differ::Span, lines_2d: &mut Vec<VecDeque<String>>, rhs: &[String], is_deleted: &mut HashSet<usize>) {
    match span.tag {
        // the lowest entry will be the first mod in the list
        Tag::Insert => insert(lines_2d, &span, rhs),
        //
        Tag::Delete => delete(&span, is_deleted, lines_2d),
        // as is, mods modifying the same line will duplicate
        // the lowest entry will be from the last mod in the list
        Tag::Replace => replace(span, is_deleted, lines_2d, rhs),
        _ => {}
    }

    return;

    fn delete(span: &differ::Span, is_deleted: &mut HashSet<usize>, lines_2d: &mut [VecDeque<String>]) {
        for i in span.a_start..span.a_end {
            if is_deleted.get(&i).is_some() { continue; }
            let entry = lines_2d.get_mut(i).expect("out of bounds");
            entry.pop_back();
            is_deleted.insert(i);
        }
    }

    fn insert(lines_2d: &mut [VecDeque<String>], span: &differ::Span, rhs: &[String]) {
        let entry = lines_2d.get_mut(span.a_start).expect("out of bounds");
        for e in rhs[span.b_start..span.b_end].iter().rev() {
            entry.push_front(e.clone());
        }
    }

    fn replace(span: differ::Span, is_deleted: &mut HashSet<usize>, lines_2d: &mut [VecDeque<String>], rhs: &[String]) {
        delete(&span, is_deleted, lines_2d);

        // append replacing lines after insertions
        let entry = lines_2d.get_mut(span.a_start).expect("out of bounds");
        for e in rhs[span.b_start..span.b_end].iter() {
            entry.push_back(e.clone());
        }
    }
}








