use std::collections::{VecDeque, HashSet};

use differ::{Differ, Tag};


#[cfg(test)]
mod tests;


pub fn read_into_lines(path: &str) -> Vec<String> {
    let file = std::fs::read_to_string(path).unwrap();
    file.lines().map(|s| s.to_owned()).collect::<Vec<_>>()
}

pub fn two_d_array_merge<'a>(lhs: &'a [String], patches: &'a [&[String]]) -> Vec<&'a str> {

    let mut lines_2d = lhs.iter().map(|line| VecDeque::from([line.as_str()])).collect::<Vec<_>>();
    lines_2d.push(VecDeque::new()); 
    let mut is_deleted = HashSet::new(); // keep track of deleted indices

    for rhs in patches {
        let differ = Differ::new(lhs, &rhs);
        for span in differ.spans() {
            match span.tag {
                // Tag::Equal => print_lines('=', &original[span.a_start..span.a_end]),
                Tag::Insert => {
                    let entry = lines_2d.get_mut(span.a_start).expect("out of bounds");
                    for e in rhs[span.b_start..span.b_end].iter().rev() {
                        entry.push_front(e);
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
                        entry.push_back(e);
                    }
                }
                _ => {}
            }
        }
    }

    lines_2d.into_iter().flatten().collect()
}