//! Simple unit test example

use rust_hp_minitoring_cookie_cutter::add;

#[test]
fn test_add_function() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(0, 0), 0);
    assert_eq!(add(100, 200), 300);
}
