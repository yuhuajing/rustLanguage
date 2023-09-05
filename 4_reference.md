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

## Smart Reference

### Box<T>
在内存中分配可读写的存储空间。适用于
1. 数据字段的大小未知
2. 转移大数据的ownership

`Box<T>`拥有 `Deref + Drop`权限， Deref提供指针引用， Drop在数据超出当前操作区时进行销毁。

```text
fn create_data(small: bool) -> Box<[u8]> { // 编译时无法确定数组大小，因此直接使用定长数组会报错
    if small {
        Box::new([1, 2, 3])
    } else {
        Box::new([1, 2, 3, 4, 5])
    }
}
fn main() {
    let data = create_data(false);
    println!("{data:?}"); // [1, 2, 3, 4, 5]
}
```

### Rc<T>
共享数据的ownership，在不能clone的数据上执行指针的引用传递
```text
use std::rc::Rc;

struct Light {
    on: bool,
}

impl Light {
    fn new() -> Self {
        Light { on: false }
    }

    fn turn_on(&mut self) {
        if !self.on {
            self.on = true;
            println!("Turned on the light.");
        }
    }

    fn turn_off(&mut self) {
        if self.on {
            self.on = false;
            println!("Turned off the light.");
        }
    }
}

impl Drop for Light {
    fn drop(&mut self) {
        self.turn_off();
    }
}

struct Person {
    light: Rc<Light>,
}

impl Person {
    fn read_book(&self) {
        if self.light.on {
            println!("What a fantastic book!");
        } else {
            println!("It is hard to read without light...");
        }
    }
}

fn main() {
    let light = {
        let mut light = Light::new();
        light.turn_on();
        // Place the light in an `Rc<T>`
        Rc::new(light)
    };

    let mick = Person {
        // Note that `Light` does not implement `Clone`. We are cloning the
        // smart pointer here, not the value contained within. It is considered
        // good practice to call the clone implementation `Rc::clone` explicitly
        // because it encodes the intent to clone the smart pointer, not the
        // value itself.
        light: Rc::clone(&light),
    };
    let anna = Person { light: Rc::clone(&light), };
    let inn = Person { light }; //light ownership转移出当前scope，调用变量的drop函数
    mick.read_book();
    anna.read_book();
    inn.read_book();
}
```

### Ref<T> RefMut<T> RefCell<T>
指针的引用传递

```text
use std::{cell::RefCell, rc::Rc};

struct Light {
    on: bool,
}

impl Light {
    fn new() -> Self {
        Light { on: false }
    }

    fn turn_on(&mut self) {
        if !self.on {
            self.on = true;
            println!("Turned on the light.");
        }
    }

    fn turn_off(&mut self) {
        if self.on {
            self.on = false;
            println!("Turned off the light.");
        }
    }
}

impl Drop for Light {
    fn drop(&mut self) {
        self.turn_off();
    }
}

struct Person {
    // You have wrapped `Light` in a `RefCell` to provide internal mutability.
    light: Rc<RefCell<Light>>,
}

impl Person {
    fn read_book(&self) {
        // You have to call `RefCell::borrow` here to obtain an immutable reference `&Light`.
        if self.light.borrow().on {
            println!("What a fantastic book!");
        } else {
            println!("It is hard to read without light...");
        }
    }
}

fn main() {
    let light = {
        let mut light = Light::new();
        light.turn_on();
        // Place the light in an `Rc<T>`
        Rc::new(RefCell::new(light))
    };

    let mick = Person {
        // Note that `Light` does not implement `Clone`. You are cloning the
        // smart pointer here, not the value contained within. It is considered
        // good practice to call the clone implementation `Rc::clone` explicitly
        // because it encodes the intent to clone the smart pointer, not the
        // value itself.
        light: Rc::clone(&light),
    };
    let anna = Person {
        light: Rc::clone(&light),
    };

    // The light is on so mick can read.
    mick.read_book();

    // In order to turn off the light, you need a mutable reference `&mut Light`.
    // If any other references were handed out at this point, the program would panic.
    light.borrow_mut().turn_off();

    // The light is off so anna will have trouble reading.
    anna.read_book();
}
```