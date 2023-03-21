#[test]
fn test_crate_dir() {
    let crate_dir = location_macros::crate_dir!();
    assert!(crate_dir.ends_with("/location-macros"));
}
