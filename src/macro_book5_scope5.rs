// 可通过 #[macro_use]属性将宏导出模组：

mod a {
    // X!(); // 未被定义
}
#[macro_use]
mod b {
    macro_rules! X {
        () => {};
    }
    X!(); // 已被定义
}
mod c {
    X!(); // 已被定义
}
