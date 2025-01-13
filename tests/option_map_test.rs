use rust_code_evolution::add_one_to_option_v2;

#[test]
fn add_one_to_option() {
    assert_eq!(add_one_to_option_v2(Some(10)), Some(11));
    assert_eq!(add_one_to_option_v2(None), None);

    let a: Option<i32> = Some(10);
    assert_eq!(a.map(|x| x + 1), Some(11));
    let a: Option<i32> = None;
    assert_eq!(a.map(|x| x + 1), None);
}
