pub mod macros {
    #[macro_export]
    macro_rules! X {
        () => {
            println!("X in macro_book6_import_export2.rs");
            Y!();
        };
    }
    #[macro_export]
    macro_rules! Y {
        () => {
            println!("Y in macro_book6_import_export2.rs");
        };
    }
}
// X!和Y!并非在此处定义的，但它们**的确**被
// 导出了，即便macros并非pub。
