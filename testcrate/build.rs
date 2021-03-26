extern crate envcache;

use envcache::EnvCache;

fn main() {
    let mut envcache = EnvCache::new();
    envcache.cache("TEST");
}
