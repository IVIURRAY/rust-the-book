use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}

#[test]
fn it_adds_two2() {
    assert_eq!(4, adder::add_two(4));
}
