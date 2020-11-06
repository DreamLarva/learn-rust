#![allow(unused_variables)]
// 测试的组织结构
// 单元测试（unit tests）与 集成测试（integration tests）
// 单元测试倾向于更小而更集中，在隔离的环境中一次测试一个模块，或者是测试私有接口。
// 集成测试对于你的库来说则完全是外部的。它们与其他外部代码一样，通过相同的方式使用你的代码，只测试公有接口而且每个测试都有可能会测试多个模块。
// 为了保证你的库能够按照你的预期运行，从独立和整体的角度编写这两类测试都是非常重要的。

// 单元测试
// 单元测试的目的是在与其他部分隔离的环境中测试每一个单元的代码，以便于快速而准确的某个单元的代码功能是否符合预期。单元测试与他们要测试的代码共同存放在位于 src 目录下相同的文件中。规范是在每个文件中创建包含测试函数的 tests 模块，并使用 cfg(test) 标注模块。

// 测试模块的 #[cfg(test)]
// 注解告诉 Rust 只在执行 cargo test 时才编译和运行测试代码，而在运行 cargo build 时不这么做
// 如果测试的文件是在另你一个文件中 那么可以不需要 #[cfg(test)]
// 你可以用 其他指令启动测试

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

// 集成测试
// 见 根目录 test 文件夹

// 二进制 crate 的集成测试
// 如果项目是二进制 crate 并且只包含 src/main.rs 而没有 src/lib.rs，这样就不可能在 tests 目录创建集成测
// 试并使用 extern crate 导入 src/main.rs 中定义的函数。只有库 crate 才会向其他 crate 暴露了可供调用和使
// 用的函数；二进制 crate 只意在单独运行。
//
// 为什么 Rust 二进制项目的结构明确采用 src/main.rs 调用 src/lib.rs 中的逻辑的方式？因为通过这种结构，集
// 成测试 就可以 通过 extern crate 测试库 crate 中的主要功能了，而如果这些重要的功能没有问题的话，
// src/main.rs 中的少量代码也就会正常工作且不需要测试。
