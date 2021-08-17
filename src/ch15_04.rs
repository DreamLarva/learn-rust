use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

// 避免引用循环: 将Rc<T> 变为 Weak<T>

// 调用Rc::clone 会增加 Rc<T> 实例的strong_count. 和 只在其strong_count为 0 时才会被清理的Rc<T> 实例
// 你也可以通过调用Rc::downgrade 并传递Rc<T>实例的引用来穿件其值的弱引用.
// 调用Rc::downgrade 会将 weak_count 加1.
// Rc<T> 类型使用weak_count 来记录其存在多少个Weak<T> 引用,weak_count 无需计数为0就能使Rc<T>实例被清理

// 强引用代表如何共享Rc<T> 实例所有权,但弱引用并不输入所有权关系.他们不会造成引用循环,
// 因为任何弱引用的循环会在其相关的强引用计数为0时被打断

// 因为Weak<T> 引用的值可能已经被丢弃了,为了使用Weak<T>所指向的值,我们必须确保其值任然有效.
// 为此可以调用Weak<T> 实例的 upgrade 方法,这会返回Option<Rc<T>>.
// 如果 Rc<T>值还未被丢弃,则结果是Some;如果Rc<T>已经被丢弃,则结果是None.
// 因为 upgrade返回一个Option<T> ,我们确信Rust 会处理Some 和 None 的情况,所以它不会返回非法指针.

// 创建树形数据结构: 带有子节点的Node

#[derive(Debug)]
struct Node {
    value: i32,
    // 父节点应该拥有其子节点：如果父节点被丢弃了，其子节点也应该被丢弃。
    // 然而子节点不应该拥有其父节点：如果丢弃子节点，其父节点应该依然存在。这正是弱引用的例子！
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    // leaf strong = 1, weak = 0

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        // branch strong = 1, weak = 0

        // 一旦在 branch 中有了 Node 实例，就可以修改 leaf 使其拥有指向父节点的 Weak<Node> 引用。
        // 这里使用了 leaf 中 parent 字段里的 RefCell<Weak<Node>> 的 borrow_mut 方法，
        // 接着使用了 Rc::downgrade 函数来从 branch 中的 Rc<Node> 值创建了一个指向 branch 的 Weak<Node> 引用。
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        // branch strong = 1, weak = 1

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
        // leaf strong = 2, weak = 0

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        // leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) }, children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } })
    }
    // branch 被清理 strong weak 都变成0

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // None

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    // leaf strong = 1, weak = 0
}
