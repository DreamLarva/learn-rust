mod a {
    // X!(); // 未被定义
}
macro_rules! X {
    () => {
        Y!();
    };
}
mod b {
    // X!(); // 已被定义, 但Y!未被定义
}
macro_rules! Y {
    () => {};
}
mod c {
    X!(); // 均已被定义
}
