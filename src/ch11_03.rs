#![allow(unused_variables)]
// 测试的组织结构
// 单元测试（unit tests）与 集成测试（integration tests）


// 测试模块的 #[cfg(test)] 注解告诉 Rust 只在执行 cargo test 时才编译和运行测试代码，而在运行 cargo build 时不这么做

// 测试私有函数
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

// internal_adder为私有函数 并没有pub
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2)); // 依然可以而测试
    }
}

// 二进制 crate 的集成测试
// 如果项目是二进制 crate 并且只包含 src/main.rs 而没有 src/lib.rs，这样就不可能在 tests 目录创建集成测
// 试并使用 extern crate 导入 src/main.rs 中定义的函数。只有库 crate 才会向其他 crate 暴露了可供调用和使
// 用的函数；二进制 crate 只意在单独运行。
//
// 为什么 Rust 二进制项目的结构明确采用 src/main.rs 调用 src/lib.rs 中的逻辑的方式？因为通过这种结构，集
// 成测试 就可以 通过 extern crate 测试库 crate 中的主要功能了，而如果这些重要的功能没有问题的话，
// src/main.rs 中的少量代码也就会正常工作且不需要测试。