use super::*;

fn original() -> Vec<String> {
    read_into_lines(r"test_files\AssetTable\!original.xml")
}

#[test]
fn boamo() {
    let original = original();
    let boamo = read_into_lines(r"test_files\AssetTable\Boamo.xml");
    let patches = vec![&boamo[..]];
    let merged = two_d_array_merge(&original, &patches);
    assert!(merged == boamo);
}

#[test]
fn playableboss() {
    let original = original();
    let playableboss = read_into_lines(r"test_files\AssetTable\PlayableBoss.xml");
    let patches = vec![&playableboss[..]];
    let merged = two_d_array_merge(&original, &patches);
    assert!(merged == playableboss);
}

#[test]
fn boamo_playableboss() {
    let original = original();
    let boamo = read_into_lines(r"test_files\AssetTable\Boamo.xml");
    let playableboss = read_into_lines(r"test_files\AssetTable\PlayableBoss.xml");
    let patches = vec![&boamo[..], &playableboss[..]];
    let merged = two_d_array_merge(&original, &patches);
    let example = read_into_lines(r"test_files\AssetTable\examples\Boamo_PlayableBoss.xml");
    assert!(merged == example);
}