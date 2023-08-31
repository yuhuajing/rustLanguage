fn main() {
    struct Mytuple(bool, u32, i8, f32);
    let tuple = Mytuple (true, 45, -4, 45.098);
    println!("{}", tuple.0); // true
    println!("{}", tuple.1); // 45
    println!("{}", tuple.2); // -4
    println!("{}", tuple.3); // 45.098
                                     // tuple.4 would result in a compilation error!

    //println!("Hello, world!");
}
