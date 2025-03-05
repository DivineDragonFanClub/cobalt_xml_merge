use crate::*;

#[test]
fn pre_append_sameline() {
    let base = r#"
        <AssetTable>
            <Param Name="Vander" Type="Jagen" Path="1" />
            <Param Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
        "#;
    let patches = &[
        r#"
        <AssetTable>
            <Param Name="Alear" Type="Protag" Path="0" />
            <Param Name="Vander" Type="Jagen" Path="1" />
            <Param Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
        "#,
        r#"
        <AssetTable>
            <Param Name="Veyle" Type="ProtagAlt" Path="0.1" />
            <Param Name="Vander" Type="Jagen" Path="1" />
            <Param Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
        "#,
        r#"
        <AssetTable>
            <Param Name="Vander" Type="Jagen" Path="1" />
            <Param Name="Framme" Type="monk" Path="2" />
            <Param Name="Clanne" Type="mage" Path="3" />
            <Param Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
        "#,
    ];

    let mut merger = Merger::new(base);
    for patch in patches {
        merger.patch(patch);
    }

    assert!(
        merger.finalize_string()
            == Merger::new(
                r#"
        <AssetTable>
            <Param Name="Veyle" Type="ProtagAlt" Path="0.1" />
            <Param Name="Alear" Type="Protag" Path="0" />
            <Param Name="Vander" Type="Jagen" Path="1" />
            <Param Name="Framme" Type="monk" Path="2" />
            <Param Name="Clanne" Type="mage" Path="3" />
            <Param Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
    "#
            )
            .finalize_string()
    );
}

#[test]
fn pre_append_sameline_empty_lines() {
    let base = r#"


        <AssetTable>

            <Param Name="Vander" Type="Jagen" Path="1" />

            <Param Name="Morb" Type="Goat" Path="69" />

        </AssetTable>
        "#;
    let patches = &[
        r#"
        <AssetTable>
            <Param Name="Alear" Type="Protag" Path="0" />


            <Param Name="Vander" Type="Jagen" Path="1" />
            <Param Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
        "#,
        r#"
        <AssetTable>
            <Param Name="Veyle" Type="ProtagAlt" Path="0.1" />
            <Param Name="Vander" Type="Jagen" Path="1" />
            <Param Name="Morb" Type="Goat" Path="69" />

        </AssetTable>


        "#,
        r#"
        <AssetTable>
            <Param Name="Vander" Type="Jagen" Path="1" />
            <Param Name="Framme" Type="monk" Path="2" />


            <Param Name="Clanne" Type="mage" Path="3" />
            <Param Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
        "#,
    ];

    let mut merger = Merger::new(base);
    for patch in patches {
        merger.patch(patch);
    }

    assert!(
        merger.finalize_string()
            == Merger::new(
                r#"
        <AssetTable>
            <Param Name="Veyle" Type="ProtagAlt" Path="0.1" />
            <Param Name="Alear" Type="Protag" Path="0" />
            <Param Name="Vander" Type="Jagen" Path="1" />
            <Param Name="Framme" Type="monk" Path="2" />
            <Param Name="Clanne" Type="mage" Path="3" />
            <Param Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
    "#
            )
            .finalize_string()
    );
}

#[test]
fn same_line_delete() {
    let base = r#"
    <AssetTable>
        <Param Name="Alear" Type="Protag" Path="0" />
        <Param Name="Veyle" Type="ProtagAlt" Path="0.1" />
        <Param Name="Vander" Type="Jagen" Path="1" />
        <Param Name="Framme" Type="monk" Path="2" />
        <Param Name="Clanne" Type="mage" Path="3" />
        <Param Name="Morb" Type="Goat" Path="69" />
    </AssetTable>"#;
    let patches = &[
        r#"
        <AssetTable>
            <Param Name="Alear" Type="Protag" Path="0" />
            <Param Name="Griss" Type="Maso" Path="0.2" />
            <Param Name="Framme" Type="monk" Path="2" />
            <Param Name="Clanne" Type="mage" Path="3" />
            <Param Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
        "#,
        r#"
        <AssetTable>
            <Param Name="Alear" Type="Protag" Path="0" />
            <Param Name="Veyle" Type="ProtagAlt" Path="0.1" />
            <Param Name="Framme" Type="monk" Path="2" />
            <Param Name="Clanne" Type="mage" Path="3" />
            <Param Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
        "#,
    ];

    let mut merger = Merger::new(base);
    for patch in patches {
        merger.patch(patch);
    }
    assert!(
        merger.finalize_string()
            == Merger::new(
                r#"
        <AssetTable>
            <Param Name="Alear" Type="Protag" Path="0" />
            <Param Name="Griss" Type="Maso" Path="0.2" />
            <Param Name="Framme" Type="monk" Path="2" />
            <Param Name="Clanne" Type="mage" Path="3" />
            <Param Name="Morb" Type="Goat" Path="69" />
        </AssetTable>
        "#
            )
            .finalize_string()
    );
}

// #[test]
// fn chained_replace_tags() {
//     let base = r#"
//     <AssetTable>
//         <Param Name="Alear" Type="Protag" Path="0" />
//         <Param Name="Veyle" Type="ProtagAlt" Path="0.1" />
//         <Param Name="Vander" Type="Jagen" Path="1" />
//         <Param Name="Framme" Type="monk" Path="2" />
//         <Param Name="Clanne" Type="mage" Path="3" />
//         <Param Name="Morb" Type="Goat" Path="69" />
//     </AssetTable>"#;
//     let patches = &[
//         r#"
//         <AssetTable>
//             <Param Name="Alear" Type="Protag" Path="1" />
//             <Param Name="Griss" Type="Maso" Path="0.2" />
//             <Param Name="Veyle" Type="ProtagAlt" Path="0.3" />
//             <Param Name="Vander" Type="Jagen" Path="2" />
//             <Param Name="Framme" Type="monk" Path="4" />
//             <Param Name="Clanne" Type="mage" Path="69" />
//             <Param Name="Morb" Type="Goat" Path="70" />
//         </AssetTable>
//         "#,
//         r#"
//         <AssetTable>
//             <Param Name="Alear" Type="ProtagAlt" Path="0" />
//             <Param Name="Veyle" Type="Protag" Path="0.1" />
//             <Param Name="Vander" Type="Jagen" Path="1" />
//             <Param Name="Framme" Type="monk" Path="2" />
//             <Param Name="Clanne" Type="mage" Path="3" />
//             <Param Name="Morb" Type="Goat" Path="69" />
//         </AssetTable>
//         "#
//     ];

//     let mut merger = Merger::new(base);
//     for patch in patches {
//         merger.patch(patch);
//     }
//     let lhs = Merger::new(r#"
//     <AssetTable>
//         <Param Name="Alear" Type="ProtagAlt" Path="1" />
//         <Param Name="Griss" Type="Maso" Path="0.2" />
//         <Param Name="Veyle" Type="Protag" Path="0.3" />
//         <Param Name="Vander" Type="Jagen" Path="2" />
//         <Param Name="Framme" Type="monk" Path="4" />
//         <Param Name="Clanne" Type="mage" Path="69" />
//         <Param Name="Morb" Type="Goat" Path="70" />
//     </AssetTable>
//     "#).finalize_string();
//     let rhs = merger.finalize_string();
//     let a = lhs == rhs;
//     if !a {
//         std::fs::write("lhs.xml", lhs).unwrap();
//         std::fs::write("rhs.xml", rhs).unwrap();
//     }
//     assert!(a);
// }
