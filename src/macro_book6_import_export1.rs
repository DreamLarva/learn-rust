// #[macro_use] 导出 mod 中所有的宏
#[macro_use]
mod macros {
    macro_rules! X {
        () => {
            Y!();
        };
    }
    macro_rules! Y {
        () => {};
    }
}

X!();
