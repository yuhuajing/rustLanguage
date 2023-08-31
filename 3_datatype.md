# RUST DataType

## scalar (simple) 
- Integers
<details>
<summary> Signed</summary>

有符号变量可以表示 负数， examples <code>i8 means -128~127</code>
Read more on  [Signed](https://careerbooster.teachable.com/courses/1869000/lectures/43560676).
![](diagrams/intergers.png)
```text
fn main() {
    let hell:i8 = 16;
    println!("{}",hell);
    let neghell:i8 = -6;
    println!("{}",neghell);
}
```
</details>

<details>
<summary>Unsigned</summary>

无符号变量仅表示正整数, examples <code>u8 means 0~255</code>
</details>

- Floating-point numbers
浮点数用于表示小数
<details>
<summary> Type</summary>
浮点数由f32 和 f64 两种长度。

定义浮点数变量有四种：
![](diagrams/float.png)
```text
fn main() {
    let hell:f32 = 123f32;
    println!("{}",hell);
    let hell1:f64 = 456f64;
    println!("{}",hell1);
    let hell2:f32 = 123.;
    println!("{}",hell2);
    let hell3:f32 = 123.789;
    println!("{}",hell3);
    let hell4:f32 = 123.789E2;
    println!("{}",hell4);
}
```
</details>

- Booleans
(true/false)bool变量占用1-bit的内存空间
```text
fn main() {
    let hell:bool = true;
    println!("{}",hell);
}
```

- Characters
<details>
<summary> char</summary>

char变量存储单一字符变量
```text
fn main() {
    let hell1:char = 'H';
    println!("{}",hell1);
}
```
</details>

<details>
<summary>str</summary>

字符串类型str，需要用 & 指定不定长的存储空间
```text
fn main() {
    let hell:&str = "Hell World!";
    println!("{}",hell);
}
```
</details>

## compound (combined)