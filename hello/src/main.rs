/// The book type provided by an external API.
#[derive(Debug)]
enum Option<T>{
    Some(T),
    None,
}

fn first_elem<T>(array: &[T]) -> Option<&T>{
    if array.len()>0{
        Option::Some(&array[0])
    }else{
        Option:None
    }
}

fn main() {
    let a =[1,2,3];
    
}
