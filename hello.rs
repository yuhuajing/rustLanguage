fn main () {
    struct Config{
        port:u8,
    }
    let config = Config{port:10};
    let config_reference = &config;

    println!("Using port {}.", config_reference.port);
}
// compile and run the program
// rustc main.rs