use std::{
    borrow::Cow,
    sync::LazyLock,
};

use cobalt_xml_merge::*;
use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};

const ASSET_TABLE_PATH: &str = "test_files/AssetTable/!original.xml";

static ASSET_TABLE: LazyLock<String> = LazyLock::new(|| std::fs::read_to_string(ASSET_TABLE_PATH).unwrap());

criterion_group!(benches, bench_add_two);
criterion_main!(benches);

fn bench_add_two(c: &mut Criterion) {
    c.bench_function("boamo + playable bosses merge", |b| {
        let path_patches = [
            "test_files/AssetTable/Boamo.xml",
            "test_files/AssetTable/PlayableBoss.xml",
        ];
        let patches = path_patches
            .iter()
            .map(|path| std::fs::read_to_string(path).unwrap())
            .collect::<Vec<_>>();
        let table = &*ASSET_TABLE;

        let boamo_playableboss = read_fs_into_strs("test_files/AssetTable/examples/Boamo_Playableboss.xml");
        let mut merged = vec![];
        b.iter(|| {
            let mut merger = Merger::new(table);

            for patch in &patches {
                merger.patch(patch);
            }

            merged = merger.finalize();
        });
        // right handside is missing xml
        compare_non_whitespace(&merged[1..].join("\n"), boamo_playableboss.join("\n")).test();
    });
}

pub fn read_fs_into_strs<'a>(path: &'a str) -> Vec<Cow<'a, str>> {
    let file = std::fs::read_to_string(path).unwrap();
    slice(&file, |s| Cow::Owned(s.to_owned()))
}

fn slice<'a, T>(s: &'a str, map: impl Fn(&'a str) -> T) -> Vec<T> {
    let lines = s.lines();
    let mut v = Vec::with_capacity(lines.size_hint().0); // hint size to avoid some reallocations
    for slice in lines {
        let s = slice.trim();
        if s.is_empty() {
            continue;
        };
        if s.starts_with("<!") {
            continue;
        }; // skip comments
        if s.starts_with("<") {
            v.push(map(s));
        };
    }
    v
}
