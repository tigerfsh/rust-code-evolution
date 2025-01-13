use rust_code_evolution::find_item_v2;

#[test]
fn find_item() {
    let a = [1, 2, 3, 4, 5];
    let res = find_item_v2(&a, 1);
    assert_eq!(res, Some(1));

    let res = find_item_v2(&a[..], 100);
    assert_eq!(res, None);
}
