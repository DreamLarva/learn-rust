/// Cargo 有两个主要的配置：
/// 运行 cargo build 时采用的 dev 配置和运行 cargo build --release 的 release 配置。
/// dev 配置被定义为开发时的好的默认配置，release 配置则有着良好的发布构建的默认配置。
///
/// 当项目的 Cargo.toml 文件中没有任何 [profile.*] 部分的时候，
/// Cargo 会对每一个配置都采用默认设置。通过增加任何希望定制的配置对应的 [profile.*] 部分，
/// 我们可以选择覆盖任意默认设置的子集。例如，如下是 dev 和 release 配置的 opt-level 设置的默认值：
/// ```
/// [profile.dev]
/// opt-level = 0
///
/// [profile.release]
/// opt-level = 3
/// ```
///
/// opt-level 设置控制 Rust 会对代码进行何种程度的优化。这个配置的值从 0 到 3。
/// 越高的优化级别需要更多的时间编译，所以如果你在进行开发并经常编译，可能会希望在牺牲一些代码性能的情况下编译得快一些。
/// 这就是为什么 dev 的 opt-level 默认为 0。
/// 当你准备发布时，花费更多时间在编译上则更好。
/// 只需要在发布模式编译一次，而编译出来的程序则会运行很多次，所以发布模式用更长的编译时间换取运行更快的代码。
/// 这正是为什么 release 配置的 opt-level 默认为 3。
fn ch14_01_release_profiles() {}

// 可以运行 cargo doc 来生成这个文档注释的 HTML 文档。
// 这个命令运行由 Rust 分发的工具 rustdoc 并将生成的 HTML 文档放入 target/doc 目录。
// 运行 cargo doc --open 会构建当前 crate 文档（同时还有所有 crate 依赖的文档）的 HTML 并在浏览器中打开
fn ch14_02_publishing_to_crates_io() {}
