extern crate proc_macro;

use crate::proc_macro::TokenStream;
// quote将syn解析的数据结构转换为Rust代码.
use quote::quote;
// syn crate 将字符串的Rust代码解析成为一个可以操作的数据结构
use syn;

// derive 自定义派宏
#[proc_macro_derive(HelloMacro_xxx)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 将 Rust 代码解析为语法树以便进行操作
    let ast = syn::parse(input).unwrap();
    // 返回类型必须是 TokenStream 所以必须 .unwrap() 或者 使用panic!

    // 构建 trait 实现
    impl_hello_macro(&ast)
}

// 当用户在一个类型上指定#[derive(HelloMacro)]时,hello_macro_derive函数就会被调用
// 原因在于我们已经使用 proc_macro_derive及其指定名称对 hello_macro_derive函数进行了注解:HelloMacro,
// 其匹配到trait名,这是大多数过程宏遵循的习惯

// 该函数首先将来自 TokenStream的input转换为为一个我们可以解释和操作的数据结构
// 这正是 syn派上用场的地方.syn中的parse_derive_input函数获取一个TokenStream并返回一个表示解析出Rust代码的DeriveInput结构体

// 带有宏属性的代码得到的 DeriveInput实例
// DeriveInput {
//     // --snip--
//
//     ident: Ident {
//         ident: "Pancakes",
//         span: #0 bytes(95..103)
//     },
//     data: Struct(
//         DataStruct {
//             struct_token: Struct,
//             fields: Unit,
//             semi_token: Some(
//                 Semi
//             )
//         }
//     )
// }

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    //              注解类型名字（标识符）的 Ident 结构体实例
    let name = &ast.ident; // 对应main.rs 这里是 "Pancakes"

    // quote! 宏让我们可以编写希望返回的Rust代码.quote! 宏执行的直接结果并不是编译器所期望的并需要转换为TokenStream/
    // 为此需要调用 into方法,它会 消费这个中间表示(intermediate representation,IR),并返回所需的TokenStream类型值
    let gen = quote! {
        impl HelloMacro for #name { // 注意这里 需要 HelloMacro 这个trait 所以用到这个宏 就要引入 HelloMacro
            fn hello_macro() {
                println!("Hello. Macro! My name is {}",stringify!(#name))
            }
        }
    };
    // quote! 宏提供的功能  模板中替换#name
    // https://docs.rs/quote/1.0.9/quote/

    // 我们期望我们的过程宏能够为通过#name 获取到的用户注解类型生成 HelloMacro trait的实现.
    // 该trait的实现有一个函数 hello_macro,其函数体包括了我们期望的功能:打印 打印 Hello, Macro! My name is 和注解的类型名。

    // 此处说使用的stringify!为Rust内置宏.其接受一个Rust表达式,如1 + 2,然后在编译时将表达式转换为一个字符串常量,如"1 + 2".
    // 这与 format! 或 print! 是不同的.它计算表达式并将结果转换为String.有一个种可能的情况是所输入的#name可能是一个需要打印的表达式
    // stringify!编译时也保留了一份将#name转换为字符串之后的内存分配
    gen.into()
}
