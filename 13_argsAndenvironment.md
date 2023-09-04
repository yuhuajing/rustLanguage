# Arguments and Environment

## 获取环境变量

```text
#![allow(unused)]
fn main() {
    for argument in std::env::args() {
        println!("{argument}");
    }
    for (key, value) in std::env::vars() {
        println!("{key}={value}");
    }
    let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
}
```