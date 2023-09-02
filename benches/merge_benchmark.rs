use cobalt_xml_merge::{two_d_array_merge, read_into_lines};
use criterion::{Criterion, criterion_group, criterion_main};

const ASSET_TABLE: &str = "test_files/AssetTable/!original.xml";

criterion_group!(benches, bench_add_two);
criterion_main!(benches);

fn bench_add_two(c: &mut Criterion) {
    c.bench_function("add_two", |b| {
        b.iter(|| {
            let patches = [
                "test_files/AssetTable/Boamo.xml",
                "test_files/AssetTable/PlayableBoss.xml"
            ];
            let merged = two_d_array_merge(ASSET_TABLE, &patches);
            let boamo_playableboss = read_into_lines("test_files/AssetTable/examples/Boamo_Playableboss.xml");
            assert!(merged == boamo_playableboss);
        });
    });
}