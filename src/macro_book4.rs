// 有时问题出在宏展开后的结果里。对于这种情况，可用编译命令--pretty来勘察。给出下列代码：
macro_rules! S {
    ($e:expr) => {
        String::from($e)
    };
}

fn main() {
    let world = S!("World");
    println!("Hello, {}!", world);
}
// 编译指令 rustc -Z unstable-options --pretty expanded hello.rs
// 应该打印

// #![feature(no_std, prelude_import)]
// #![no_std]
// #[prelude_import]
// use std::prelude::v1::*;
// #[macro_use]
// extern crate std as std;
// // Shorthand for initialising a `String`.
// fn main() {
//     let world = String::from("World");
//     ::std::io::_print(::std::fmt::Arguments::new_v1(
//         {
//             static __STATIC_FMTSTR: &'static [&'static str]
//                 = &["Hello, ", "!\n"];
//             __STATIC_FMTSTR
//         },
//         &match (&world,) {
//              (__arg0,) => [
//                 ::std::fmt::ArgumentV1::new(__arg0, ::std::fmt::Display::fmt)
//             ],
//         }
//     ));
// }

// 但是报错 error: Unrecognized option: 'pretty'
