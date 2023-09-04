
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
