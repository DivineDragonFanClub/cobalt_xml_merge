use std::hash::{Hash, Hasher};

use cobalt_xml_merge::*;
use differ::{Differ, Tag};

fn main() {
    // filter_test(r"test_files\chapter\!original.xml");
    // filter_test(r"test_files\Shop\!original.xml");
    // filter_test(r"test_files\AssetTable\!original.xml");
    // let a = r#"<Param Name="hello yeah" Type="world" Path="" />"#;
    // let values = find_values(a);
    // println!("{:?}", values);
    let base = std::fs::read_to_string(r"test_files\AssetTable\!original.xml").unwrap();
    let boamo = std::fs::read_to_string(r"test_files\AssetTable\Boamo.xml").unwrap();
    let boss = std::fs::read_to_string(r"test_files\AssetTable\PlayableBoss.xml").unwrap();

    let instant = std::time::Instant::now();
    let patches = &[boamo, boss];
    let merger = Merger::new(&base, patches);
    merger.diff();
    println!("merge took {:?}", instant.elapsed());
}

fn filter_test(path: impl AsRef<str>) {
    let base_assettable = std::fs::read_to_string(path.as_ref()).unwrap();
    let sw = std::time::Instant::now();
    let tags = find_tags(&base_assettable);
    println!("find_tags took {:?}", sw.elapsed());
    let content = tags.into_iter().map(|tag| match tag {
        XmlTag::Raw(s) => s,
        XmlTag::Empty(s) => s,
        _ => "",
    }).collect::<Vec<_>>();
    let content = content.concat();
    let pretty = prettify_xml(&content).unwrap();
    compare_non_whitespace(base_assettable, pretty).test()
}

struct Merger<'file, 'patches> {
    base: Vec<PatchItem<'file>>,
    patches: Vec<Vec<PatchItem<'patches>>>,
    merge_arena: Vec<PatchItem<'file>>
}

impl<'file, 'patches> Merger<'file, 'patches> {
    fn new(base: &'file impl AsRef<str>, patches: &'patches [impl AsRef<str>]) -> Self {
        let base = find_tags(base.as_ref()).into_iter().map(PatchItem::new).collect::<Vec<_>>();
        // read patches
        let patches = patches.iter().map(|p| {
            find_tags(p.as_ref())
                .into_iter()
                .map(PatchItem::new)
                .collect()
        }).collect();
        Self {
            merge_arena: base.clone(),
            base,
            patches,
        }
    }

    fn diff(&self) {
        let sw = std::time::Instant::now();
        for patch in &self.patches {
            diff(&self.base, patch)
        }
        println!("diff took {:?}", sw.elapsed());
    }
}

fn diff<T: PartialEq>(left: &[T], right: &[T]) {

}

type Span = std::ops::Range<usize>;

enum Hunk {
    Equal,
    Delete(usize),
    Insert { index: usize, span: Span },
    Replace { left: Span , right: Span },
}

#[derive(Clone, Debug)]
enum XmlTag<'data> {
    Raw(&'data str),
    Empty(&'data str),
    DeadSpace,
}

#[derive(Clone, Debug)]
struct PatchItem<'tag> {
    tag: XmlTag<'tag>,
    hash: u32,
}

impl<'tag> PatchItem<'tag> {
    fn new(tag: XmlTag<'tag>) -> Self {
        let hash = match tag {
            XmlTag::Empty(s) => crc32fast::hash(s.as_bytes()),
            _ => 0,
        };
        Self {
            tag,
            hash,
        }
    }
}

impl PartialEq for PatchItem<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

/// Finds all tags in the xml string. Ignoring comments.
fn find_tags<'a>(xml: &'a str) -> Vec<XmlTag<'a>> {
    // all tags end with >
    let splits = xml.split_inclusive('>');
    let mut tags = vec![];
    for s in splits {
        // everything before <
        let s = s.trim_start_matches(|c| c != '<');
        // now we have <...>

        // couple things to skip
        if s.is_empty() { continue }; // Everything that isn't <...> will be cut out here
        if s.len() == 2 { continue }; // skip pure <>

        // see if 2nd last char is / to determine if it is an empty tag
        if s.chars().rev().nth(1) == Some('/') {
            tags.push(XmlTag::Empty(s));
        } else {
            tags.push(XmlTag::Raw(s));
        }

        // dead space so when we diff, hunks never overlap
        tags.push(XmlTag::DeadSpace);
    }

    return tags;
}

fn find_values<'a>(entry: &'a str) -> Vec<&'a str> {
    let mut values = vec![];
    let mut iter = entry.split('"');
    iter.next(); // skip first group <Param Name=
    while let Some(value) = iter.next() {
        values.push(value);
        iter.next(); // skip next ` Attribute=`
    }
    return values;
}