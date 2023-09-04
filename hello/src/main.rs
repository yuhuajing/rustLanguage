struct Mystruct;
trait ConstantValue {
    // Associated types
    const VALUE:u32;
    fn get_te()->u32;
}
impl ConstantValue for Mystruct {
    // Define type of associated types
    const VALUE:u32=10;
    fn get_te() -> u32 {
        Self::VALUE
    }
}

fn get_value<J: ConstantValue>(_joining: &J) -> String {
    format!("Person: {}", J::get_te())
}

fn main() {
    let mystruct = Mystruct{};
    println!(
        "{}",
        get_value(&mystruct)
    );
}
