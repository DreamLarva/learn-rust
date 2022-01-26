mod a {
    // X!(); // 未被定义
}
macro_rules! X {
    () => {};
}
mod b {
    X!(); // 已被定义
}
mod c {
    X!(); // 已被定义
}
