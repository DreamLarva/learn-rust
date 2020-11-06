// 集成测试
// 在 Rust 中，集成测试对于你需要测试的库来说完全是外部的。
// 同其他使用库的代码一样使用库文件，也就是说它们只能调用一部分库中的公有 API 。
// 集成测试的目的是测试库的多个部分能否一起正常工作。
// 一些单独能正确运行的代码单元集成在一起也可能会出现问题，
// 所以集成测试的覆盖率也是很重要的。

// 这是一个库啊
use adder;
// 与单元测试不同，我们需要在文件顶部添加 use adder。
// 这是因为每一个 tests 目录中的测试文件都是完全独立的 crate，
// 所以需要在每一个文件中导入库。

mod common;
// 接着在测试函数中就可以调用 common::setup() 了。

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_one(3));
}

// 我们仍然可以通过指定测试函数的名称作为 cargo test 的参数来运行特定集成测试。
// 也可以使用 cargo test 的 --test 后跟文件的名称来运行某个特定集成测试文件中的所有测试
// cargo test --test integration_test 单独执行某个集成测试

// 二进制 crate 的集成测试
// 如果项目是二进制 crate 并且只包含 src/main.rs 而没有 src/lib.rs，
// 这样就不可能在 tests 目录创建集成测试并使用 extern crate 导入 src/main.rs 中定义的函数。
// 只有库 crate 才会向其他 crate 暴露了可供调用和使用的函数；二进制 crate 只意在单独运行。

// 为什么 Rust 二进制项目的结构明确采用 src/main.rs 调用 src/lib.rs 中的逻辑的方式？
// 因为通过这种结构，集成测试 就可以 通过 extern crate 测试库 crate 中的主要功能了，而如果这些重要的功能没有问题的话，
// src/main.rs 中的少量代码也就会正常工作且不需要测试。

// package 的 name 指向 src/lib.rs
use the_rust_programming_language;
fn test2() {
    the_rust_programming_language::eat_at_restaurant()
}
