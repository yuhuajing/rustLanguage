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

## Reference
数据在传递到别的函数作用域的话时候会将数据的所有权转移到别的函数，因此当前作用域内已经不能调用原数据。因此，希望在传递数据后然能使用该数据的话，需要传递数据的引用地址或者在现作用域内保留一份数据副本。

- 传递只读指针 `&<expression>`
```text
fn length_of_string(value: String) -> (String, usize) {
    let len = value.len();
    (value, len)
}
fn main() {
    let s1 = String::from("Hey there!");
  //  let len = String::len(&s1); //Delivery reference does not tranfer the ownership of the data.
    let (s1, len) = length_of_string(s1); //get the data from the other scope because the delivery data alters the data ownership. 
    println!("The length of {s1:?} is {len}."); 
}
```

- 传递读写指针 `&mut <expression>`
```text
fn length_of_string(value: String) -> (String, usize) {
    let len = value.len();
    (value, len)
}
fn length_of_string_mut(value: &mut String) -> usize {
    value.push_str("clay!!");
    value.len()
}
fn main() {
    let mut s1 = String::from("Hey there!");
  //  let len = String::len(&s1); //Delivery reference does not tranfer the ownership of the data.
   // let (s1, len) = length_of_string(s1); //get the data from the other scope because the delivery data alters the data ownership. 
   let len = length_of_string_mut(&mut s1);
   println!("The length of {s1:?} is {len}."); 
}
```

## Reference Rules
指针传递的规则需要注意前后引用的冲突--一旦出现`&mut`引用，对于该参数的引用只能出现一次。
```text
fn append_world(value: &mut String) {
    value.push_str(", World!");
}

fn main() {
    let mut s1 = String::from("Hello");
    let mut w1 = &mut s1;
    let w2 = &mut w1;  //ok
   //  let w2 = &mut s1;  // error : s1 cannot be borrowed mutably more than once at a time.
    append_world(w1);
    println!("The value is now {s1:?}.");
}
```