
pub fn binary_search<T: Ord + std::fmt::Debug>(input: &[T], target: &T) -> Option<usize> {
    let mut min: i8 = 0;
    let mut max: i8 = input.len() as i8 - 1;
    while min <= max {
        let mid = ((max - min) / 2) + min;
        let mid_index = mid as usize;
        let value = &input[mid_index];
        if value == target {
            println!("\n mid_index : {mid_index:?},value : {value:#?} ");
            return Some(mid_index);
        }
        if value < target {
            min = mid + 1;
        }
        if value > target {
            max = mid - 1
        }
    }
    None
}

// fn main() {

// }