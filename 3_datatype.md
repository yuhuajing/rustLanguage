# RUST DataType

## scalar (simple) 
### Integers
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

### Floating-point numbers
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

### Booleans
(true/false)bool变量占用1-bit的内存空间
```text
fn main() {
    let hell:bool = true;
    println!("{}",hell);
}
```

### Characters
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

字符串类型str，需要用 & 指定不定长的存储空间，因为字符串可以截取改变长度。
```text
fn main() {
    let mut hello = String::from("Hello ");
    hello.push('w'); // insert a signle charactor
    hello.push_str("orld!"); // insert a string charactor
    println!("{}", hello); 

    let mut qian:&'static str = "qian";
   // qian.push("king");
    println!("{}", qian); 
}
```
</details>

## compound (combined)
### Array
数组分为定长和不定长数组，数组内部参数的类型和数组类型一致。
<details>
<summary>定长数组</summary>

Array定长数组，在定义时指定类型和长度：<code>[T;N]</code> where T is he type and the N is the size of the array. 或者 直接初始化数组。总之数组长度固定。
```text
fn main() {
    let array: [u32; 3] = [1, 2, 3]; // let array = [1, 2, 3]; 
    println!("{}", array[0]); // 1
    println!("{}", array[1]); // 2
    println!("{}", array[2]); // 3
}
```
</details>
<details>
<summary>不定长数组</summary>

不定长数组叫做切片，在定义时通过指定内存地址分配不定长空间，切片数组长度不固定。
```text
fn main() {
    let array: &[u32] = &[1, 2, 3]; // let array = &[1, 2, 3]; 
    println!("{}", array[0]); // 1
    println!("{}", array[1]); // 2
    println!("{}", array[2]); // 3
}
```
</details>

### Tuples
元组数组中各参数的类型可以不一致，用于输入/接收不同类型的数据变量
```text
fn main() {
    let tuple: (bool, u32, i8, f32) = (true, 45, -4, 45.098);
    println!("{}", tuple.0); // true
    println!("{}", tuple.1); // 45
    println!("{}", tuple.2); // -4
    println!("{}", tuple.3); // 45.098
                             // tuple.4 would result in a compilation error!
}
```
### Struct
结构体用于表示通用的数据结构，内部参数可以指定名称或者直接通过类型定义

结构体内部参数在new 新的对象时必须全部初始化新的值。
<details>
<summary>参数定义的结构体</summary>

```text
fn main() {
    struct Mytuple {
        bool_param: bool,
        uint_param: u32,
        int_param: i8,
        float_param: f32,
    }
    let tuple = Mytuple {
        bool_param: true,
        uint_param: 45,
        int_param: -4,
        float_param:45.098 // Error 如果缺失结构体中的浮点数的值的话，就会报错missing `float_param`
    };
    println!("{}", tuple.bool_param); // true
    println!("{}", tuple.uint_param); // 45
    println!("{}", tuple.int_param); // -4
    println!("{}", tuple.float_param); // 45.098  
                                     // tuple.4 would result in a compilation error!
}
```
</details>

<details>
<summary>缺省结构体</summary>

```text
fn main() {
    struct Mytuple(bool, u32, i8, f32);
    let tuple = Mytuple (true, 45, -4, 45.098);
    println!("{}", tuple.0); // true
    println!("{}", tuple.1); // 45
    println!("{}", tuple.2); // -4
    println!("{}", tuple.3); // 45.098
                                     // tuple.4 would result in a compilation error!
}
```
</details>

<details>
<summary> Examples </summary>

```text
struct Person {
    name: &'static str,
    age: u32,
    isman: bool,
}

impl Person {
    fn new() -> Person {
        Person {
            name: "MIke",
            age: 18,
            isman: true,
        }
    }
    fn get_name(&self) -> &'static str {
        self.name
    }
}

struct Person2(&'static str,u32,bool);

impl Person2{
    fn get_name(&self) -> &'static str {
        self.0
    }
}

fn main() {
    let person = Person {
        name: "MIke",
        age: 18,
        isman: true,
    };
    println!("{}", person.get_name());
    let person2 = Person2("Leo",18,true);
    println!("{}", person2.get_name());
}
```
</details>

### ENUM

枚举类型，内部定义可复写的参数状态

<details>
<summary>简单枚举</summary>

```text
#![allow(unused)]
fn main() {
enum CardinalDirection {
    North,
    East,
    South,
    West,
}

let mut d = CardinalDirection::East;
d= CardinalDirection::West;

if let CardinalDirection::East = d {
    println!("We are going east!");
} else {
    println!("We are not going east but in some other direction!");
}
}
```
</details>

<details>
<summary>复杂枚举</summary>

