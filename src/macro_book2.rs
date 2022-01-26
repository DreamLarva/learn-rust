pub fn main() {
    // 不是标识符的标识符
    // 由于两个标记,当你撞见时,很有可能最终认为它们是标识符,但实际上它们不是.然而正式这些标记.某些情况下又的确是标识符
    // 第一个是 self
    macro_rules! what_is {
        (self) => {
            "the keyword `self`"
        };
        ($i:ident) => {
            concat!("the identifier `", stringify!($i), "`")
        };
    }

    macro_rules! call_with_ident {
        ($c:ident($i:ident)) => {
            $c!($i)
        };
    }

    println!("{}", what_is!(self)); // the keyword `self`
    println!("{}", call_with_ident!(what_is(self))); // the keyword `self`

    {
        // macro_rules! make_self_mutable {
        //     ($i:ident) => {
        //         let mut $i = self;
        //     };
        // }
        // struct Dummy(i32);
        // impl Dummy {
        //     fn double(self) -> Dummy {
        //         make_self_mutable!(mut_self);
        //         mut_self.0 *= 2;
        //         mut_self
        //     }
        // }

        // println!("{:?}", Dummy(4).double().0); // error
        // this function has a `self` parameter, but a macro invocation can only access identifiers it receives from parameters
        // 宏在匹配的时候，会欣然把self当作标识符接受，进而允许你把self带到那些实际上没办法使用的情况中去
    }
    {
        // macro_rules! double_method {
        //     ($body:expr) => {
        //         fn double(mut self) -> Dummy {
        //             $body
        //         }
        //     };
        // }
        // struct Dummy(i32);
        // impl Dummy {
        //     double_method! {{
        //         self.0 *= 2;
        //         self
        //     }}
        // }
        // println!("{:?}", Dummy(4).double().0); // error
        // this function has a `self` parameter, but a macro invocation can only access identifiers it receives from parameters
    }

    // 所以说，self是关键词，但当它想的时候，它同时也能是一个标识符。
    // 所以要保证卫生性
    macro_rules! double_method {
    ($self_:ident, $body:expr) => {
            fn double(mut $self_) -> Dummy {
                $body
            }
        };
    }
    struct Dummy(i32);
    impl Dummy {
        double_method! {self, {
                self.0 *= 2;
                self
            }
        }
    }
    println!("{:?}", Dummy(4).double().0); // ok
}
