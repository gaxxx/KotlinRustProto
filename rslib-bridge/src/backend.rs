use crate::proto::{BackendResult, DroidBackendService, Resp, SaveIn, Str, OpenIn};
use crate::{db, lmdb, mem};
use rkv::Value;

use std::path::Path;
use std::sync::atomic::{AtomicI32, Ordering};
use num_enum::FromPrimitive;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, FromPrimitive)]
#[repr(i8)]
enum End {
    #[num_enum(default)]
    MEM,
    LMDB,
    SLED,
}

static USE_END: AtomicI32 = std::sync::atomic::AtomicI32::new(0);

pub struct Backend {}

impl Backend {
    pub fn new() -> Backend {
        Backend {}
    }
}

impl DroidBackendService for Backend {
    fn create(&self, input: OpenIn) -> BackendResult<Resp> {
        USE_END.store(input.mode, Ordering::Relaxed);
        match End::from(USE_END.load(Ordering::Relaxed) as i8) {
            End::LMDB => return lmdb::open(Path::new(&input.path)),
            End::SLED => {
                log::info!("ready to create {:?}", input);
                db::create(Path::new(&input.path));
            }
            _ => {}
        }
        Ok(Resp {
            ret: 0,
            msg: "".into(),
        })
    }

    fn save(&self, input: SaveIn) -> BackendResult<Resp> {
        match End::from(USE_END.load(Ordering::Relaxed) as i8) {
            End::LMDB => {
                let store = lmdb::store();
                let crc = lmdb::crc();
                let env = crc.read().unwrap();
                let mut writer = env.write().unwrap();
                store
                    .put(&mut writer, input.key, &Value::Str(&input.val))
                    .unwrap();
                writer.commit().unwrap();
            }
            End::SLED => {
                let store = db::store();
                let _ = &store.set(input.key.as_str(), input.val.as_str());
            }
            End::MEM => {
                let store = mem::store();
                let mut wl = store.write().unwrap();
                wl.insert(input.key, input.val);
            }
        }
        Ok(Resp {
            ret: 0,
            msg: "".to_owned(),
        })
    }

    fn get(&self, input: Str) -> BackendResult<Str> {
        match End::from(USE_END.load(Ordering::Relaxed) as i8) {
            End::LMDB => {
                let crc = lmdb::crc();
                let env = crc.read().unwrap();
                let reader = env.read().unwrap();
                let store = lmdb::store();
                if let Some(Value::Str(str)) = store.get(&reader, input.val).unwrap() {
                    return Ok(Str {
                        val: str.to_owned(),
                    });
                } else {
                    return Err(anyhow::anyhow!("not exists"))
                }
            }
            End::SLED => {
                let store = db::store();
                if let Some(raw) = &store.get(input.val.as_str())? {
                    return Ok(Str {
                        val: String::from_utf8(raw.to_vec())?,
                    });
                }
            }
            End::MEM => {
                let store = mem::store();
                let wl = store.read().unwrap();
                return Ok (Str {
                    val: wl.get(&input.val).unwrap_or(&"".to_owned()).clone()
                })
            }
        }
        Ok(Str { val: "".to_owned() })
    }
}
