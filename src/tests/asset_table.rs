use crate::*;

const ASSET_TABLE_PATH: &str = "test_files/AssetTable/!original.xml";

lazy_static::lazy_static! {
    static ref ASSET_TABLE: String = std::fs::read_to_string(ASSET_TABLE_PATH).unwrap();
}

#[test]
fn boamo() {
    let path_patches = [
        "test_files/AssetTable/Boamo.xml"
    ];
    let patches = path_patches.iter().map(|path| std::fs::read_to_string(path).unwrap()).collect::<Vec<_>>();
    let mut merger = Merger::new(&*ASSET_TABLE);

    for patch in &patches {
        merger.patch(patch);
    }
    let merged = merger.finalize();

    let boamo = read_fs_into_strs("test_files/AssetTable/Boamo.xml");
    assert!(merged == boamo);
}

#[test]
fn playableboss() {
    let path_patches = [
        "test_files/AssetTable/PlayableBoss.xml"
    ];
    let patches = path_patches.iter().map(|path| std::fs::read_to_string(path).unwrap()).collect::<Vec<_>>();
    let mut merger = Merger::new(&*ASSET_TABLE);

    for patch in &patches {
        merger.patch(&patch);
    }

    let merged = merger.finalize();

    let playableboss = read_fs_into_strs("test_files/AssetTable/PlayableBoss.xml");
    assert!(merged == playableboss);
}

use crate::read_fs_into_strs;

#[test]
fn boamo_playableboss() {
    let path_patches = [
        "test_files/AssetTable/Boamo.xml",
        "test_files/AssetTable/PlayableBoss.xml"
    ];
    let patches = path_patches.iter().map(|path| std::fs::read_to_string(path).unwrap()).collect::<Vec<_>>();

    let mut merger = Merger::new(&*ASSET_TABLE);

    for patch in &patches {
        merger.patch(&patch);
    }

    let merged = merger.finalize();
    let boamo_playableboss = read_fs_into_strs("test_files/AssetTable/examples/Boamo_Playableboss.xml");
    assert!(merged == boamo_playableboss);
}