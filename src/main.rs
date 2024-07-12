mod binary_search;
use crate::binary_search::binary_search;

fn main() {
    println!("Hello, world!");
    let mut input = [6, 8, 2, 9, 4, 0, 1].to_vec();
    input.sort();
    let y = binary_search::<i32>(&input, &4);
    println!("\n y : {y:?}")
}
