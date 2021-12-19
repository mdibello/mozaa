use crate::*;

#[test]
fn test_initialize() {
    initialize();
    assert_eq!(tiles_remaining(), 62);
}

#[test]
fn test_empty_space() {
    initialize();
    std::assert!(!contains_tile(&coordinate(31, 31)));
}