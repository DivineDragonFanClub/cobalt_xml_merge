use crate::*;

#[test]
fn basic() {
    let base = r#"
        <AssetTable>
            <Param Name="Vander" Type="Jagen" Path="1" />
        </AssetTable>
        "#;
    let patches = &[
        r#"
        <AssetTable>
            <Param Name="Vander" Type="Oifey" Path="1" />
        </AssetTable>
        "#,
        r#"
        <AssetTable>
            <Param Name="Vander" Type="Jagen" Path="2" />
        </AssetTable>
        "#,
        r#"
        <AssetTable>
            <Param Name="Vanderu" Type="Jagen" Path="1" />
        </AssetTable>
        "#,
    ];

    let mut merger = Merger::new(base);
    for patch in patches {
        merger.patch(patch);
    }

    assert!(merger.finalize_string() == Merger::new(r#"
        <AssetTable>
            <Param Name="Vanderu" Type="Oifey" Path="2" />
        </AssetTable>
    "#).finalize_string());
}