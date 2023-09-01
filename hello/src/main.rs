fn append_world(value: &mut String) {
    value.push_str(", World!");
}

fn main() {
    let mut s1 = String::from("Hello");
    let mut w1 = &mut s1;
    let w2 = &mut w1;  //ok
   //  let w2 = &mut s1;  // error : s1 cannot be borrowed mutably more than once at a time.
    append_world(w1);
    println!("The value is now {s1:?}.");
}
