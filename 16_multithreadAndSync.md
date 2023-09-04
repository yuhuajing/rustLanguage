# Multi Thread and Synchronization
`std::thread` 实现多线程
```text
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
```text

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
