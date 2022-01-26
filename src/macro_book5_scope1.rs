// 作用域

macro_rules! X {
    () => {};
}
mod a {
    X!(); // 已被定义
}
mod b {
    X!(); // 已被定义
}
mod c {
    X!(); // 已被定义
}
