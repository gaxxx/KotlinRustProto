use kv;
use kv::{Bucket, Config, Raw, Store};
use once_cell::sync::Lazy;
use std::borrow::BorrowMut;
use std::fs;
use std::path::Path;
use std::sync::{RwLock};

static mut KV_STORE: Option<Bucket<Raw, Raw>> = None;
static KV_LOCK: Lazy<RwLock<bool>> = Lazy::new(|| RwLock::new(false));

pub fn create(path: &Path) {
    let mut kv_lock = KV_LOCK.write().unwrap();
    if *kv_lock == true {
        panic!("already opened")
    }
    fs::remove_dir_all(&path).unwrap();
    fs::create_dir_all(&path).unwrap();
    let cfg = Config::new(path);
    let store = Store::new(cfg).unwrap();
    *kv_lock = true;
    unsafe { *KV_STORE.borrow_mut() = Some(store.bucket::<Raw, Raw>(None).unwrap()) }
}

pub fn store() -> Bucket<'static, Raw, Raw> {
    let lock = KV_LOCK.read().unwrap();
    if !*lock {
        panic!("need open a store");
    }
    unsafe { KV_STORE.clone().unwrap() }
}
