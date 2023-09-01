# data ownership
在程序的共享资源中,通常通过垃圾回收、指针计数的方式管理内存数据，Rust内置数据所有权的概念，在数据编译时指定数据的内存分配情况。将所有数据和所有者绑定，无人绑定的数据直接回收释放。

## String
通过`String::with_capacity(4)`定义String类型数据，
```text
fn main() {
    // Print the stack-size of a String.
    println!("The size of a `String` is {}", std::mem::size_of::<String>());
    // Create a String with a capacity of 4.
    let mut hello = String::with_capacity(4);
    // Print how the String is represented on the stack.
    print_string_stack_data(&hello);
    // Append the text "Hello!" to the (currently empty) String.
    hello.push_str("Hello!");
    // The capacity and length should have changed, and maybe the pointer.
    print_string_stack_data(&hello);
    hello.push_str("clay!!");
    println!("{}", hello);
}
// Learning about unsafe Rust is out of scope so ignore this function.
fn print_string_stack_data(value: &String) {
    let ptr = value as *const _ as *const usize;
    println!("pointer  {0:16} 0x{0:016X}", unsafe { *ptr });
    println!("capacity {0:16} 0x{0:016X}", unsafe { *ptr.offset(1) });
    println!("length   {0:16} 0x{0:016X}", unsafe { *ptr.offset(2) });
}
```

## Drop
Rust中所有的数据都有归属，如果一个数据没有定义归属的话、被重写的话，就需要销毁。此时，只有拥有`Drop`权限的数据结构的数据才能进行销毁。

函数在离开作用域时释放资源或执行必要的清理操作，也就是资源的drop是自下向上进行。

## Copy
只有定长的数据结构可以拷贝。拷贝的数据作为值传递，改动后不会修改原值。
```text
fn main() {
    let mut a = String::from("a");
    let mut b = &mut a;
    *b = "b".to_string();
    println!("a = {a}"); // a=b
    let aa = String::from("a");
    let mut bb = aa.clone(); // Explicitly create a duplicate.
    println!("aa = {aa}"); // aa = a
    println!("bb = {bb}"); // bb=a
    bb = "c".to_string(); 
    println!("aa = {aa}"); //aa=a
    println!("bb = {bb}");//bb=c
}
```
