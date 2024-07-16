// use std::collections::HashMap;

struct Viking {
    name: String,
    country: String,
}

// impl Viking {
// fn new(name: &String, country: &String) -> Self {
//     Self {
//         name: name.to_string(),
//         country: country.to_string(),
//     }
// }
// }

use std::hash::{DefaultHasher, Hash, Hasher};

const DEFAULT_MAX_SIZE: u64 = 256;

#[derive(Debug)]
pub struct HashMap<T, V> {
    curr_size: usize,
    arr: [Option<KeyValue<T, V>>; DEFAULT_MAX_SIZE as usize],
}

#[derive(Clone, Debug)]
pub struct KeyValue<T, V> {
    key: T,
    value: V,
    next: Option<Box<KeyValue<T, V>>>,
}

fn hash_key<T: Hash>(key: T) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let hash_val = hasher.finish();
    hash_val
}

impl<T: Clone + std::hash::Hash, V> HashMap<T, V> {
    const INIT: Option<KeyValue<T, V>> = None;
    pub fn new() -> HashMap<T, V> {
        HashMap {
            curr_size: 0,
            arr: [Self::INIT; DEFAULT_MAX_SIZE as usize],
        }
    }

    fn put(&mut self, key: T, value: V) -> Option<V> {
        let hash_val: u64 = hash_key(key.clone());

        println!("\n hash_val : {hash_val:?}");
        let position = hash_val % DEFAULT_MAX_SIZE;
        println!("\n position : {position:?}");

        None
    }

    // fn get(self, key: T) -> Option<V> {}

    // fn remove(self, key: T) -> Option<V> {}

    // fn clear() -> () {}
}

mod tests {
    use super::*;

    #[test]
    pub fn put_search() {
        let key = 1;
        let value = "hello".to_string();

        let mut my_hash: HashMap<i32, String> = HashMap::new();
        my_hash.put(key.clone(), value);
        println!("{:?}", &my_hash);
    }
}