在枚举中增加复杂数据结构。

<code>match</code>关键字进行enum字段的匹配，根据不同的分支处理。

```text
#![allow(unused)]
fn main() {
    enum CardinalDirection {
        Squre { side: f64 },
        Circle { radius: f64 },
        Rectangle { width: f64, height: f64 },
        West,
    }

    let mut d = CardinalDirection::West;

    if let CardinalDirection::West = d {
        println!("We are going west!");
    } else {
        println!("We are not going west but in some other direction!");
    }

    let s = CardinalDirection::Rectangle {
        width: 12.3,
        height: 34.7,
    };
    match s {
        CardinalDirection::Squre { side } => {
            println!("A {}x{} square!", side, side);
        }
        CardinalDirection::Rectangle { width, height } => {
            println!("A {}x{} rectangle!", width, height);
        }
        CardinalDirection::Circle { radius } => {
            println!(
                "A circle of radius {} and diameter {}!",
                radius,
                radius * 2.0
            );
        }
        CardinalDirection::West => {
            println!("We are going west!");
        }
    }
}
```
</details>

### Sequences
表示一组同类型数据的集合，数组`[T;N]`、`Vec <T>`,其中数组表示固定大小的数据序列，但是Vec表示栈数据结构的动态数据增`push` 和 删`pop`。

<details>
<summary>Examples</summary>

```text
fn main() {
    let mut vecValue:Vec<u8> = vec![0,1,2,3];
    println!("{:?}", vecValue.iter());  //Iter([0, 1, 2, 3])
    vecValue.push(8);
    println!("{:?}", vecValue.iter()); //Iter([0, 1, 2, 3, 8])
    println!("{:?}", vecValue.pop()); //8
    println!("{:?}", vecValue.iter());// Iter([0, 1, 2, 3])
        struct Person {
        name: &'static str,
        age: u32,
    }

    let people: Vec<Person> = vec![
        Person {
            name: "MIke",
            age: 18,
        },
        Person {
            name: "Leo",
            age: 19,
        },
    ];

    println!(
        "{:?}",
        people.iter().find(|profile| profile.name == "Leo").unwrap().age
    ); //19  unwrap 表示将数据结构映射出来 
}
```
</details>

### HashMap<K, V> and BTreeMap<K, V>

HashMap<K, V>存储时需要 hash K 值，因此HashMap<K, V> 要求 K 可以被hash。

BTreeMap<K, V>存储时需要按照key值进行排序存储，因此BTreeMap<K, V> 要求 K 是可以排序的。

<details>
<summary>Examples</summary>

```text
fn main() {
    #[derive(Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    let people: Vec<Person> = vec![
        Person {
            name: "Mike".to_string(),
            age: 18,
        },
        Person {
            name: "Leo".to_string(),
            age: 19,
        },
    ];
    let name_to_profile: std::collections::HashMap<String, Person> = people
    // let name_to_profile: std::collections::BTreeMap<String, Person> = people
        .clone()
        .into_iter()
        .map(|profile| (profile.name.clone(), profile))
        .collect();

    println!("{:?}", name_to_profile["Mike"].age); // 30

    println!(
        "{:?}",
        people
            .iter()
            .find(|profile| profile.name == "Leo")
            .unwrap()
            .age
    ); //19  unwrap 表示将数据结构映射出来
}
```
</details>

### Set

`HashSet<T>、BTreeSet<T>`集合存储不重复的数值
> https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.capacity

<details>
<summary>Examples</summary>

```text
use std::collections::HashSet;
use std::collections::BTreeSet;

fn main() {
    let mut cool_numbers = HashSet::from([21, 2, 16]); // 随机排列
    println!("{:?}", cool_numbers.iter()); //[21, 2, 16]
    cool_numbers.insert(8);
    println!("{:?}", cool_numbers.iter()); //[21, 16, 2, 8]
    cool_numbers.insert(16);
    println!("{:?}", cool_numbers.iter()); //[21, 16, 2, 8]
    //Delet data
    cool_numbers.remove(&2);
    println!("{:?}", cool_numbers.iter()); //{16, 21, 8}

    let mut tree_numbers = BTreeSet::from([21, 2, 16]); //顺序排列
    println!("{:?}", tree_numbers.iter()); //2，16，21
    tree_numbers.insert(8);
    println!("{:?}", tree_numbers.iter()); //Iter([2, 8, 16, 21])
    tree_numbers.insert(16);
    println!("{:?}", tree_numbers.iter()); //Iter([2, 8, 16, 21])
     //Delet data
     tree_numbers.remove(&2);
     println!("{:?}", tree_numbers.iter()); //Iter([8, 16, 21])
}
```
</details>