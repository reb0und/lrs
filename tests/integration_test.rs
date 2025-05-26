use lrs::add_two;

#[test]
fn it_adds_two() {
    let result = add_two(5);
    assert_eq!(result, 7);
}
