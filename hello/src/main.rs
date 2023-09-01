#![allow(unused)]
use std::ops::Add;
fn main() {
    struct Sequence<A, B, C> {
        first: A,
        second: B,
        third: C,
    }
    impl<A, B, C> Sequence<A, B, C>
    where
        A: Copy,
        B: Copy,
    {
        pub fn get_first(&self) -> A {
            self.first
        }
        pub fn get_second(&self) -> B {
            self.second
        }
    }

    let sequence = Sequence {
        first: "mik",
        second: 3.987,
        third: 5,
    };
    println!("{}", sequence.get_first());
    println!("{}", sequence.get_second());

    enum MyEnum<A, B> {
        A(A),
        B(B),
    }
    let e = MyEnum::<u32, f32>::B(3.6982);
    //  println!("{}", e);
}
