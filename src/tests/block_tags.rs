use crate::*;

#[test]
fn two_mods_editing_the_same_sheet_opener() {
    let base = r#"
        <Book Count="1">
            <Sheet Name="Item" Count="2">
                <Data>
                    <Param Iid="IID_Sword" Power="5" />
                    <Param Iid="IID_Axe" Power="7" />
                </Data>
            </Sheet>
        </Book>
        "#;
    let patches = &[
        r#"
        <Book Count="1">
            <Sheet Name="Item" Count="3">
                <Data>
                    <Param Iid="IID_Sword" Power="5" />
                    <Param Iid="IID_Axe" Power="7" />
                    <Param Iid="IID_ModA" Power="9" />
                </Data>
            </Sheet>
        </Book>
        "#,
        r#"
        <Book Count="1">
            <Sheet Name="Item" Count="4">
                <Data>
                    <Param Iid="IID_Sword" Power="5" />
                    <Param Iid="IID_Axe" Power="7" />
                    <Param Iid="IID_ModB1" Power="1" />
                    <Param Iid="IID_ModB2" Power="2" />
                </Data>
            </Sheet>
        </Book>
        "#,
    ];

    let mut merger = Merger::new(base);
    for patch in patches {
        merger.patch(patch);
    }
    let merged = merger.finalize_string();

    assert_eq!(merged.matches("<Sheet ").count(), 1, "duplicated sheet opener:\n{merged}");
    assert_eq!(merged.matches("</Sheet>").count(), 1, "{merged}");

    for iid in ["IID_Sword", "IID_Axe", "IID_ModA", "IID_ModB1", "IID_ModB2"] {
        assert!(merged.contains(iid), "lost {iid}:\n{merged}");
    }
}

#[test]
fn added_blocks_are_kept() {
    let base = r#"
        <Book Count="1">
            <Sheet Name="Item" Count="1">
                <Param Iid="IID_Sword" />
            </Sheet>
        </Book>
        "#;
    let patches = &[r#"
        <Book Count="2">
            <Sheet Name="Item" Count="1">
                <Param Iid="IID_Sword" />
            </Sheet>
            <Sheet Name="Extra" Count="1">
                <Param Iid="IID_New" />
            </Sheet>
        </Book>
        "#];

    let mut merger = Merger::new(base);
    for patch in patches {
        merger.patch(patch);
    }
    let merged = merger.finalize_string();

    assert_eq!(merged.matches("<Sheet ").count(), 2, "{merged}");
    assert_eq!(merged.matches("</Sheet>").count(), 2, "{merged}");
    assert!(merged.contains("IID_New"), "{merged}");
}
