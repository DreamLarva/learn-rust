use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
use std::iter::FromIterator;

// 类函数宏
// 类函数宏定义看起来像函数调用的宏。
// 类似于 macro_rules!，它们比函数更灵活；
// 例如，可以接受未知数量的参数。
// 然而 macro_rules! 宏只能使用之前 “使用 macro_rules! 的声明宏用于通用元编程” 介绍的类匹配的语法定义。
// 类函数宏获取 TokenStream 参数，其定义使用 Rust 代码操纵 TokenStream，就像另两种过程宏一样。
#[proc_macro]
pub fn hex(input: TokenStream) -> TokenStream {
    let ts = TokenStream::from_iter(input.into_iter());
    TokenStream::from(TokenTree::Group(Group::new(Delimiter::Bracket, ts)))
}
