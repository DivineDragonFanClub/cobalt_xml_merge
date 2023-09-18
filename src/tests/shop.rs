use crate::*;

const BASE_PATH: &str = "test_files/shop/!original.xml";

lazy_static::lazy_static! {
    static ref BASE: String = std::fs::read_to_string(BASE_PATH).unwrap();
}

#[test]
fn merciful_emblems_no_ep_music() {
    let path_patches = [
        "test_files/Shop/Boamo.xml",
        "test_files/Shop/Improve.xml"
    ];
    let patches = path_patches.iter().map(|path| std::fs::read_to_string(path).unwrap()).collect::<Vec<_>>();

    let mut merger = Merger::new(&*BASE);

    for patch in &patches {
        merger.patch(&patch);
    }

    let merged = merger.finalize();
    let ideal = read_fs_into_strs("test_files/Shop/examples/Boamo_Improve.xml");
    assert!(merged == ideal);
}