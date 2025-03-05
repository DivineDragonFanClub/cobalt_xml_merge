use crate::*;

const ASSET_TABLE_PATH: &str = "test_files/AssetTable/!original.xml";

lazy_static::lazy_static! {
    static ref ASSET_TABLE: String = std::fs::read_to_string(ASSET_TABLE_PATH).unwrap();
}

#[test]
fn boamo() {
    let path_patches = ["test_files/AssetTable/Boamo.xml"];
    let patches = path_patches
        .iter()
        .map(|path| std::fs::read_to_string(path).unwrap())
        .collect::<Vec<_>>();
    let mut merger = Merger::new(&*ASSET_TABLE);

    for patch in &patches {
        merger.patch(patch);
    }
    let merged = merger.finalize();

    let boamo = read_fs_into_strs("test_files/AssetTable/Boamo.xml");
    compare_non_whitespace(merged.join("\n"), boamo.join("\n")).test();
}

#[test]
fn playableboss() {
    let path_patches = ["test_files/AssetTable/PlayableBoss.xml"];
    let patches = path_patches
        .iter()
        .map(|path| std::fs::read_to_string(path).unwrap())
        .collect::<Vec<_>>();
    let mut merger = Merger::new(&*ASSET_TABLE);

    for patch in &patches {
        merger.patch(&patch);
    }

    let merged = merger.finalize();

    let playableboss = read_fs_into_strs("test_files/AssetTable/PlayableBoss.xml");
    compare_non_whitespace(merged.join("\n"), playableboss.join("\n")).test();
}

use crate::read_fs_into_strs;

#[test]
fn boamo_playableboss() {
    let path_patches = [
        "test_files/AssetTable/Boamo.xml",
        "test_files/AssetTable/PlayableBoss.xml",
    ];
    let patches = path_patches
        .iter()
        .map(|path| std::fs::read_to_string(path).unwrap())
        .collect::<Vec<_>>();

    let mut merger = Merger::new(&*ASSET_TABLE);

    for patch in &patches {
        merger.patch(&patch);
    }

    let merged = merger.finalize();
    let boamo_playableboss = read_fs_into_strs("test_files/AssetTable/examples/Boamo_Playableboss.xml");
    compare_non_whitespace(merged.join("\n"), boamo_playableboss.join("\n")).test();
}

#[test]
fn boamo_siegfried() {
    let path_patches = [
        "test_files/AssetTable/Boamo2.xml",
        "test_files/AssetTable/Siegfried.xml",
    ];
    let patches = path_patches
        .iter()
        .map(|path| std::fs::read_to_string(path).unwrap())
        .collect::<Vec<_>>();

    let mut merger = Merger::new(&*ASSET_TABLE);

    for patch in &patches {
        merger.patch(&patch);
    }

    let merged = merger.finalize();
    let boamo_playableboss = read_fs_into_strs("test_files/AssetTable/examples/Boamo_Siegfried.xml");
    compare_non_whitespace(merged.join("\n"), boamo_playableboss.join("\n")).test();
}
