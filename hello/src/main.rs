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
        float_param:45.098  // Error 如果缺失结构体中的浮点数的值的话，就会报错missing `float_param`
    };
    println!("{}", tuple.bool_param); // true
    println!("{}", tuple.uint_param); // 45
    println!("{}", tuple.int_param); // -4
    println!("{}", tuple.float_param); // 45.098
                                     // tuple.4 would result in a compilation error!

    //println!("Hello, world!");
}
