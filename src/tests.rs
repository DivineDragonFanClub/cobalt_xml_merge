use super::*;

const ASSET_TABLE: &str = "test_files/AssetTable/!original.xml";

#[test]
fn boamo() {
    let patches = [
        "test_files/AssetTable/Boamo.xml"
    ];
    let merged = two_d_array_merge(ASSET_TABLE, &patches);
    let boamo = read_into_lines("test_files/AssetTable/Boamo.xml");
    assert!(merged == boamo);
}

#[test]
fn playableboss() {
    let patches = [
        "test_files/AssetTable/PlayableBoss.xml"
    ];
    let merged = two_d_array_merge(ASSET_TABLE, &patches);
    let playableboss = read_into_lines(r"test_files/AssetTable/PlayableBoss.xml");
    assert!(merged == playableboss);
}

#[test]
fn boamo_playableboss() {
    let patches = [
        "test_files/AssetTable/Boamo.xml",
        "test_files/AssetTable/PlayableBoss.xml"
    ];
    let merged = two_d_array_merge(ASSET_TABLE, &patches);
    let boamo_playableboss = read_into_lines("test_files/AssetTable/examples/Boamo_Playableboss.xml");
    assert!(merged == boamo_playableboss);
}