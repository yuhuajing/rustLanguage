# RUST

## data ownership

Rust语言特殊的一点在于，它有所有权（ownership）的概念，在程序编译时就指定内存区的占用。当你拿到一个值的时候，这个值的所有权就归你所有，你可以随意使用，但当你不再使用它时，你必须交还给Rust，否则Rust就会在后台释放内存，这会导致你的程序崩溃。

Example:
```text
    fn determine_thing() -> &'static str {
        "THING"
    }
    fn determine_thing_2() -> String {
        String::from("THING")
    }
```
- &'static str 表示字符串常量，String表示字符串变量。
- 字符串常量存储在静态区，返回对当前对象的引用，声明周期与程序周期一致。
- 字符串变量存储在堆上，返回对当前对象的拷贝，声明周期为当前作用域。

### 所有权规则

- 变量被分配到栈上，当栈上的值不再被使用时，栈上的值就会被释放。
- 变量被分配到堆上，当堆上的值不再被使用时，堆上的值会被释放。