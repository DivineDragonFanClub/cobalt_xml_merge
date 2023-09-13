use crate::*;

#[test]
fn pre_append_sameline() {
    let base = r#"
        <AssetTable>
            <Asset Name="Vander" Type="Jagen" Path="1" />
            <Asset Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
        "#;
    let patches = &[
        r#"
        <AssetTable>
            <Asset Name="Alear" Type="Protag" Path="0" />
            <Asset Name="Vander" Type="Jagen" Path="1" />
            <Asset Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
        "#,
        r#"
        <AssetTable>
            <Asset Name="Veyle" Type="ProtagAlt" Path="0.1" />
            <Asset Name="Vander" Type="Jagen" Path="1" />
            <Asset Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
        "#,
        r#"
        <AssetTable>
            <Asset Name="Vander" Type="Jagen" Path="1" />
            <Asset Name="Framme" Type="monk" Path="2" />
            <Asset Name="Clanne" Type="mage" Path="3" />
            <Asset Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
        "#,
    ];

    let mut merger = Merger::new(base);
    for patch in patches {
        merger.patch(patch);
    }

    assert!(merger.finalize_string() == Merger::new(r#"
        <AssetTable>
            <Asset Name="Alear" Type="Protag" Path="0" />
            <Asset Name="Veyle" Type="ProtagAlt" Path="0.1" />
            <Asset Name="Vander" Type="Jagen" Path="1" />
            <Asset Name="Framme" Type="monk" Path="2" />
            <Asset Name="Clanne" Type="mage" Path="3" />
            <Asset Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
    "#).finalize_string());
}

#[test]
fn pre_append_sameline_empty_lines() {
    let base = r#"


        <AssetTable>

            <Asset Name="Vander" Type="Jagen" Path="1" />

            <Asset Name="Morb" Type="Goat" Path="69" />

        </AssetTable>
        "#;
    let patches = &[
        r#"
        <AssetTable>
            <Asset Name="Alear" Type="Protag" Path="0" />


            <Asset Name="Vander" Type="Jagen" Path="1" />
            <Asset Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
        "#,
        r#"
        <AssetTable>
            <Asset Name="Veyle" Type="ProtagAlt" Path="0.1" />
            <Asset Name="Vander" Type="Jagen" Path="1" />
            <Asset Name="Morb" Type="Goat" Path="69" />

        </AssetTable>


        "#,
        r#"
        <AssetTable>
            <Asset Name="Vander" Type="Jagen" Path="1" />
            <Asset Name="Framme" Type="monk" Path="2" />


            <Asset Name="Clanne" Type="mage" Path="3" />
            <Asset Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
        "#,
    ];

    let mut merger = Merger::new(base);
    for patch in patches {
        merger.patch(patch);
    }

    assert!(merger.finalize_string() == Merger::new(r#"
        <AssetTable>
            <Asset Name="Alear" Type="Protag" Path="0" />
            <Asset Name="Veyle" Type="ProtagAlt" Path="0.1" />
            <Asset Name="Vander" Type="Jagen" Path="1" />
            <Asset Name="Framme" Type="monk" Path="2" />
            <Asset Name="Clanne" Type="mage" Path="3" />
            <Asset Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
    "#).finalize_string());
}

#[test]
fn same_line_delte() {
    let base = r#"
    <AssetTable>
        <Asset Name="Alear" Type="Protag" Path="0" />
        <Asset Name="Veyle" Type="ProtagAlt" Path="0.1" />
        <Asset Name="Vander" Type="Jagen" Path="1" />
        <Asset Name="Framme" Type="monk" Path="2" />
        <Asset Name="Clanne" Type="mage" Path="3" />
        <Asset Name="Morb" Type="Goat" Path="69" />
    </AssetTable>"#;
    let patches = &[
        r#"
        <AssetTable>
            <Asset Name="Alear" Type="Protag" Path="0" />
            <Asset Name="Griss" Type="Maso" Path="0.2" />
            <Asset Name="Framme" Type="monk" Path="2" />
            <Asset Name="Clanne" Type="mage" Path="3" />
            <Asset Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
        "#,
        r#"
        <AssetTable>
            <Asset Name="Alear" Type="Protag" Path="0" />
            <Asset Name="Veyle" Type="ProtagAlt" Path="0.1" />
            <Asset Name="Framme" Type="monk" Path="2" />
            <Asset Name="Clanne" Type="mage" Path="3" />
            <Asset Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
        "#
    ];

    let mut merger = Merger::new(base);
    for patch in patches {
        merger.patch(patch);
    }
    assert!(merger.finalize_string() == Merger::new(r#"
        <AssetTable>
            <Asset Name="Alear" Type="Protag" Path="0" />
            <Asset Name="Griss" Type="Maso" Path="0.2" />
            <Asset Name="Framme" Type="monk" Path="2" />
            <Asset Name="Clanne" Type="mage" Path="3" />
            <Asset Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
        "#).finalize_string());
}