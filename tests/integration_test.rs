use sigmaos_craft_font::{FontFace, FontRegistry, FontStyle, FontWeight};

#[test]
fn font_registry_integration_aaa() {
    let mut fr = FontRegistry::new();
    fr.register(FontFace {
        family: "Inter".into(),
        weight: FontWeight::Regular,
        style: FontStyle::Normal,
        path: "/Inter-Regular.ttf".into(),
    })
    .unwrap();
    fr.register(FontFace {
        family: "Inter".into(),
        weight: FontWeight::Bold,
        style: FontStyle::Normal,
        path: "/Inter-Bold.ttf".into(),
    })
    .unwrap();
    assert_eq!(fr.family_faces("Inter").len(), 2);
    assert!(fr
        .find_best_match("Inter", FontWeight::Medium, FontStyle::Normal)
        .is_some());
}
