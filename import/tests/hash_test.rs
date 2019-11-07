use spc_pil_import;

#[test]
fn test_hash() {
    let bytes = include_bytes!("fixtures/MHRA MIP HLD v0.1.pdf");
    assert_eq!(
        "03e1b22894bd2d3bd087402367147fce9e50ffea",
        spc_pil_import::hash(bytes)
    );
}
