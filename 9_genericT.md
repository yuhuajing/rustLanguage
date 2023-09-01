# generic
定义和数据类型无关的通用数据结构 `<T>`
<details>
<summary>同类型参数结构体</summary>

```text
#![allow(unused)]
fn main() {
    struct Sequence<T> {
        first: T,
        second: T,
        third:T,
    }
    impl <T> Sequence<T> where T: Copy {
       pub fn get_first(&self) ->  T {
            self.first
        }
    }

    let sequence = Sequence { first: "milk", second: "jerry", third: "Dan" };
    println!("{}", sequence.get_first());
}
```

`impl` 内部绑定函数的返回类型由 `T`决定，因此 判断内部数据是否全部一致的问题时需要调用外部函数实现，因为 str 类型的变量无法单纯通过 `==`判断
```text
#![allow(unused)]
fn main() {
    struct Sequence<T> {
        first: T,
        second: T,
        third:T,
    }
    impl <T> Sequence<T> where T: Copy + std::cmp::PartialEq {
       pub fn get_first(&self) ->  T {
            self.first
        }

        pub fn is_equal(&self)->bool{
            self.first==self.second && self.second==self.third
        }
    }

    let sequence = Sequence { first: "milk", second: "jerry", third: "Dan" };
    println!("{}", sequence.is_equal());
}
```
</details>

<details>
<summary>不同类型参数结构体</summary>
通过 `<A,B,C>`等多种不确定的泛型定义含有不同参数类型的数据

```text
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
```
</details>