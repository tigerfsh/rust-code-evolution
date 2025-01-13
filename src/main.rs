fn main() {
    // 循环结构：从命令式到函数式
    for number in (1..=3).rev() {
        println!("number: {}", number);
    }

    (1..=3)
        .rev()
        .for_each(|number| println!("number: {}", number));

    /*
    在 Rust 中，切片（slice）是一个动态大小的类型（DST，Dynamically Sized Type），它本身没有固定的内存大小。Rust 的函数参数需要是固定大小的类型，因此不能直接将切片作为参数传递，而必须使用切片的引用（如 &[T] 或 &mut [T]）。

    let a = [1, 2, 3, 4, 5];
    let b = a[..2];
    let c = a[..3];

    b和c都是[i32]，但是长度不同

    */
    let a = [1, 2, 3, 4, 5];
    let _b = &a[..2];
    let _c = &a[..3];
}
