//We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)]. 
//sCargo treats the tests directory specially and compiles files in this directory only when we run cargo test

use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}