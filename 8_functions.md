# Functions

## functions
`fn fn_name(input1: InputType1, input2: InputType2) -> OutputType {
    // body
    //fn fn_name(input1: InputType1, input2: InputType2) -> OutputType {} //internal functions
}
`
```text
#![allow(unused)]
fn main() {
    println!("{}", add(1, 2));
}

fn add(a: u8, b: u8) -> u8 {
    a + b // return a+b;
}
```

## Associated function
`impl`关键字为特定的数据结构绑定函数

<details>
<summary>Example</summary>

```text
#![allow(unused)]
fn main() {
    struct Person {
        name: &'static str,
        age: u8,
    }
    impl Person {
        fn get_name(&self) -> &'static str {
            self.name
        }
        fn set_name(&mut self, _name: &'static str) {
            self.name = _name;
        }
    }

    let mut person = Person {
        name: "Mike",
        age: 18,
    };
    println!("{}", person.get_name());
    person.set_name("Inn");
    println!("{}", person.get_name());
}
```
</details>

## closures
变量作为函数使用，
```text
let a = [1, 2, 3];
let n: i32 = a.iter().map(|x| x * 2).sum();
println!("Sum of {:?} after doubling: {}", a, n); //Sum of [1, 2, 3] after doubling: 12

fn perform_operation(should_add: bool, amount_to_add: i32, value: i32) -> i32 {
    let operation: Box<dyn Fn(i32) -> i32> = if should_add {
        Box::new(|value: i32| value + amount_to_add)
    } else {
        Box::new(|value: i32| value + 1)
    };

    operation(value)
}
```
