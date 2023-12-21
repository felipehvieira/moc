
pub struct CacheObject{
    key: i32,
    value: String
}


pub struct CacheLinked {
    value: Option<CacheObject>,
    next: Option<Box<CacheLinked>>
}

impl CacheObject{
    pub fn new(key: i32, value: String) -> Self{
        Self{
            key,
            value
        }
    }
    pub fn key(&self) -> i32{
        self.key
    }
    pub fn value(&self) -> &str
    {
        self.value.as_str()
    }
}

impl CacheLinked {

}