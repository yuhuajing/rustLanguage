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
