extern crate proc_macro;

use proc_macro::TokenStream;
// quote将syn解析的数据结构转换为Rust代码.
use quote::quote;
// syn crate 将字符串的Rust代码解析成为一个可以操作的数据结构
use syn;

// 类属性宏
// 类属性宏和自定义派生宏相似,不同于derive属性生成代码,它们允许你创建新的属性.
// 他们也更为灵活;derive只能用于结构体和枚举;属性还可以用于其他的项,比如函数.
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 这里有两个 TokenStream 类型的参数；
    // 第一个用于属性内容本身，也就是 GET, "/" 部分。
    // 第二个是属性所标记的项：在本例中，是 fn index() {} 和剩下的函数体。
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    // 这里的 println! 是构建的时候才会输出
    // attr: "GET, "/""
    // item: "pub fn macro_attribute() { println! ("fn macro_attribute") }"
    item
}
