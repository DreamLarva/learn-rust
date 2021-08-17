use std::thread;
use std::time::Duration;

/// 在大部分现在操作系统中, 已执行的程序在一个 *进程* (process)中运行,操作系统则复测管理多个进程
/// 在程序内部,也可以拥有多个同时运行的独立部分.
/// 运行这些独立部分的功能称为 *线程*(threads).
///
/// 将程序中的计算拆分进多个线程可以改善性能,因为程序可以同时进行多个任务,不过这也会增加复杂性.
/// 因为线程是同时运行的,所以无法预先保证不同线程中的代码的执行顺序.这会导致诸如此类的问题:   
/// 1. 竞争状态(Race conditions) . 多个线程以不一致的顺序访问数据或资源
/// 2. 死锁(Deadlocks),两个线程互相等待对方停止使用所拥有的资源,这会阻止他们继续运行
/// 3 只会发生在特定情况且难以稳定重现和修复的bug
///
/// 由编程语言调用操作系统 API 创建线程的模型有时被称为 1:1，一个 OS 线程对应一个语言线程
/// Rust只提供了 1:1线程模型实现.如果要使用M:N线程模型要使用 对应的crate
pub fn ch16_01_threads() {
    {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the 子线程!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        // 等待 新建线程执行完毕
        handle.join().unwrap();

        for i in 1..5 {
            println!("hi number {} from the 主线程!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
    // 线程与 move 闭包
    {
        let v = vec![1, 2, 3];
        // 强制闭包获取 使用值的所有权
        let handle = thread::spawn(move || {
            println!("Here's a vector{:?}", v);
        });

        handle.join().unwrap();
    }

    ()
}

use std::ptr::read_volatile;
use std::rc::Rc;
use std::sync::{mpsc, Arc};

// 使用消息传递在线程间传送数据
pub fn ch16_02_message_passing() {
    // Rust中实现消息传递 并发的主要工具是 通道(channel),Rust标准库提供了其实现的编程概念.
    // 你可以将其想象为一个水流的通道,比如河流或小溪.
    // 如果你将诸如橡皮鸭或小船之类的东西放入其中,它们会顺流而下到达下游.
    //
    // 编程中的通道有两部分,一个是发送者(transmitter) 和 一个接受者(receiver).
    // 发送者位于上游位置,在这里可以将橡皮鸭放入河中,接受者位于下游,橡皮鸭最终会漂流至此.
    // 此代码的一部分调用发送者的方法以及希望发送的数据,另一部分则检查接受端收到的消息.
    // 当发送者或接收者任一被丢弃时认为通道被关闭了(closed)了.
    {
        let (tx, rx) = mpsc::channel();
        // 这里使用 mpsc::channel 函数创建一个新的通道;mpsc是 多个生产者单个消费者(multiple producer.single consumer) 的缩写.
        // 简而言之. 如是的标准库实现通道的方式以为着 一个通道可以有多个产生值的发送(sending)端,但只能有一个消费这些值的接收(receiving)端
        // mpsc::channel 函数返回一个元组: 第一个是发送端.第二个是接收端

        thread::spawn(move || {
            let val = String::from("hi1");
            // move 将 tx 移动到 线程的 闭包中
            // send 方法返回一个 Result<T,E> 类型,所以如果接收端已经被丢弃了,将没有发送值的目标,所以发送操作会返回错误
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!(" Got: {}", received);
    }
    // 发送多个值 并观察接收者的等待
    {
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi2"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1))
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }
    // 通过克隆放着来创建多个 生产者
    {
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi3"),
                String::from("from3"),
                String::from("the3"),
                String::from("thread3"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more4"),
                String::from("messages4"),
                String::from("for4"),
                String::from("you4"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        for received in rx {
            println!("Got: {}", received);
        }
    }
}

// 共享状态并发
pub fn ch16_03_shared_state() {
    // 互斥器 一次只允许一个线程 访问数据
    // 互斥器(mutex) 是mutual exclusion的缩写.也就是说,任意时刻,其只允许一个线程访问某些数据.
    // 为了方位互斥器的数据,线程首先需要通过互斥器的 锁(lock)来表明其希望访问的数据.
    // 锁是一个 作为互斥器 一部分的数据结构,它几率谁有数据的排他访问权.
    // 因此,我们描述互斥器为通过锁系统 保护(guarding)其数据

    // 互斥器 以难以使用著称,因为你不得不记住:
    // 1. 在使用数据之前 尝试获取 锁.
    // 2. 处理完 被互斥器所保护的数据后,必须解锁数据,这样其他线程才能获取权限

    // Mutex<T> 的 API
    use std::sync::Mutex;
    {
        let m = Mutex::new(5);
        {
            // 使用lock 的方式获取 锁
            // 这个调用会阻塞 当前线程,直到我们拥有锁为止.
            // 如果另一个线程拥有锁,并且那个线程panic了,则lock调用会失败.
            // 这种情况下,没人能够再获取锁,所以这了选择 unwrap 并在遇到这种情况下使线程panic
            // 一旦获取了锁,就可以将返回值(这里是num) 视为一个内部数据的可变引用了.
            // 类型系统确保了我们在使用m中的值 之前获取锁: Mutex<i32> ,所以必须获取锁才能使用这个i32值
            // Mutex是一个只能指针.lock调用返回了叫做 MutexGuard的智能指针.这个指针实现了Deref来指向其内部数据
            // 其也提供了一个Drop 实现当Mutex离开作用域的 时候自动释放锁.
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        println!("m = {:?}", m);
    }

    // 线程间共享 Mutex<T>
    {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result {}", *counter.lock().unwrap())
    }

    // RefCell<T> / Rc<T> 与 Mutex<T> / Arc<T> 的相似性
    // Mutex提供了内部的可变性, 就像Cell系列类型那样.
    // RefCell<T> 可以改变 Rc<T>中的内容那样, 同样的可以使用 Mutex<T> 来改变Arc<T>的内容
    // 另一个 值得注意的细节是 Rust 不能避免 Mutex<T>的全部逻辑错误.
    // Rc<T> 就有造成引用循环的风险,这两个Rc<T>值相互引用,造成内存泄露.
    // 同理Mutex<T> 页游可能造成死锁的风险.这发生于当一个造作需要锁住两个资源而两个线程各持有一个锁,这会造成他们永远相互等待着.
}

/// # 使用 Sync 和 Send trait 的可拓展并发
/// 有两个并发概念是内嵌于语言中的  std::marker 中的 Sync 和 Send trait
///
/// # 通过 send 允许在线程间转移所有权
/// Send 标记 trait 表明类型的所有权可以在线程间产地.季候所有的Rust的类型都是 Send 的,
/// 不过有一些例外,包括Rc<T>: 这是不可能Send 的,因为如果克隆了 Rc<T>的值并尝试将克隆的所有权转移到另一个线程,
/// 这两个线程不可能同时更新引用计数. 因此Rc<T>被实现用于单线程场景,这时不需要为拥有线程安全的引用计数而付出性能的代价
/// 因此Rust的类型系统和trait bound 确保永远不会意外的将不安全的 Rc<T> 在线程间发送.
/// 另一个 Send 的是 裸指针(raw pointer)
///
/// # Sync允许多线程访问
/// sync标记 trait表明衣蛾实现了 Sync的类型可以安全的在多个线程中拥有其值的引用.
/// 换一种方式说,对于任意类型T,如果&T(T的引用)是Send 的情况,基本类型是Sync的,这意味着其引用就可以安全送到另一个线程.
/// 类似于Send 的情况,基本类型是Sync 的,完全由Sync的类型组成也是Sync的.    
/// 智能指针Rc<T>也不是Sync的,处于岂不是Send相同的原因.RefCell<T>和Cell<T> 系列类型都不是Sync的.
/// RefCell<T>在运行时所进行的借用检查也不是线程安全的.
/// Mutex<T> 是Sync的, 是线程安全的,可以在多线程间被访问
///
/// # 手动实现Send 和 Sync 是不安全的
/// 通常不需要手动实现Send 和 Sync trait,因为由 Send 和 Sync的类型组成的类型,自动就是 Send 和Sync 的.   
/// 因为他们是标记 trait , 甚至都不需要实现任何方法.他们只是用来加强并发相关的不可变性.
///
pub fn ch16_04_extensible_concurrency_sync_and_send() {}
