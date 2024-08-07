use self::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

// 引用循环与内存泄漏
// Rust的内存安全性 使其难以意外地制造永远也不会被清理的内存(被称为内存泄露),
// 但并不是不可能.与在编译时拒绝数据竞争不同,Rust并完全地避免内存泄露,这意味着内存泄露在Rust被认为
// 是内存安全的.这一点可以通过Rc<T> 和 RefCell<T> 看出创建引用循环的可能性是存在的.
// 这会造成内存泄露,因为每一项的引用计数永远也到不了0,其值也永远不会被丢弃

#[derive(Debug)]
enum List {
    // 不能够修改 i32 的值，
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    // 我们希望能够修改 Cons 成员所指向的 List。这里还增加了一个 tail 方法来方便我们在有 Cons 成员的时候访问其第二项。
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item  = {:?}", a.tail());

    // b 指向 a
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        // a 指向 b 成为 循环引用
        // 每一项的引用计数永远也到不了0,其值也永远不会被丢弃
        // 这段内存就泄露了
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing b = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment  the next line to see that we have a cycle;
    // it twill overflow the stack
    // println!("a next item = {:?}", a.tail());

    // a initial rc count = 1
    // a next item  = Some(RefCell { value: Nil })
    // a rc count after creation = 2
    // b initial rc count = 1
    // b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
    // b rc count after changing b = 2
    // a rc count after changing a = 2
}


pub struct Worker {
    id: usize,
    log: Rc<RefCell<Vec<String>>>,
}

impl Worker {
    pub fn new(id: usize, log: Rc<RefCell<Vec<String>>>) -> Self {
        Worker { id, log }
    }

    pub fn run(&self) {
        let mut log = self.log.borrow_mut();
        log.push(format!("Worker {} did some work", self.id))
    }
}


pub struct Engine {
    log: Rc<RefCell<Vec<String>>>,
    workers: Vec<Worker>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            log: Rc::new(RefCell::new(Vec::new())),
            workers: vec![],
        }
    }

    pub fn add_worker(&mut self, id: usize) {
        // 把Engine的 log 共享给 每个 worker

        //                                 这里clone 的是 Rc
        // self.workers.push(Worker::new(id, self.log.clone()));

        // 我推荐这么写
        self.workers.push(Worker::new(id, Rc::clone(&self.log)))
    }

    pub fn run(&self) {
        // 下面的都可以
        self.workers.iter().for_each(Worker::run);

        // self.workers.iter().for_each(|v| v.run());
    }

    pub fn print_log(&self) {
        let mut log = self.log.borrow();

        for entry in &*log {
            println!("{}", entry)
        }
    }
}

#[test]
fn case1() {
    let mut engine = Engine::new();
    for id in 0..3 {
        engine.add_worker(id)
    }

    engine.run();
    println!("Log content:");
    engine.print_log();

    engine.run();
    println!("\nFinal log content:");
    engine.print_log();
}