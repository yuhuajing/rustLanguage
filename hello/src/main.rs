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
