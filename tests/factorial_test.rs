use rust_code_evolution::*;

#[test]
fn factorial() {
    for i in 0..=10 {
        assert_eq!(factorial_v1(i), factorial_v2(i));
        assert_eq!(factorial_v1(i), factorial_v3(i));
    }
}
