#![allow(unused_variables)] // 不对 未使用的变量 warning

use std::collections::*;
use std::fs::{self, File};
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::ops::Add;

pub fn ch08_01_vectors() {
    // 新建 Vector
    {
        // vector 矢量 向量
        // 方法1 创建矢量 存放 i32 内容的矢量
        let v1: Vec<i32> = Vec::new();

        // 方法2 让Rust 自己推断 出存放的类型
        let v2 = vec![1, 2, 3];
    }

    // 更新 Vector
    {
        // 厉害啊 rust 直接靠之后 第一次 update 使用的类型 来声明Vec 的类型
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        // v.push("1"); // error
    }
    {
        // 正确推断成了 &str
        let mut v = Vec::new();
        v.push("string");
    }

    // 丢弃vector 时 也会丢弃其所有的元素
    {
        let v = vec![1, 2, 3];
    } // 此处已经清理所有的元素 和vector

    // 读取vector 元素
    {
        let v = vec![1, 2, 3, 4, 5];
        let third: &i32 = &v[2]; // 索引语法
        let third = &v[2]; // 自动推断
        println!("the third element is {}", third);

        // get 语法 返回的 是以索引作为参数来返回一个 Option<&T>
        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element"),
        }

        // 尝试读取 越界的元素
        // let does_not_exist = &v[100]; // 运行时 error
        let does_not_exist = v.get(100); // 不会报错 因为返回的是 Option类型
                                         // 当 get 方法被传递了一个数组外的索引时，它不会 panic 而是返回 None。
    }

    // 一旦程序获取了一个有效的引用，
    // 借用检查器将会执行所有权和借用规则（第四章讲到）来确保 vector 内容的这个引用和任何其他引用保持有效。
    // 只能有 一个 mut 或者 多个 &
    {
        let mut v = vec![1, 2, 3, 4, 5];

        // let first_1 = &v[0]; // error 在操作 mut 之前 操作第一个元素

        v.push(6); // borrow 改变内容
        v.push(6); // borrow 改变内容

        let first_3 = &v[0]; // 获取所有权 读取数据
        println!("The first element is: {}", first_3);
    }
    // 为什么第一个元素的引用会关心 vector 结尾的变化？
    // 不能这么做的原因是由于 vector 的工作方式：在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻
    // 存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。
    // 这时，第一个元素的引用就指向了被释放的内存。
    // 借用规则阻止程序陷入这种状况。

    // 遍历vector 中的元素
    {
        let v = vec![100, 32, 57];

        // 一般而言遍历都是借用而不是move
        for i in &v {
            println!("{i}");
        }
    }
    // 同时需要 index
    {
        let v = vec![100, 32, 57];

        for v in v.iter() {
            println!("{v}");
        }

        for (v, index) in v.iter().enumerate() {
            println!("{},{}", v, index);
        }
    }
    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            // 注意是 borrow mut
            // *i 类型 是 i32
            *i += 50; // 为了修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值
            println!("{}", i);
        }
        for i in v.iter_mut() {
            *i += 50; // 为了修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值
            println!("{}", i);
        }
    }

    // 使用枚举来存储多种类型
    // 枚举的成员都被定义为相同的枚举类型，所以当需要在 vector 中储存不同类型值时，我们可以定义并使用一个枚举！
    {
        #[derive(Debug)]
        enum SpreadSheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        // 同样 rust 推断出了 vector 中的类型是 SpreadSheetCell 枚举
        let mut row = vec![
            SpreadSheetCell::Int(3),
            SpreadSheetCell::Text(String::from("blue")),
            SpreadSheetCell::Float(10.12),
        ];

        let some_data = row.pop(); // pop 弹出一个元素

        // 牛啊 直接判断 Int Text 的分支是不会进入的
        match &some_data {
            Float => (),
            Text => (),
            Int => (),
        }
        println!("{:?}", some_data.unwrap());
    }

    // 文档中的示例
    {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);

        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], 1);

        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.len(), 1);

        vec[0] = 7;
        assert_eq!(vec[0], 7);

        // extend 是 std::iter::traits::Extend 有重载
        // 类似js 的 concat
        vec.extend([0, 1]); // 传入一个数组
        vec.extend(&[2]);
        vec.extend([3].iter().copied()); // 传入迭代器
        vec.extend([4, 5].iter());
        vec.extend(vec![6]); // 另一个 vec

        for x in &vec {
            println!("{}", x);
        }
        assert_eq!(vec, [7, 0, 1, 2, 3, 4, 5, 6]);
    }
    // 使用 vec! 宏生成
    {
        // 按照 长度生成 初始化
        let vec = vec![0; 5];
        assert_eq!(vec, [0, 0, 0, 0, 0]);

        // The following is equivalent, but potentially slower:
        let mut vec = Vec::with_capacity(5);
        vec.resize(5, 0);
        assert_eq!(vec, [0, 0, 0, 0, 0]);
        let vec = vec![0; 5];
        assert_eq!(vec, [0, 0, 0, 0, 0]);

        // The following is equivalent, but potentially slower:
        let mut vec = Vec::with_capacity(5);
        vec.resize(5, 0); // 按照 new_len 扩大或缩小 数组, 用 value 填充
        assert_eq!(vec, [0, 0, 0, 0, 0]);
    }
    // 使用 vac 作为一个 stack使用
    {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            // Prints 3, 2, 1
            println!("{}", top);
        }
    }
    // 切片
    {
        fn read_slice(slice: &[usize]) {
            // ...
        }

        // 我操 这页太强了吧 这也能推断出来
        let v = vec![0, 1];
        read_slice(&v);

        // ... and that's all!
        // you can also do it like this:
        let u: &[usize] = &v;
        // or like this:
        let u: &[_] = &v;
        let w = u.to_vec();
        assert_eq!(v, w)
    }
    // pub fn with_capacity(capacity: usize) -> Vec<T>
    {
        let mut vec = Vec::with_capacity(10);

        // The vector contains no items, even though it has capacity for more
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 10);

        // These are all done without reallocating...
        for i in 0..10 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 10);
        assert_eq!(vec.capacity(), 10);

        // ...but this may make the vector reallocate
        vec.push(11);
        assert_eq!(vec.len(), 11);
        println!("vec.capacity(): {}", vec.capacity()); // 20?
        assert!(vec.capacity() >= 11);
    }

    // pub fn shrink_to_fit(&mut self)
    // 尽量 缩小 capacity
    {
        let mut vec = Vec::with_capacity(10);
        vec.extend([1, 2, 3].iter().cloned());
        assert_eq!(vec.capacity(), 10);
        vec.shrink_to_fit();
        println!("shrink_to_fit:{}", vec.capacity());
        // 因为是尽量 所以依然可能 > 3
        assert!(vec.capacity() >= 3);
    }
    // pub fn into_boxed_slice(self) -> Box<[T], Global>
    // 将数组转换成 Box<[T]>
    // 任何多余的 capacity 都会被丢弃
    {
        let v = vec![1, 2, 3];
        let slice = v.into_boxed_slice();

        let mut vec = Vec::with_capacity(10);
        vec.extend([1, 2, 3].iter().cloned());

        assert_eq!(vec.capacity(), 10);
        let slice = vec.into_boxed_slice();
        assert_eq!(slice.into_vec().capacity(), 3);
    }
    // pub fn truncate(&mut self, len: usize)
    // 保留前 len 个元素
    {
        let mut vec = vec![1, 2, 3, 4, 5];
        vec.truncate(2);
        assert_eq!(vec, [1, 2]);

        let mut vec = vec![1, 2, 3, 4, 5];
        vec.truncate(2);
        assert_eq!(vec, [1, 2]);

        let mut vec = vec![1, 2, 3];
        vec.truncate(0);
        assert_eq!(vec, []);
    }

    // pub fn as_slice(&self) -> &[T]
    // 提取包含整个向量的切片
    // 同 &s[..]
    {
        use std::io::{self, Write};
        let buffer = vec![1, 2, 3, 5, 8];
        io::sink().write(buffer.as_slice()).unwrap();
    }
    // pub fn swap_remove(&mut self, index: usize) -> T
    // 删除 index 位置的元素 并返回 它
    // 删除的元素 会被 原来 vec 的最后一个元素替代
    // 所以这个方法是不会保证 vec 原来顺序的
    {
        let mut v = vec!["foo", "bar", "baz", "qux"];

        assert_eq!(v.swap_remove(1), "bar");
        assert_eq!(v, ["foo", "qux", "baz"]);

        assert_eq!(v.swap_remove(0), "foo");
        assert_eq!(v, ["baz", "qux"]);
    }

    // pub fn insert(&mut self, index: usize, element: T)
    // 在 index 位置 插入 element
    {
        let mut vec = vec![1, 2, 3];
        vec.insert(1, 4);
        assert_eq!(vec, [1, 4, 2, 3]);
        vec.insert(4, 5);
        assert_eq!(vec, [1, 4, 2, 3, 5]);
    }

    // pub fn remove(&mut self, index: usize) -> T
    // 在 index 位置的 元素
    {
        let mut v = vec![1, 2, 3];
        assert_eq!(v.remove(1), 2);
        assert_eq!(v, [1, 3]);
    }

    // pub fn retain<F>(&mut self, f: F)
    // where
    //     F: FnMut(&T) -> bool,
    // 保留符合 f 函数的元素 , 原地删除不符合的元素
    {
        let mut vec = vec![1, 2, 3, 4];
        vec.retain(|&x| x % 2 == 0);
        assert_eq!(vec, [2, 4]);
    }
    // 按照索引保留数据
    {
        let mut vec = vec![1, 2, 3, 4, 5];
        let keep = [false, true, true, false, true];
        let mut i = 0;
        vec.retain(|_| (keep[i], i += 1).0);
        assert_eq!(vec, [2, 3, 5]);
    }
    // ```
    // pub fn dedup_by_key<F, K>(&mut self, key: F)
    // where
    //     F: FnMut(&mut T) -> K,
    //     K: PartialEq<K>,
    // ```
    // 移除vec中 调用F方法后 出现连续的相同的值就只保留一个
    {
        let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

        vec.dedup_by_key(|i| *i / 10);
        // [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1]
        // 所以只保留 值1 和 值10 位置上的原来的值
        println!("{:?}", vec);

        assert_eq!(vec, [1, 10]);
    }

    // pub fn dedup_by<F>(&mut self, same_bucket: F)
    // where
    //     F: FnMut(&mut T, &mut T) -> bool,
    // 移除vec中 调用same_bucket方法后 相同返回值中的多余一个的其他元素
    // same_bucket 方法 传入 两个元素 a,b  ;
    // a,b 表示的值为 current , previous
    // 如果 same_bucket(a,b) 返回 true, 则 a 被删除
    {
        let mut vec = vec!["foo", "bar", "Bar", "baz", "bar"];

        vec.dedup_by(|a, b| {
            // println!("a:{},b:{}", a, b);
            a.eq_ignore_ascii_case(b)
        });
        // 打印的内容为
        // a:bar,b:foo
        // a:Bar,b:bar
        // a:baz,b:bar
        // a:bar,b:baz

        assert_eq!(vec, ["foo", "bar", "baz", "bar"]);
    }

    // pub fn push(&mut self, value: T)
    // 在最末尾增加一个元素
    // Panics if the new capacity exceeds isize::MAX bytes.
    {
        let mut vec = vec![1, 2];
        vec.push(3);
        assert_eq!(vec, [1, 2, 3]);
    }

    // pub fn pop(&mut self) -> Option<T>
    // 删除并弹出啊 vec 的最后一个元素, 如果vec没有元素 则返回none
    {
        let mut vec = vec![1, 2, 3];
        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec, [1, 2]);
    }
    // pub fn append(&mut self, other: &mut Vec<T>)
    // 将 other 中元素 全部移到 self 中, other 被置为 空
    {
        let mut vec = vec![1, 2, 3];
        let mut vec2 = vec![4, 5, 6];
        vec.append(&mut vec2);
        assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
        assert_eq!(vec2, []);
    }
    // pub fn drain<R>(&mut self, range: R) -> Drain<'_, T>
    // Drain 上业实现了 trait Iterator
    // where
    //     R: RangeBounds<usize>,
    // 创建一个 可以删除并yield指定range的vec 的 draining iterator
    // 当 iterator 被 dropped 的时候, 不管iterator有没有被完全消耗所有 vec 的 range 中的元素都被移除
    // 如果 Iterator 没有被 dropped 如(使用 mem::forget) , it is unspecified how many elements are removed.
    {
        let mut v = vec![1, 2, 3];
        let u: Vec<_> = v.drain(1..).collect();
        assert_eq!(v, &[1]);
        assert_eq!(u, &[2, 3]);

        // 完全消耗掉
        v.drain(..);
        assert_eq!(v, &[]);
    }

    // 清空 vec , 删除所有元素
    // 对已分配 capacity 的  vec 是没有作用的
    {
        let mut v = vec![1, 2, 3];
        v.clear();
        assert!(v.is_empty());
    }
    // pub fn len(&self) -> usize
    // 返回数组元素的个数
    {
        let a = vec![1, 2, 3];
        assert_eq!(a.len(), 3);
    }

    // pub fn is_empty(&self) -> bool
    // vec 是否为空
    {
        let mut v = Vec::new();
        assert!(v.is_empty());
        v.push(1);
        assert!(!v.is_empty());
    }

    // pub fn split_off(&mut self, at: usize) -> Vec<T>
    // 返回一个新的已经分配内存的 包含了 [at,len]的vec
    // 在调用完 本方法后,原来的vec 只剩下 [0,at) 且 capacity 不变
    {
        let mut vec = vec![1, 2, 3];
        let vec2 = vec.split_off(1);
        assert_eq!(vec, [1]);
        assert_eq!(vec2, [2, 3]);
    }

    // pub fn resize_with<F>(&mut self, new_len: usize, f: F)
    // where
    //     F: FnMut() -> T,
    // 原地调整 vec 的大小 到 new_len
    // 如果 new_len 大于 len, 则 分别用调用闭包函数 f 填充每个位置
    // 如果 new_len 小于 len, 则 直接剪切掉多余部分
    // 这个方法使用闭包来生成每个新的元素.如果你希望 Clone 一个值,使用Vec::resize
    // 如果希望 使用 Default trait 来生成值, 可以传 Default::default 作为第二个参数
    {
        let mut vec = vec![1, 2, 3];
        vec.resize_with(5, Default::default);
        assert_eq!(vec, [1, 2, 3, 0, 0]);

        let mut vec = vec![];
        let mut p = 1;
        vec.resize_with(4, || {
            p *= 2;
            p
        });
        assert_eq!(vec, [2, 4, 8, 16]);
    }

    // pub fn leak<'a>(self) -> &'a mut [T]
    // where
    //     T: 'a,
    // 消耗并 leak vec,返回一个 可变的内容的引用
    // Note that the type T must outlive the chosen lifetime 'a.
    // If the type has only static references, or none at all, then this may be chosen to be 'static.
    // This function is similar to the leak function on Box except that there is no way to recover the leaked memory.
    // This function is mainly useful for data that lives for the remainder of the program's life.
    // Dropping the returned reference will cause a memory leak.
    {
        let x = vec![1, 2, 3];
        let static_ref: &'static mut [usize] = x.leak();
        static_ref[0] += 1;
        assert_eq!(static_ref, &[2, 2, 3]);
    }
    // impl<T> Vec<T>
    // where
    //     T: Clone,
    //  pub fn resize(&mut self, new_len: usize, value: T)
    // 调整数组的大小 , 不够用 value 补
    // This method requires T to implement Clone, in order to be able to clone the passed value.
    // If you need more flexibility (or want to rely on Default instead of Clone), use Vec::resize_with.
    {
        let mut vec = vec!["hello"];
        vec.resize(3, "world");
        assert_eq!(vec, ["hello", "world", "world"]);

        let mut vec = vec![1, 2, 3, 4];
        vec.resize(2, 0);
        assert_eq!(vec, [1, 2]);
    }

    // impl<T> Vec<T>
    // where
    //     T: Clone,
    // pub fn extend_from_slice(&mut self, other: &[T])
    // clone 并 追加 other 中的内容到 vec
    // 方法等同于 extend 只是入参是 slice (此方法可能被废弃)
    {
        let mut vec = vec![1];
        vec.extend_from_slice(&[2, 3, 4]);
        assert_eq!(vec, [1, 2, 3, 4]);
    }

    // impl<T> Vec<T>
    // where
    //     T: PartialEq<T>,
    // pub fn dedup(&mut self)
    // 根据 PartialEq 移除连续的重复元素
    {
        let mut vec = vec![1, 2, 2, 3, 2];
        vec.dedup();
        assert_eq!(vec, [1, 2, 3, 2]);
    }

    // impl<T> Vec<T>
    // pub fn splice<R, I>(
    //     &mut self,
    //     range: R,
    //     replace_with: I
    // ) -> Splice<'_, <I as IntoIterator>::IntoIter>ⓘ
    // where
    //     I: IntoIterator<Item = T>,
    //     R: RangeBounds<usize>,
    // 创建一个 splicing 的 iterator 用 replace_with 替换 特定 range 的 vec, 并返回 从 vec 删除的内容
    // range 内的内容一定被移除, 即使 iterator 没有被消耗
    // It is unspecified how many elements are removed from the vector if the Splice value is leaked.
    // The input iterator replace_with is only consumed when the Splice value is dropped.
    // This is optimal if:
    //  The tail (elements in the vector after range) is empty,
    //  or replace_with yields fewer elements than range’s length
    //  or the lower bound of its size_hint() is exact.
    // Otherwise, a temporary vector is allocated and the tail is moved twice.
    {
        let mut v = vec![1, 2, 3];
        let new = [7, 8];
        //                                       克隆每个 new 中的元素的 iterator    将 iterator 再转 vec
        let u: Vec<_> = v.splice(..2, new.iter().cloned()).collect();
        assert_eq!(v, &[7, 8, 3]);
        assert_eq!(u, &[1, 2]);
    }

    // Methods from Deref<Target = [T]>
    // Deref trait 实现的方法

    // pub const fn len(&self) -> usize
    // 返回 vec 的长度
    {
        let a = [1, 2, 3];
        assert_eq!(a.len(), 3);
    }
    // pub const fn is_empty(&self) -> bool
    // 是否为 空
    {
        let a = [1, 2, 3];
        assert!(!a.is_empty());
    }

    // pub fn first(&self) -> Option<&T>
    // 返回第一个元素 的 slice, 没有 返回 None
    // 非常合理 返回的元素为 引用
    {
        let v = [10, 40, 30];
        assert_eq!(Some(&10), v.first());

        let w: &[i32] = &[];
        assert_eq!(None, w.first());
    }

    // pub fn first_mut(&mut self) -> Option<&mut T>
    // 返回一个 第一个元素的可变指针,或者 None
    {
        let x = &mut [0, 1, 2];
        if let Some(first) = x.first_mut() {
            *first = 5;
        }
        assert_eq!(x, &[5, 1, 2]);
    }

    // pub fn split_first(&self) -> Option<(&T, &[T])>
    // 返回 第一个元素 和 剩余元素的 元组, 没有返回 None
    {
        let x = &[0, 1, 2];
        if let Some((first, elements)) = x.split_first() {
            assert_eq!(first, &0);
            assert_eq!(elements, &[1, 2]);
        }
    }

    // pub fn split_first_mut(&mut self) -> Option<(&mut T, &mut [T])>
    // 返回 第一个元素 和 剩余元素的 元组, 没有返回 None
    {
        let x = &mut [0, 1, 2];
        if let Some((first, elements)) = x.split_first_mut() {
            *first = 3;
            elements[0] = 4;
            elements[1] = 5;
        }

        assert_eq!(x, &[3, 4, 5]);
    }
    // pub fn split_last(&self) -> Option<(&T, &[T])>
    // pub fn split_last_mut(&mut self) -> Option<(&mut T, &mut [T])>
    // pub fn last(&self) -> Option<&T>
    // pub fn last_mut(&mut self) -> Option<&mut T>

    // pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>
    // where
    //     I: SliceIndex<[T]>,
    // 入参是 一个位置,则 返回 这个元素的引用,否则返回 None
    // 入参是 一个 range,返回 这个 range 的切片,否则返回 None
    {
        let v = [10, 40, 30];
        assert_eq!(Some(&40), v.get(1));
        assert_eq!(Some(&[10, 40][..]), v.get(0..2));
        assert_eq!(None, v.get(3));
        assert_eq!(None, v.get(0..4));
    }
    // pub fn get_mut<I>(
    //     &mut self,
    //     index: I
    // ) -> Option<&mut <I as SliceIndex<[T]>>::Output>
    // where
    //     I: SliceIndex<[T]>,
    {
        let x = &mut [0, 1, 2];
        if let Some(elem) = x.get_mut(1) {
            *elem = 42;
        }
        assert_eq!(x, &[0, 42, 2]);
    }
    // pub fn swap(&mut self, a: usize, b: usize)
    // 交换 vec 中的2个元素
    {
        let mut v = ["a", "b", "c", "d"];
        v.swap(1, 3);
        assert_eq!(v, ["a", "d", "c", "b"]);
    }

    // pub fn reverse(&mut self)
    // 反转vec 原地修改
    {
        let mut v = [1, 2, 3];
        v.reverse();
        assert_eq!(v, [3, 2, 1]);
    }

    // pub fn iter(&self) -> Iter<'_, T>
    // pub fn iter_mut(&mut self) -> IterMut<'_, T>
    // 返回 iterator
    {
        let x = &[1, 2, 4];
        let mut iterator = x.iter();

        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.next(), None);
    }
    // pub fn iter_mut(&mut self) -> IterMut<'_, T>
    // 滑动窗口 迭代
    {
        let x = &mut [1, 2, 4];
        for elem in x.iter_mut() {
            *elem += 2
        }
        assert_eq!(x, &[3, 4, 6]);
        // println!("{:?}", [1, 2, 3, 4, 5, 6].windows(2).collect::<Vec<_>>());
    }

    // pub fn split_at(&self, mid: usize) -> (&[T], &[T])
    // 将 vec 分成 [0, mid) 和  [mid, len) 两份
    {
        let v = [1, 2, 3, 4, 5, 6];
        {
            let (left, right) = v.split_at(0);
            assert_eq!(left, []);
            assert_eq!(right, [1, 2, 3, 4, 5, 6]);
        }

        {
            let (left, right) = v.split_at(2);
            assert_eq!(left, [1, 2]);
            assert_eq!(right, [3, 4, 5, 6]);
        }

        {
            let (left, right) = v.split_at(6);
            assert_eq!(left, [1, 2, 3, 4, 5, 6]);
            assert_eq!(right, []);
        }
    }
    // pub fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T])
    {
        let mut v = [1, 0, 3, 0, 5, 6];
        // scoped to restrict the lifetime of the borrows
        {
            let (left, right) = v.split_at_mut(2);
            assert_eq!(left, [1, 0]);
            assert_eq!(right, [3, 0, 5, 6]);
            left[1] = 2;
            right[1] = 4;
        }
        assert_eq!(v, [1, 2, 3, 4, 5, 6]);
    }

    // pub fn contains(&self, x: &T) -> bool
    // where
    //     T: PartialEq<T>,
    // 是否包含目标元素
    {
        let v = [10, 40, 30];
        assert!(v.contains(&30));
        assert!(!v.contains(&50));

        // If you do not have an &T,
        // but just an &U such that T: Borrow<U> (e.g. String: Borrow<str>),
        // you can use iter().any:
        let v = [String::from("hello"), String::from("world")];
        assert!(v.iter().any(|e| e == "hello"));
        assert!(!v.iter().any(|e| e == "hi"));
    }

    // pub fn starts_with(&self, needle: &[T]) -> bool
    // where
    //     T: PartialEq<T>,
    {
        let v = [10, 40, 30];
        assert!(v.starts_with(&[10]));
        assert!(v.starts_with(&[10, 40]));
        assert!(!v.starts_with(&[50]));
        assert!(!v.starts_with(&[10, 50]));
    }

    // pub fn ends_with(&self, needle: &[T]) -> bool
    // where
    //     T: PartialEq<T>,
    {
        let v = [10, 40, 30];
        assert!(v.ends_with(&[30]));
        assert!(v.ends_with(&[40, 30]));
        assert!(!v.ends_with(&[50]));
        assert!(!v.ends_with(&[50, 30]));
    }

    // pub fn binary_search(&self, x: &T) -> Result<usize, usize>
    // where
    //     T: Ord,
    // 二分查找找到 x 值, 找到返回 Result::Ok(找到的位置), 如果有多个可能返回其中的任意一个
    // 没找到 则返回 Result::Err(可以插入目标值的位置)
    {
        let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        assert_eq!(s.binary_search(&13), Ok(9));
        assert_eq!(s.binary_search(&4), Err(7));
        assert_eq!(s.binary_search(&100), Err(13));
        let r = s.binary_search(&1);
        assert!(match r {
            Ok(1..=4) => true,
            _ => false,
        });

        // 插入一个值 并保持顺序
        let mut s = vec![0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        let num = 42;
        //                这里返回的值是 Err()      解 Err() 中的值
        let idx = s.binary_search(&num).unwrap_or_else(|x| x);
        s.insert(idx, num);
        assert_eq!(s, [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 42, 55]);
    }
    // 不稳定的排序 更快, 但是相同的元素 可能移动位置 不适用额外的辅助空间
    // sort_unstable
    // sort_unstable_by

    // 稳定的排序 速度相对相对 较慢 需要使用额外的辅助空间, 更适合已经近似排序的内容
    // pub fn sort(&mut self)
    // where
    //     T: Ord,
    {
        let mut v = [-5, 4, 1, -3, 2];

        v.sort();
        assert_eq!(v, [-5, -3, 1, 2, 4]);
    }

    // pub fn sort_by<F>(&mut self, compare: F)
    // where
    //     F: FnMut(&T, &T) -> Ordering,
    {
        // f64 没有实现 Ord 因为 NaN != NaN
        // 所以当我们知道 不包含 NaN , 就可以使用 partial_cmp 作为排序的依据
        let mut floats = [5f64, 4.0, 1.0, 3.0, 2.0];
        floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(floats, [1.0, 2.0, 3.0, 4.0, 5.0]);

        let mut v = [5, 4, 1, 3, 2];
        v.sort_by(|a, b| a.cmp(b));
        assert_eq!(v, [1, 2, 3, 4, 5]);

        // reverse sorting
        v.sort_by(|a, b| b.cmp(a));
        assert_eq!(v, [5, 4, 3, 2, 1]);
    }

    // pub fn sort_by_key<K, F>(&mut self, f: F)
    // where
    //     F: FnMut(&T) -> K,
    //     K: Ord,
    // Sorts the slice with a key extraction function.
    // 不懂什么意思 但是传入的 函数并不需要比较, 只需要返回一个可以进行比较的值 即可
    {
        let mut v = [-5i32, 4, 1, -3, 2];

        v.sort_by_key(|k| {
            // 每个元素 可以进入本函数 多次
            // println!("{}", k);
            k.abs()
        });
        assert_eq!(v, [1, 2, -3, 4, -5]);
    }
    // sort_by_cached_key
    // 缓存 传入函数的 方法 , 其他同 sort_by_key

    // pub fn to_vec(&self) -> Vec<T>ⓘ
    // where
    //     T: Clone,
    // 复制当前vec 到 一个vec
    {
        let s = [10, 40, 30];
        let x = s.to_vec();
        // x , s 完全不影响 , 毕竟是 clone 的
    }

    // pub fn repeat(&self, n: usize) -> Vec<T>ⓘ
    // where
    //     T: Copy,
    // 新建一个 拷贝了当前vec n 次 的vec
    {
        assert_eq!([1, 2].repeat(3), vec![1, 2, 1, 2, 1, 2]);
    }

    // pub fn concat<Item>(&self) -> <[T] as Concat<Item>>::Output
    // where
    //     Item: ?Sized,
    //     [T]: Concat<Item>,
    // Flattens a slice of T into a single value Self::Output 不知道 怎么翻译
    {
        assert_eq!(["hello", "world"].concat(), "helloworld");
        assert_eq!([[1, 2], [3, 4]].concat(), [1, 2, 3, 4]);
        println!("{:?}", [[[[1]]], [[[1]]]].concat().concat()); // 一次只展开一层
    }

    // pub fn join<Separator>(
    //     &self,
    //     sep: Separator
    // ) -> <[T] as Join<Separator>>::Outputⓘ
    // where
    //     [T]: Join<Separator>,
    // 类似 concat 但是会在每个元素之间加入 sep
    {
        assert_eq!(["hello", "world"].join(" "), "hello world");
        assert_eq!([[1, 2], [3, 4]].join(&0), [1, 2, 0, 3, 4]);
        assert_eq!([[1, 2], [3, 4]].join(&[0, 0][..]), [1, 2, 0, 0, 3, 4]);
    }
}
