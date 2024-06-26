use std::collections::HashSet;
use std::sync::RwLock;

use lazy_static::lazy_static;
use rand::random;

lazy_static! {
    static ref COMPOSER_HASH: RwLock<HashSet<u64>> = RwLock::new(HashSet::new());
}

pub(crate) fn generate_hash_code() -> u64 {
    let mut hash: u64 = random();

    {
        let mut composer_hash = COMPOSER_HASH.write().unwrap();

        // hash < 1000 reserve for Composer
        while hash < 1000 && composer_hash.contains(&hash) {
            hash = random();
        }
        composer_hash.insert(hash.clone());
    }

    return hash;
}
