// 为了不让 common 出现在测试输出中，我们将创建 tests/common/mod.rs ，而不是创建 tests/common.rs 。
// 这是一种 Rust 的命名规范，这样命名告诉 Rust 不要将 common 看作一个集成测试文件。

// 这样 setup 就相当于是tests中的一个公用的方法了 本身并不参加测试
pub fn setup() {
    // 编写特定库测试所需的代码
}
