# Multi Thread and Synchronization
`std::thread` 实现多线程
```rust
fn main() {
    let handle = std::thread::spawn(|| {
        for i in 0..6 {
            println!("spawned: {i}");
            std::thread::yield_now();
        }
    });
    for i in 0..6 {
        println!("main: {i}");
        std::thread::yield_now();
    }
    handle.join().unwrap();
}
```
`std::sync`实现控制数据同步的通道读写结构
```rust

#![allow(unused)]
fn main() {
// Create a simple streaming channel.
let (tx1, rx) = std::sync::mpsc::channel();

// Copy the producer.
let tx2 = tx1.clone();

std::thread::spawn(move || {
    tx1.send(1).unwrap();
});

std::thread::spawn(move || {
    tx2.send(2).unwrap();
});

// Wait until you receive two messages on the main thread.
println!("{}", rx.recv().unwrap());
println!("{}", rx.recv().unwrap());
}
```

在 Rust 中，Arc 是 "原子引用计数"（Atomic Reference Counting）的缩写。Arc 是一个智能指针，用于在多线程环境中共享数据，确保安全地进行引用计数管理。

Arc 的主要作用包括：

1. 多线程共享数据: Arc 允许多个线程同时访问和共享数据，而无需显式地使用互斥锁（Mutex）等同步机制。这对于多线程编程非常有用，因为它可以减少竞态条件和死锁的风险。
2. 引用计数管理: Arc 跟踪数据的引用计数，当没有任何线程引用数据时，数据会被自动释放。这可以避免内存泄漏，因为只有当最后一个引用离开作用域时，数据才会被销毁。
3. 线程安全: Arc 是线程安全的，可以安全地在多线程环境中共享。它使用原子操作来更新引用计数，确保在并发情况下不会出现竞态条件。
4. 克隆数据: 与 Rc（引用计数）不同，Arc 允许多个线程之间克隆指向同一数据的引用，而不会引入可变性问题。这意味着多个线程可以拥有相同数据的不可变引用，而不会造成竞态条件。
5. 跨线程传递数据: Arc 允许将数据跨线程传递，因为它实现了 Send 和 Sync trait。这使得在不同线程之间安全地传递和共享数据成为可能。

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);
    
    for _ in 0..3 {
        let data_clone = Arc::clone(&data);
        thread::spawn(move || {
            // 在不同线程中共享 data_clone
            println!("{:?}", data_clone);
        });
    }

    // 等待所有线程完成
    thread::sleep(std::time::Duration::from_secs(2));
}
```
