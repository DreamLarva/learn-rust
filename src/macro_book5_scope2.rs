mod a {
    // X!(); // 未被定义
}
mod b {
    // X!(); // 未被定义
    macro_rules! X {
        () => {};
    }
    X!(); // 已被定义
}
mod c {
    // X!(); // 未被定义
}
