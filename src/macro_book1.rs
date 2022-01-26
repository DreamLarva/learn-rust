macro_rules! four {
    () => {
        1 + 3
    };
}
macro_rules! gibberish {
    (4 fn ['spang "whammo"] @_@) => {};
}

macro_rules! multiply_add {
    ($a:expr, $b:expr, $c:expr) => {
        $a * ($b + $c)
    };
}

// 模式中可以包含重复。这使得匹配标记序列成为可能。重复的一般形式为$ ( ... ) sep rep.
macro_rules! vec_strs {
    (
        // 重复开始：
        $(
            // 每次重复必须有一个表达式...
            $element:expr
        )
        // ...重复之间由“,”分隔...
        ,
        // ...总共重复0或多次.
        *
    ) => {
        // 为了能包含多条语句，
        // 我们将扩展部分包裹在花括号中...
        {
            let mut v = Vec::new();
            // 重复开始：
            $(
                // 每次重复将包含如下元素，其中
                // “$element”将被替换成其相应的展开...
                v.push(format!("{}", $element));
            )*
            v
        }
    };
}

// 因为输入被解析为AST节点，替换所得的结果将无法析构。也就是说，你没办法检查其内容，或是再按原先相符的匹配匹配它。
macro_rules! capture_expr_then_stringify {
    ($e:expr) => {
        stringify!($e)
    };
}

macro_rules! capture_then_match_tokens {
    ($e:expr) => {
        match_tokens!($e)
    };
}
macro_rules! match_tokens {
    ($a:tt + $b:tt) => {
        "($a:tt + $b:tt)"
    };
    (($i:ident)) => {
        "($i:ident)"
    };
    ($($other:tt)*) => {
        "$($other:tt)*"
    };
}

macro_rules! capture_then_what_is {
    (#[$m:meta]) => {what_is!(#[$m])};
}
macro_rules! what_is {
    (#[no_mangle]) => {"#[no_mangle]"};
    (#[inline]) => {"#[inline]"};
    ($($tts:tt)*) => {concat!("something else (", stringify!($($tts)*), ")")};
}

// 匹配
// 捕获由$符号紧跟一个标识符(identifier)紧跟一个冒号(:)紧跟捕获种类组成。捕获种类须是如下之一：
// item: 条目，比如函数、结构体、模组等。 可用标记:任何标记
// block: 区块(即由花括号包起的一些语句加上/或是一项表达式)。 可用标记:任何标记
// stmt: 语句 可用标记: => 、 ;
// pat: 模式 可用标记:=> 、 =、 if、 in
// expr: 表达式 可用标记:=> 、 ;
// ty: 类型 可用标记:,、 =>、 :、 =、 >、 ;、 as
// ident: 标识符 (例如 foo, Bambous, self, we_can_dance, LaCaravane, …) 可用标记:任何标记
// path: 路径 (例如 foo, ::std::mem::replace, transmute::<_, int>, …) 可用标记:,、 =>、 :、 =、 >、 ;、 as
// meta: 元条目，即被包含在 #[...]及#![...]属性内的东西。 可用标记:任何标记
// tt: 标记树 可用标记:任何标记
// 上面个中配合的集合 可能有包含关系

// 此外，macro_rules! 通常不允许一个重复紧跟在另一重复之后，即便它们的内容并不冲突。
// 一般而言，在书写宏规则时，应从最具体的开始写起，依次写至最不具体的。

// 卫生性
// Rust宏是部分尾声的.具体来说.对于大部数标识符,它是卫生的;但对泛型参数和生命周期来算,它不是.
// 之所以能做到卫生,在于每个标识符都被赋予一个看不见的句法上下文.
// 在比较两个标识符时,只有在标识符的明面名字和句法上下文都一致的情况下,两个标识符才能被视作等同.

macro_rules! using_a1 {
    ($e:expr) => {{
        let a = 42;
        $e
    }};
}

macro_rules! using_a2 {
    ($a:ident, $e:expr) => {{
        let $a = 42;
        $e
    }};
}

pub fn main() {
    println!("{}", multiply_add!(1, 2, 3));
    let s = vec_strs![1, "a", true, 3.14159f32];
    assert_eq!(&*s, &["1", "a", "true", "3.14159"]);
    println!("{:?}", stringify!(dummy(2 * (1 + (3)))));
    println!("{:?}", capture_expr_then_stringify!(dummy(2 * (1 + (3)))));

    println!(
        "{}\n{}\n{}\n",
        match_tokens!((caravan)), // (($i:ident))
        match_tokens!(3 + 6),     // ($a:tt + $b:tt)
        match_tokens!(5)          // ($($other:tt)*)
    );
    println!(
        "{}\n{}\n{}\n",
        capture_then_match_tokens!((caravan)), // ($($other:tt)*)
        capture_then_match_tokens!(3 + 6),     // ($($other:tt)*)
        capture_then_match_tokens!(5)          // ($($other:tt)*)
    );

    println!(
        "{}\n{}\n{}\n{}\n",
        what_is!(#[no_mangle]),              // #[no_mangle]
        what_is!(#[inline]),                 // #[inline]
        capture_then_what_is!(#[no_mangle]), // something else (#[no_mangle])
        capture_then_what_is!(#[inline]),    // something else (#[inline])
    );

    // 卫生性
    // let four = using_a1!(a / 10); // 宏中的a 和 调用宏的a 并不是一个a
    let four = using_a2!(a, a / 10); // ok
}
