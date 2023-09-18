use crate::*;

const CHAPTER_PATH: &str = "test_files/chapter/!original.xml";

lazy_static::lazy_static! {
    static ref CHAPTER: String = std::fs::read_to_string(CHAPTER_PATH).unwrap();
}

#[test]
fn merciful_emblems_no_ep_music() {
    let path_patches = [
        "test_files/chapter/dont-kill.xml",
        "test_files/chapter/no-ep-music.xml"
    ];
    let patches = path_patches.iter().map(|path| std::fs::read_to_string(path).unwrap()).collect::<Vec<_>>();

    let mut merger = Merger::new(&*CHAPTER);

    for patch in &patches {
        merger.patch(&patch);
    }

    let merged = merger.finalize();
    let ideal = read_fs_into_strs("test_files/chapter/examples/chapter-music-dont-kill-ideal-merge.xml");
    assert!(merged == ideal);
}