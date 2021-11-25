use rkv::backend::{BackendEnvironmentBuilder, Lmdb, LmdbDatabase, LmdbEnvironment};
use rkv::{Manager, Rkv, SingleStore, StoreOptions};
use std::borrow::BorrowMut;
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use std::sync::{Arc, RwLock};

static mut KV: Option<Arc<RwLock<Rkv<LmdbEnvironment>>>> = None;
static mut STORE: Option<SingleStore<LmdbDatabase>> = None;
use once_cell::sync::Lazy;
static KV_LOCK: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

pub fn open(path: &Path) {
    let mut kv_lock = KV_LOCK.lock().unwrap();
    if *kv_lock == true {
        panic!("already opened")
    }
    fs::remove_dir_all(&path).unwrap();
    fs::create_dir_all(&path).unwrap();

    // The `Manager` enforces that each process opens the same environment at most once by
    // caching a handle to each environment that it opens. Use it to retrieve the handle
    // to an opened environment—or create one if it hasn't already been opened:
    let mut manager = Manager::<LmdbEnvironment>::singleton().write().unwrap();
    let created_arc = manager
        .get_or_create(path, |path| {
            let mut builder = Lmdb::new();
            builder.set_map_size(1024 * 1024 * 1024);
            builder.set_max_dbs(100);
            Rkv::from_builder(path, builder)
        })
        .unwrap();

    let store: SingleStore<LmdbDatabase>;
    {
        let env = created_arc.read().unwrap();
        //let mut option = StoreOptions::<LmdbDatabaseFlags>::create();
        let options = StoreOptions::create();
        store = env.open_single("mydb", options).unwrap();
    }
    *kv_lock = true;
    unsafe {
        *KV.borrow_mut() = Some(created_arc);
        *STORE.borrow_mut() = Some(store);
    }
}

pub fn crc() -> Arc<RwLock<Rkv<LmdbEnvironment>>> {
    let lock = KV_LOCK.lock().unwrap();
    if !*lock {
        panic!("need open a store");
    }
    unsafe { KV.clone().unwrap() }
}

pub fn store() -> SingleStore<LmdbDatabase> {
    let lock = KV_LOCK.lock().unwrap();
    if !*lock {
        panic!("need open a store");
    }
    unsafe { STORE.clone().unwrap() }
}
