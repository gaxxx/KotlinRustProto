use std::borrow::BorrowMut;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use kv;
use kv::{Bucket, Config, Raw, Store};
use once_cell::sync::Lazy;


static mut KV_STORE : Option<Arc<Mutex<Bucket<Raw, Raw>>>> = None;
static KV_LOCK: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

pub fn open(path : &Path) {

    let mut kv_lock = KV_LOCK.lock().unwrap();
    if *kv_lock == true {
        panic!("already opened")
    }
    fs::create_dir_all(&path).unwrap();
    let mut cfg = Config::new(path);
    let store = Store::new(cfg).unwrap();
    *kv_lock = true;
    unsafe {
        *KV_STORE.borrow_mut() = Some(Arc::new(Mutex::new(store.bucket::<Raw, Raw>(None).unwrap())))
    }
}

pub fn store() -> Arc<Mutex<Bucket<'static, Raw, Raw>>> {
    let lock = KV_LOCK.lock().unwrap();
    if !*lock {
        panic!("need open a store");
    }
    unsafe {
        KV_STORE.clone().unwrap()
    }
}
