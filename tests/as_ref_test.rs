use rust_code_evolution::*;

#[test]
fn calculate_length() {
    let a = String::from("hello,world");
    let b = "hello,world";

    calculate_length_v3(&a);
    calculate_length_v3(a);
    calculate_length_v3(b);
}
