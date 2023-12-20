
use rust_book::add_two

#[test]
fn test_integration(){
    assert_eq!(4, add_two(2));
}