use adder::add;
mod common;

#[test]
fn it_adds() {
    common::setup();
    let result = add(9, 5);
    assert_eq!(result, 14);
}