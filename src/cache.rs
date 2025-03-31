use lru::LruCache;
use std::sync::Mutex;
use std::net::IpAddr;
use std::usize;
use std::num::NonZeroUsize;
use std::sync::PoisonError;

pub struct  GeoCache {
    cache: Mutex<LruCache<IpAddr, String>>,
}

impl GeoCache {
    pub fn new(size: usize) -> Self {
        let cache_size = NonZeroUsize::new(size).unwrap();
        Self {
            cache: Mutex::new(LruCache::new(cache_size)),
        }
    }

    pub fn get(&self, ip:IpAddr) -> Option<String> {
        self.cache.lock().unwrap_or_else(PoisonError::into_inner).get(&ip).cloned()
    }

    pub fn insert(&self, ip:IpAddr, result:String) {
        self.cache.lock().unwrap_or_else(PoisonError::into_inner).put(ip, result);
    }
}