# Control flow

## If
`if statement{}else if statement{}else{}`
```text
#![allow(unused)]
fn main() {
    let score = 99;
    if score < 60 {
        println!("C");
    } else if score >= 60 && score < 95 {
        println!("B");
    } else {
        println!("A");
    }
}
```
## looping
### loop
无限循环直到 `break` 终止循环或者 `return`跳出函数
<details>
<summary>Example</summary>

```text

#![allow(unused)]
fn main() {
let mut i = 10;
loop {
    if i == 0 {
        break;
    }
    println!("{i}...");
    i -= 1;
}
println!("Launch!");
}
```
</details>

### while
无限循环直到不满足`while true{}`的条件
<details>
<summary>Example</summary>

```text

#![allow(unused)]
fn main() {
let mut i = 10;
while i != 0 {
    println!("{i}...");
    i -= 1;
}
println!("Launch!");
}
```
</details>

### for
循环`for conditions{}`
<details>
<summary>Example</summary>
其中 `a..=b(a<=i<=b)`和`a..b(a<=i`<`b)` 表示遍历a~b，`=`表示终止条件是`<` 或者`<=`

```text

#![allow(unused)]
fn main() {
for i in (1..=10).rev() {
    if i % 2 == 0 {
        continue; //跳出本轮循环
    }
    println!("{i}...");
}
println!("Launch!");
}
```
</details>