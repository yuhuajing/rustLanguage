#![allow(unused)]
fn main() {
    let score = 99;
    if score < 60 {
        println!("C");
    } else if score >= 60 && score < 95 {
        println!("B");
    } else {
        println!("A");
    }
}
