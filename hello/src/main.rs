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

    let c = |x| x * 2;
    println!("{}", c(6));
}

fn perform_operation(should_add: bool, amount_to_add: i32, value: i32) -> i32 {
    let operation: Box<dyn Fn(i32) -> i32> = if should_add {
        Box::new(|value: i32| value + amount_to_add)
    } else {
        Box::new(|value: i32| value + 1)
    };

    operation(value)
}

