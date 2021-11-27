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
use crate::Resp;

static KV_LOCK: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

pub fn open(path: &Path) -> anyhow::Result<Resp>{
    let mut kv_lock = KV_LOCK.lock().unwrap();
    if *kv_lock == true {
        return Ok(Resp {
            ret : 0,
            msg: "".to_string()
        })
    }
    fs::create_dir_all(&path)?;

    // The `Manager` enforces that each process opens the same environment at most once by
    // caching a handle to each environment that it opens. Use it to retrieve the handle
    // to an opened environment—or create one if it hasn't already been opened:
    let mut manager = Manager::<LmdbEnvironment>::singleton().write().unwrap();
    let created_arc = manager
        .get_or_create(path, |path| {
            let mut builder = Lmdb::new();
            builder.set_make_dir_if_needed(true);
            builder.set_map_size(1024 * 1024 * 100);
            Rkv::from_builder(path, builder)
        })?;

    let crc_clone = created_arc.clone();
    let env = crc_clone.read().unwrap();
    let options = StoreOptions::default();
    let store = match env.open_single(None, options) {
        Ok(ret) => ret,
        Err(e) => {
            log::error!("open db error {:?}", e);
            env.open_single(None, StoreOptions::create())?
        }
    };
    *kv_lock = true;
    unsafe {
        *KV.borrow_mut() = Some(created_arc);
        *STORE.borrow_mut() = Some(store);
    }
    Ok(
        Resp {
            ret : 0,
            msg: "".to_string()
        }
    )
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
