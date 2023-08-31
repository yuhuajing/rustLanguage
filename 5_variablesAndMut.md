# Variables and Mutability

## Multability
直接声明的变量不可修改，通过 <code>mut</code>关键字可以声明为可修改的变量，通过直接赋值更改，而不是通过内存地址的修改。
```text
#![allow(unused)]
fn main() {
    let mut x:u8=3;
    x =5;
}
```
## Scoping
Rust 语言声明的字段都必须以`;`结尾，通过关键字 `{}` 可以声明独立的变量区间，内部的变量生命周期仅存在于变量区间内部，外部无法访问内部的变量值。
```text
#![allow(unused)]
fn main() {
    let mut x:u8=3;
    x =5;
    {
        let mut y:u8 = 9;
        y += x;
        println!("{}.", y);
    }
   // println!("{}.", y); // ERROR for reading a internal scope variables
   let mut i_like = "cats";
    { i_like = "dogs"; }
     println!("{}", i_like); // dogs
}
```
## Shadowing
Rust允许声明同名的变量，后声明的变量会覆盖掉前一个变量的值。
```text
#![allow(unused)]
fn main() {
    let x:u8=3;
    println!("{}.", x); //3
    let x:u8=8;
    println!("{}.", x);//8
}
```
## Patterns
当函数、结构体等返回多个数据时，通过自定义接受的变量名称可以选择性的接收数据
```text
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
```