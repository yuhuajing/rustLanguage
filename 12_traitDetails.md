# trait
`trait`关键字为数据结构实现通用的配置,为数据结构绑定额外的数据类型定义和函数。

类似于通用的接口，其余只需要`impx xx for xx`使用即可。


<details>
<summary>Types</summary>

```text
struct Index(i32);
trait Joining {
    // Associated types
    type A;
    type B;
    fn join_to_str(&self, _: &Self::A, _: &Self::B) -> String;
}
impl Joining for Index {
    // Define type of associated types
    type A = String;
    type B = String;
    fn join_to_str(&self, name: &Self::A, last_name: &Self::B) -> String {
        format!("{}. {} {}", self.0, name, last_name)
    }
}
fn get_joined_str<J: Joining>(joining: &J, name: &J::A, last_name: &J::B) -> String {
    format!("Person: {}", joining.join_to_str(name, last_name))
}
fn main() {
    let index = Index(10);
    println!(
        "{}",
        get_joined_str(&index, &"John".to_string(), &"Connor".to_string())
    );
}
```
</details>

<details>
<summary>Constants</summary>

```text
struct Mystruct;
trait ConstantValue {
    // Associated types
    const VALUE:u32;
    fn get_te()->u32;
}
impl ConstantValue for Mystruct {
    // Define type of associated types
    const VALUE:u32=10;
    fn get_te() -> u32 {
        Self::VALUE
    }
}

fn get_value<J: ConstantValue>(_joining: &J) -> String {
    format!("Person: {}", J::get_te())
}

fn main() {
    let mystruct = Mystruct{};
    println!(
        "{}",
        get_value(&mystruct)
    );
}
```
</details>

<details>
<summary>多态</summary>

```text
struct Car;
struct Motorcycle;

trait Vehicle {
    fn get_wheel_count() -> u32;
}

impl Vehicle for Car {
    fn get_wheel_count() -> u32 {
        4
    }
}

impl Vehicle for Motorcycle {
    fn get_wheel_count() -> u32 {
        2
    }
}

fn get_value<J: Vehicle>(_joining: &J) -> String {
    format!("Wheel: {}", J::get_wheel_count())
}

enum tools {
    Car { wheel_count: u32 },
    Motorcycle { wheel_count: u32 },
}
trait Vehicle2 {
    fn new_car() -> Self;
    fn new_motorcycle() -> Self;
    fn wheel_count(&self) -> u32;
}

impl Vehicle2 for tools {
    fn new_car() -> Self {
        Self::Car { wheel_count: 4 }
    }
    fn new_motorcycle() -> Self {
        Self::Car { wheel_count: 2 }
    }
    fn wheel_count(&self) -> u32 {
        match self {
            tools::Car { wheel_count, .. } => *wheel_count,
            tools::Motorcycle { wheel_count, .. } => *wheel_count,
        }
    }
}

fn main() {
    let mystruct = Motorcycle {};
    println!("{}", get_value(&mystruct));

    let mycar = tools::new_car();
    println!("{}", mycar.wheel_count());

    let motorcycle = tools::new_motorcycle();
    println!("{}", motorcycle.wheel_count());
}
```
</details>