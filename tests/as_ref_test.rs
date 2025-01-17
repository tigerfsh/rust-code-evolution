use rust_code_evolution::*;

#[test]
fn calculate_length() {
    let a = String::from("hello,world");
    let b = "hello,world";

    /*
    https://doc.rust-lang.org/std/convert/trait.AsRef.html#reflexivity
    这段代码的意思是：如果 T 实现了 AsRef<str>，那么 &T 也会自动实现 AsRef<str>。
    实现的方式是通过解引用 self（即 *self），然后调用 T 的 as_ref() 方法。
    */

    calculate_length_v3(&a);
    calculate_length_v3(a);
    calculate_length_v3(b);
}
