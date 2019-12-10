use import;

#[test]
fn test_hash() {
    let bytes = include_bytes!("fixtures/test-file.txt");
    assert_eq!(
        "f457cac73aa9eaa1b7cbd31d5a3dbb7442bc27b5",
        import::hash::hash(bytes)
    );
}
