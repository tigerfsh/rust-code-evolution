use rust_code_evolution::*;

#[test]
fn fibonacci() {
    let res1 = fibonacci_v1(10);
    let res2 = fibonacci_v2(10);
    let res3 = fibonacci_v3(10);
    let res4 = fibonacci_v4(10);
    assert_eq!((res2, res3, res4), (res1, res1, res1));
}
