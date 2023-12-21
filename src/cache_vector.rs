use crate::cache_object::{CacheLinked};
use log::{info};

pub enum CacheSizeType{
    SLOTS
}
#[warn(unused_variables)]
pub struct CacheVector {
    cache: Vec<CacheLinked>
}

impl CacheVector {
    pub fn initialize_cache(cache_size: usize, _cache_size_type: CacheSizeType) -> Self{
        info!("Initializing My Own Cache!");
        if cache_size > 0 {
           return Self{
                cache: Vec::with_capacity(100)
            }
        }
        Self{
            cache: Vec::with_capacity(cache_size)
        }
    }
}