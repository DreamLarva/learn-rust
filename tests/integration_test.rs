use adder;
// 与单元测试不同，我们需要在文件顶部添加 use adder。
// 这是因为每一个 tests 目录中的测试文件都是完全独立的 crate，所以需要在每一个文件中导入库。

mod common;
// 接着在测试函数中就可以调用 common::setup() 了。

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_one(3));
}

// 我们仍然可以通过指定测试函数的名称作为 cargo test 的参数来运行特定集成测试。也可以使用 cargo test 的 --test 后跟文件的名称来运行某个特定集成测试文件中的所有测试

// cargo test --test integration_test 单独执行某个集成测试
