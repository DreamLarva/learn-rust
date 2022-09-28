use std::cell::RefCell;
use std::rc::Rc;

// 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者

// RefCell<T> 的一个常见用法是与 Rc<T>结合. 会议一下 Rc<T> 允许对相同多个数据有个多个所有者,
// 只不过能提供数据的不可变访问.如果有一个储存了 RefCell<T> 的Rc<T>的化,就可以有多个所有者并且可以修改的值了
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use self::List::{Cons, Nil};

pub fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    // 有两个 引用
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    // a after = Cons(RefCell { value: 15 }, Nil)
    // b after = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
    // c after = Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))

    let d = RefCell::new(Rc::new(1)); // 这种就是希望替换 Rc 一般是树的节点
    let d = Rc::new(RefCell::new(1)); // 这种就是希望替换 RefCell 中的值, 一般是节点上的value
}
