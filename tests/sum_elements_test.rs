use rust_code_evolution::*;

#[test]
fn sum_elements() {
    let a = [1, 2, 3, 4, 5];

    let res = sum_elements_v1(a.to_vec());
    assert_eq!(res, 15);

    let res = sum_elements_v2(a.to_vec());
    assert_eq!(res, 15);

    let res = sum_elements_v3(&a[..]);
    assert_eq!(res, 15);
}
