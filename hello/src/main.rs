struct Boom;

impl Drop for Boom {
    fn drop(&mut self) {
        panic!("Boom!");
    }
}

fn main() {
    let _boom = Boom;
    panic!("Stop!");
}
