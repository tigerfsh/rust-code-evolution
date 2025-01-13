use std::fs;
use std::io;

// 斐波那契数列
pub fn fibonacci_v1(term: u32) -> u32 {
    if term == 0 {
        0
    } else if term == 1 {
        1
    } else {
        fibonacci_v1(term - 1) + fibonacci_v1(term - 2)
    }
}

pub fn fibonacci_v2(term: u32) -> u32 {
    match term {
        0 | 1 => term,
        _ => fibonacci_v2(term - 1) + fibonacci_v2(term - 2),
    }
}

pub fn fibonacci_v3(term: u32) -> u32 {
    if term == 0 || term == 1 {
        term
    } else {
        let mut a: u32 = 0;
        let mut b: u32 = 1;
        for _ in 2..=term {
            // let tmp = b;
            // b = a + b;
            // a = tmp;
            (a, b) = (b, a + b);
        }
        b
    }
}

pub fn fibonacci_v4(term: u32) -> u32 {
    match term {
        0 | 1 => term, // 竖线表示模式匹配中的`或`操作符
        _ => {
            let res = (0..term).fold((0, 1), |(a, b), _| (b, a + b));
            res.0
        }
    }
}

// 错误处理：勇敢面对现实
fn read_file_v1(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).unwrap();
    contents
}

fn read_file_v2(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
}

// Option类型：空值处理的艺术
fn find_item_v1(items: &[i32], target: i32) -> i32 {
    for &item in items {
        if item == target {
            return item;
        }
    }
    -1
}

// 有或者没有，用options表示，some/none
pub fn find_item_v2(items: &[i32], target: i32) -> Option<i32> {
    items.iter().find(|&&item| item == target).copied()
}

// Map与Option：优雅的值变换
fn add_one_to_option_v1(option: Option<i32>) -> Option<i32> {
    match option {
        Some(i) => Some(i + 1),
        None => None,
    }
}

pub fn add_one_to_option_v2(option: Option<i32>) -> Option<i32> {
    option.map(|i| i + 1)
}

// 迭代器：从显式到隐式

pub fn sum_elements_v1(vec: Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in 0..vec.len() {
        sum += vec[i];
    }
    sum
}

pub fn sum_elements_v2(vec: Vec<i32>) -> i32 {
    let mut sum = 0;
    for item in vec {
        sum += item;
    }
    sum
}

pub fn sum_elements_v3(vec: &[i32]) -> i32 {
    vec.iter().sum()
}

pub fn factorial_v1(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial_v1(n - 1),
    }
}

pub fn factorial_v2(num: u64) -> u64 {
    (1..=num).fold(1, |acc, x| acc * x)
}

pub fn factorial_v3(num: u64) -> u64 {
    (1..=num).product()
}

// 处理Option类型：简化错误处理

fn set_default_if_none_v1(option: Option<i32>) -> i32 {
    match option {
        Some(value) => value,
        None => 0,
    }
}

fn set_default_if_none_v2(option: Option<i32>) -> i32 {
    option.unwrap_or(100)
}

// 封装与模块化：让代码更具可维护性

#[derive(Debug, PartialEq)]
pub struct User {
    pub username: String,
    pub age: u32,
}

impl User {
    pub fn new(username: String, age: u32) -> Self {
        User { username, age }
    }
}

// Match vs If-Let：选择最简洁的方式
// if let 语法简化了模式匹配，当我们只对一种模式感兴趣时，if let 是最好的选择。它比 match 更简洁，且不失表达力。
fn my_if_let() {
    let number = Some(10);

    if let Some(7) = number {
        println!("Lucky number 7!");
    } else {
        println!("Not a lucky number :(");
    }
}

trait Shape {
    fn area(&self) -> u32;
}
// Traits的有效使用：提高通用性与灵活性
// shape1和shape2可能是不同的类型
fn compare_areas_v1(shape1: &impl Shape, shape2: &impl Shape) {
    if shape1.area() == shape2.area() {
        println!("The areas are the same!");
    }
}

// 使用泛型参数T 来明确限制shape1 和shape2 必须是相同类型的Shape，能够提高代码的类型安全性，避免了过度泛化的情况。
fn compare_areas_v2<T: Shape>(shape1: &T, shape2: &T) {
    if shape1.area() == shape2.area() {
        println!("The areas are the same!");
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
}
