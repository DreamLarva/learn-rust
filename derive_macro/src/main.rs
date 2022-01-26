// 这句 trait 引用 必须有, 名字可以和 下面的宏同名
use hello_macro::HelloMacro; // 宏 中有用到 这个 trait,且名字必须一样

use hello_macro_derive::HelloMacro_xxx;

#[derive(HelloMacro_xxx)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro(); // ::hello_macro() 没有自动提示
}
