use std::collections::{VecDeque, HashSet};

use differ::{Differ, Tag};


#[cfg(test)]
mod tests;


pub fn read_into_lines(path: &str) -> Vec<String> {
    let file = std::fs::read_to_string(path).unwrap();
    file.lines().map(|s| s.to_owned()).collect::<Vec<_>>()
}

pub fn two_d_array_merge<'a>(base: &str, patches: &[&str]) -> Vec<String> {
    let lhs = read_into_lines(base);
    let mut lines_2d = lhs.iter().map(|line| VecDeque::from([line.to_owned()])).collect::<Vec<_>>();
    lines_2d.push(VecDeque::new()); // end of file appended lines will be stored here
    let mut is_deleted = HashSet::new(); // keep track of deleted indices

    for path in patches {
        let rhs = read_into_lines(path);
        let differ = Differ::new(&lhs, &rhs);
        for span in differ.spans() {
            match span.tag {
                // the lowest entry will be the first mod in the list
                Tag::Insert => {
                    let entry = lines_2d.get_mut(span.a_start).expect("out of bounds");
                    for e in rhs[span.b_start..span.b_end].iter().rev() {
                        entry.push_front(e.clone());
                    }
                }
                Tag::Delete => {
                    for i in span.a_start..span.a_end {
                        if is_deleted.get(&i).is_some() { continue; }
                        let entry = lines_2d.get_mut(i).expect("out of bounds");
                        entry.pop_back();
                        is_deleted.insert(i);
                    }
                }
                // as is, mods modifying the same line will duplicate
                // the lowest entry will be from the last mod in the list
                Tag::Replace => {
                    for i in span.a_start..span.a_end {
                        if is_deleted.get(&i).is_some() { continue; }
                        let entry = lines_2d.get_mut(i).expect("out of bounds");
                        entry.pop_back();
                        is_deleted.insert(i);
                    }

                    // append replacing lines after insertions
                    let entry = lines_2d.get_mut(span.a_start).expect("out of bounds");
                    for e in rhs[span.b_start..span.b_end].iter() {
                        entry.push_back(e.clone());
                    }
                }
                _ => {}
            }
        }
    }

    lines_2d.into_iter().flatten().collect()
}