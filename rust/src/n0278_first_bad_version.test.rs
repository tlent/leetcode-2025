use crate::n0278_first_bad_version::first_bad_version;

#[test]
fn test_first_bad_version_example_one() {
    assert_eq!(first_bad_version(5, |v| v >= 4), 4);
}

#[test]
fn test_first_bad_version_example_two() {
    assert_eq!(first_bad_version(1, |v| v >= 1), 1);
}
