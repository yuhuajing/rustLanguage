use std::collections::HashSet;
use std::collections::BTreeSet;

fn main() {
    let mut cool_numbers = HashSet::from([21, 2, 16]); // 随机排列
    println!("{:?}", cool_numbers.iter()); //[21, 2, 16]
    cool_numbers.insert(8);
    println!("{:?}", cool_numbers.iter()); //[21, 16, 2, 8]
    cool_numbers.insert(16);
    println!("{:?}", cool_numbers.iter()); //[21, 16, 2, 8]
    //Delet data
    cool_numbers.remove(&2);
    println!("{:?}", cool_numbers.iter()); //{16, 21, 8}

    let mut tree_numbers = BTreeSet::from([21, 2, 16]); //顺序排列
    println!("{:?}", tree_numbers.iter()); //2，16，21
    tree_numbers.insert(8);
    println!("{:?}", tree_numbers.iter()); //Iter([2, 8, 16, 21])
    tree_numbers.insert(16);
    println!("{:?}", tree_numbers.iter()); //Iter([2, 8, 16, 21])
     //Delet data
     tree_numbers.remove(&2);
     println!("{:?}", tree_numbers.iter()); //Iter([8, 16, 21])
}
