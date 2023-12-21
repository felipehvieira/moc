mod cache_object;
mod cache_vector;

use cache_object::CacheObject;
use crate::cache_vector::{CacheVector, CacheSizeType};


fn main() {
    let z = CacheObject::new(1, String::from("Felipe"));
    println!("{}", z.key());
    println!("{}", z.value());
    CacheVector::initialize_cache(100, CacheSizeType::SLOTS);
}
