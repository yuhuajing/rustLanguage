#![allow(unused)]
fn main() {
    struct Person {
        name: &'static str,
        age: u8,
        isman: bool,
    }

    let p = Person {
        name: "Mike",
        age: 18,
        isman: true,
    };
    let Person { name, age, .. } = p;
    println!("{}.", name);
}
