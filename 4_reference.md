# Reference

## 指针
指针变量指向内存地址， 通过指针变量可以对内存地址进行操作。

<details>
<summary>只读指针</summary>

通过传递只读指针的方式传递数据，避免数据拷贝带来的额外存储。

对变量<code>T</code>的只读指针引用为<code>&T</code>

```text
#![allow(unused)]
fn main() {
    struct Rectangle {
        width: f64,
        height: f64,
    };

    let s = Rectangle {
        width: 12.3,
        height: 34.7,
    };

    let read_s = &s;
    println!("Using width {}.", read_s.width);

    let val = 10;
    let r1 = &val;
    let r2 = &val;
    println!("{r1} should be the same as {r2}.");
}
```
</details>

<details>
<summary>读写指针</summary>

对变量<code>T</code>的读写指针引用为<code>&mut T</code>
```text
#![allow(unused)]
fn main() {
    struct Rectangle {
        width: f64,
        height: f64,
    };

    let mut s = Rectangle {
        width: 12.3,
        height: 34.7,
    };

    let read_s = &mut s;
    read_s.width=3.19;
    println!("Using width {}.", s.width); //3.19
}
```
</details>

<details>
<summary>*</summary>

<code>*</code> 取出地址的数据
```text
#![allow(unused)]
fn main() {
    let val: i32 = 10;
    let r1: &i32 = &val;
    // This creates a copy of the value 10.
    let val2: i32 = *r1;
    println!("Using width {}.", val2); //3.19
}
```
</details>

## Copy
Data Type 拥有内置的数据权限，只有拥有copy权限的数据类型才能够复制数据到新值

无需指针定义的基础变量都拥有内置的Copy权限 (integers、char、bool、定长array数组、tuple元组)

```text
#![allow(unused)]
fn main() {
    let val: i32 = 10;
    let r1 = &val;
    // This creates a copy of the value 10.
    let val2 = *r1;
    println!("Using width {}.", val2);
}
```