use lazy_static::lazy_static;
use std::sync::RwLock;
use std::collections::HashSet;
use rand::random;

lazy_static! {
    static ref COMPOSER_HASH : RwLock<HashSet<i64>> = RwLock::new(HashSet::new());
}

pub(crate) fn generate_hash_code() -> i64 {
    let mut hash: i64 = random();

    {
        let mut composer_hash = COMPOSER_HASH.write().unwrap();

        while composer_hash.contains(&hash) {
            hash = random();
        };
        composer_hash.insert(hash.clone());
    }

    return hash;
}