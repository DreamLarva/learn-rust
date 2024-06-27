use std::borrow::Cow;
use std::collections::*;
use std::cell::Cell;

/// 类型 Cow 是一个提供复制-写入功能的智能指针：它可以封装并提供对借用数据的不可变访问，并在需要修改或拥有所有权时延迟复制数据。
/// 此类型设计用于通过 Borrow 特质来处理一般的借用数据。
/// Cow 实现 了 Deref 特质，这意味着您可以直接调用它所封装的数据的非变异方法。如果需要进行变异，to_mut 方法将获取对已拥有数据的可变引用，并在必要时进行克隆。
/// 如果您需要引用计数指针，请注意 Rc::make_mut 和 Arc::make_mut 也能提供复制-写入功能。

pub fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[test]
fn case1() {
    let slice = [0, 1, 2];
    // 使用的切片 并没有所有权
    let mut input = Cow::from(&slice[..]);
    assert!(matches!(input, Cow::Borrowed(_)), "Sorry, your assumption is incorrect!");
    // 没有clone
    assert!(matches!(abs_all(&mut input), Cow::Borrowed(_)), "Sorry, your assumption is incorrect!");
}

#[test]
fn case2() {
    let slice = [-1, 0, 1];
    // 使用的切片 并没有所有权
    let mut input = Cow::from(&slice[..]);
    assert!(matches!(input, Cow::Borrowed(_)), "Sorry, your assumption is incorrect!");
    assert!(matches!(abs_all(&mut input), Cow::Owned(_)), "Sorry, your assumption is incorrect!");
    assert!(matches!(abs_all(&mut input), Cow::Owned(_)));
}

#[test]
fn case3() {
    let vec = vec![0, 1, 2];
    // 获得所有权 之后都是 clone
    let mut input = Cow::from(vec);
    assert!(matches!(input, Cow::Owned(_)), "Sorry, your assumption is incorrect!");
    assert!(matches!(abs_all(&mut input), Cow::Owned(_)), "Sorry, your assumption is incorrect!");
}

#[test]
fn case4() {
    let vec = vec![-2, -1, 0, 1];
    // 获得所有权 之后都是 clone
    let mut input = Cow::from(vec);
    assert!(matches!(input, Cow::Owned(_)), "Sorry, your assumption is incorrect!");
    assert!(matches!(abs_all(&mut input), Cow::Owned(_)), "Sorry, your assumption is incorrect!");
}

/// std::cell::Cell 可变的内存位置。
/// 不可变的结构上 可以改变的部分

struct SomeStruct {
    regular_field: u8,
    special_field: Cell<u8>,
}

#[test]
fn case5() {
    let my_struct = SomeStruct {
        regular_field: 0,
        special_field: Cell::new(1),
    };

    let new_value = 100;

    // ERROR: `my_struct` is immutable
    // my_struct.regular_field = new_value;

    // WORKS: although `my_struct` is immutable, `special_field` is a `Cell`,
    // which can always be mutated
    my_struct.special_field.set(new_value);
    assert_eq!(my_struct.special_field.get(), new_value);
}


pub struct Switcher<'a> {
    lamp: &'a Cell<Lamp>,
}

impl<'a> Switcher<'a> {
    pub fn new(lamp: &'a Cell<Lamp>) -> Self {
        Switcher { lamp }
    }
    pub fn switch(&mut self) {
        let mut lamp = self.lamp.take();
        if lamp.is_on() {
            lamp.switch_off()
        } else {
            lamp.switch_on()
        }

        self.lamp.set(lamp);
    }
}


#[derive(Debug, Default, Clone, Copy)]
pub struct Lamp {
    on: bool,
}

impl Lamp {
    pub fn is_on(&self) -> bool {
        self.on
    }
    pub fn switch_on(&mut self) {
        self.on = true
    }
    pub fn switch_off(&mut self) {
        self.on = false
    }
}


#[test]
fn case6() {
    let lamp = Cell::new(Lamp::default());

    let mut sw1 = Switcher::new(&lamp);
    let mut sw2 = Switcher::new(&lamp);
    // let mut sw2 = Switcher::new(&sw1.lamp);

    println!("{:?}", lamp.get());
    sw1.switch();
    println!("{:?}", lamp.get());
    sw2.switch();
    println!("{:?}", lamp.get());
}

