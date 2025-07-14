use crate::n0067_add_binary::add_binary;

#[test]
fn test_add_binary() {
    assert_eq!(add_binary("11", "1"), "100");
    assert_eq!(add_binary("1010", "1011"), "10101");
}
