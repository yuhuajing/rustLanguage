#![allow(unused)]
fn main() {
    let val: i32 = 10;
    let r1 = &val;
    // This creates a copy of the value 10.
    let val2 = *r1;
    println!("Using width {}.", val2);
}
