use once_cell::sync::Lazy;
use std::path::Path;
use std::sync::{RwLock, Arc};
use std::collections::HashMap;

static KV_STORE: Lazy<Arc<RwLock<HashMap<String, String>>>> = Lazy::new(|| {
    Arc::new(RwLock::new(HashMap::new()))
});

pub fn create(_ : &Path) {
}

pub fn store() -> Arc<RwLock<HashMap<String, String>>> {
    KV_STORE.clone()
}
