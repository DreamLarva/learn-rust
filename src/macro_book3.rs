pub fn main() {
    // 调试
    // rustc 提供了调试宏.
    // 其中最有用的就是 trace_macros!.
    // 他会只是编译器,在每一个宏调用被展开将其转印出来

    macro_rules! each_tt {
            () => {};
            ($tt:tt $($rest:tt)*) => {
                println!("{}",stringify!($tt));
                each_tt!($($rest)*);
        };
    }

    // each_tt!(foo bar baz quux);
    // trace_macros!(true);
    each_tt!(spim wak plee whum);
    // 构建时打印
    // = note: expanding `each_tt! { spim wak plee whum }`
    // = note: to `each_tt! (wak plee whum) ;`
    // = note: expanding `each_tt! { wak plee whum }`
    // = note: to `each_tt! (plee whum) ;`
    // = note: expanding `each_tt! { plee whum }`
    // = note: to `each_tt! (whum) ;`
    // = note: expanding `each_tt! { whum }`
    // = note: to `each_tt! () ;`
    // = note: expanding `each_tt! {  }`
    // = note: to ``
    // trace_macros!(false);
    // each_tt!(trom qlip winp xod);

    // 宏log_syntax!。它将使得编译器输出所有经过编译器处理的标记。
    macro_rules! sing {
        () => {};
        ($tt:tt $($rest:tt)*) => {log_syntax!($tt); sing!($($rest)*);};
    }

    sing! {
        ^ < @ < . @ *
        '\x08' '{' '"' _ # ' '
        - @ '$' && / _ %
        ! ( '\t' @ | = >
        ; '\x08' '\'' + '$' ? '\x7f'
        , # '"' ~ | ) '\x07'
    }
    // 什么都没有打印 ????
}
