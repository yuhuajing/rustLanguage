
async fn hello_world() {
    println!("hello, world!");
}

#[tokio::main]
async fn main() {
    let _future = hello_world().await; // Nothing is printed
}