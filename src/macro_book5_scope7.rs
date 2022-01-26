// 已经废弃
// rust 2018
// 现在 直接use 宏 引入进来 使用

// mod a {
//     // X!(); // 已被定义，但Y!并未被定义
// }
// macro_rules! Y {
//     () => {};
// }
// mod b {
//     X!(); // 均已被定义
// }
// #[macro_use]
// extern crate macs;
// mod c {
//     X!(); // 均已被定义
// }
