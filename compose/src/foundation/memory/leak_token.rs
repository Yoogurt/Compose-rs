use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Mutex;

use lazy_static::lazy_static;

pub(crate) trait LeakableObject {
    fn tag() -> &'static str;
}

lazy_static! {
    static ref TAG_REMEMBER: Mutex<HashMap<&'static str, usize>> = Mutex::new(HashMap::new());
}

#[derive(Debug)]
pub(crate) struct LeakToken<T>
    where
        T: LeakableObject,
{
    tag: &'static str,
    _data: PhantomData<T>,
}

impl<T> Default for LeakToken<T>
    where
        T: LeakableObject,
{
    fn default() -> Self {
        match TAG_REMEMBER.lock().expect("").entry(T::tag()) {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += 1;
            }
            Entry::Vacant(mut entry) => {
                entry.insert(1);
            }
        }
        Self {
            tag: T::tag(),
            _data: PhantomData::default(),
        }
    }
}

impl<T> Drop for LeakToken<T>
    where
        T: LeakableObject,
{
    fn drop(&mut self) {
        match TAG_REMEMBER.lock().expect("").entry(T::tag()) {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() -= 1;
            }
            Entry::Vacant(mut entry) => {
                panic!()
            }
        }
    }
}

pub fn validate_leak() {
    let mut leak_count = 0;

    TAG_REMEMBER
        .lock()
        .expect("")
        .iter()
        .for_each(|(key, &value)| {
            if value != 0 {
                println!("{key} leaks for {value} times");
                leak_count += 1;
            }
        });

    println!("leak analyze complete, found leak objects: {leak_count}");
}
