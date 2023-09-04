use std::{cell::RefCell, rc::Rc};

struct Light {
    on: bool,
}

impl Light {
    fn new() -> Self {
        Light { on: false }
    }

    fn turn_on(&mut self) {
        if !self.on {
            self.on = true;
            println!("Turned on the light.");
        }
    }

    fn turn_off(&mut self) {
        if self.on {
            self.on = false;
            println!("Turned off the light.");
        }
    }
}

impl Drop for Light {
    fn drop(&mut self) {
        self.turn_off();
    }
}

struct Person {
    // You have wrapped `Light` in a `RefCell` to provide internal mutability.
    light: Rc<RefCell<Light>>,
}

impl Person {
    fn read_book(&self) {
        // You have to call `RefCell::borrow` here to obtain an immutable reference `&Light`.
        if self.light.borrow().on {
            println!("What a fantastic book!");
        } else {
            println!("It is hard to read without light...");
        }
    }
}

fn main() {
    let light = {
        let mut light = Light::new();
        light.turn_on();
        // Place the light in an `Rc<T>`
        Rc::new(RefCell::new(light))
    };

    let mick = Person {
        // Note that `Light` does not implement `Clone`. You are cloning the
        // smart pointer here, not the value contained within. It is considered
        // good practice to call the clone implementation `Rc::clone` explicitly
        // because it encodes the intent to clone the smart pointer, not the
        // value itself.
        light: Rc::clone(&light),
    };
    let anna = Person {
        light: Rc::clone(&light),
    };

    // The light is on so mick can read.
    mick.read_book();

    // In order to turn off the light, you need a mutable reference `&mut Light`.
    // If any other references were handed out at this point, the program would panic.
    light.borrow_mut().turn_off();

    // The light is off so anna will have trouble reading.
    anna.read_book();
}
