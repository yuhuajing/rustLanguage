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
