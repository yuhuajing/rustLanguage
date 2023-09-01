# Pattern Matching

## let
结构体、函数等返回多个参数的数据结构中，通过 `let <pattern>: <type> = <expression>;` 从结构复杂数据结构获取特定的数据。

解构数据时必须覆盖全部的分支，结构体中通过 `..`表示结构体内部的缺省；面对具有多个值的enum数据，需要 `else { todo!() }` 关键字表明缺省的其他遍历值 或者 `if let` 表明遍历enum。

<details>
<summary>结构体</summary>

```text
#![allow(unused)]
fn main() {
    struct Person {
        name: &'static str,
        age: u8,
    }
    let milk = Person {
        name: "Mike",
        age: 18,
    };
    let Person { name, .. } = milk; // .. 表示缺省值
    println!("{}.", name);
}
```
</details>


<details>
<summary>Enum</summary>

```text
#![allow(unused)]
fn main() {
    enum Person {
        whitePerson { name: &'static str, age: u8 },
        blackPerson { name: &'static str, age: u8 },
    }

    let milk = Person::whitePerson {
        name: "Mike",
        age: 18,
    };
    let Person::whitePerson { name, .. } = milk else{todo!()}; // 覆盖blackPerson的分支
    println!("{}.", name);

    if let Person::whitePerson { name, .. } = milk { //获取 模式 内部参数
        println!("{}.", name);
    };
}
```
</details>

## match
类似 switch case， `match <pattern>`,内部定义不同处理分支. 

<details>
<summary>match for</summary>

```text
#![allow(unused)]
fn main() {
for n in 0..6 {
    match n {
        1 => println!("It was one!"),
        2 => println!("It was two!"),
        // or-pattern
        3 | 4 => println!("It was a bit more than two!"),
        // match guard
        high if high >= 5 => println!("It was a high number!"), // 不属于上述三个分支并且数值大于等于5的处理分支
        // a pattern consisting only of the name `other`
        other => println!("It was {other}!"), // 不属于上述4个分支的处理分支
    }
}
}
```
</details>


<details>
<summary>match enum</summary>

通过catch遍历enum，内部分支需要定义enum 字段的所有处理分支

```text

#![allow(unused)]
fn main() {
enum Meal {
    FishAndChips { chip_proportion: f64 },
    Hamburger { vegetarian: bool },
}

let meal = Meal::FishAndChips {
    chip_proportion: 0.6,
};

match meal {
    Meal::FishAndChips { chip_proportion } => { // Meal::FishAndChips { chip_proportion } if if chip_proportion > 0.5{}
        if chip_proportion > 0.5 {
            println!("I had some fish and plenty of chips!");
        } else if chip_proportion < 0.5 {
            println!("I had plenty of fish and some chips!");
        } else {
            println!("I had fish and chips!");
        }
    }
    Meal::Hamburger { vegetarian } => {
        if vegetarian {
            println!("I had a vegetarian hamburger!");
        } else {
            println!("I had a meaty hamburger!");
        }
    }
}
}
```
</details>

## while let
`while conditions`循环当 `conditions == false `时终止。

```text
#![allow(unused)]
fn main() {
    enum Meal {
        FishAndChips { chip_proportion: f64 },
        Hamburger { vegetarian: bool },
    }

    let mut meal = Meal::FishAndChips {
        chip_proportion: 0.6,
    };

    while let Meal::FishAndChips { chip_proportion } = meal {
        if chip_proportion > 0.3 {
            println!("I had some fish and plenty of chips!");
            meal = Meal::FishAndChips {
                chip_proportion: chip_proportion - 0.2,
            }
        } else {
            meal = Meal::Hamburger { vegetarian: true } // 循环终止
        }
    }
}
```

## for range遍历
`for <pattern> in <expression> { <body> } ` 遍历数据。

```text
#![allow(unused)]
fn main() {
    let scores: [(&'static str, u32); 3] = [("mike", 87), ("lizi", 56), ("dan", 77)];
    for (name, score) in scores {
        println!("Score of #{name} is {score}!");
    }
    let colors = ["red", "white", "blue"];
    // for (color) in colors {
    //     println!("Color of #{color}!");
    // }
    for (index, color) in colors.into_iter().enumerate() {
        let numbering = index + 1;
        println!("Color #{numbering} is {color}!");
    }
}
```