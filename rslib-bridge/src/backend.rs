use crate::proto::{BackendResult, DroidBackendService, SaveIn, Str, Resp};
use crate::{db, lmdb};
use rkv::{Value};

use std::path::{Path};
use crate::backend::End::NOOP;


enum End {
    NOOP,
    LMDB,
    SLED
}

const useEnd : End = End::NOOP;

pub struct Backend {
}

impl Backend {
    pub fn new() -> Backend {
        Backend{}
    }
}

impl DroidBackendService for Backend {
    fn open(&self, input: Str) -> BackendResult<Resp> {
        match useEnd {
            End::LMDB=> {
                lmdb::open(Path::new(&input.val))
            }
            End::SLED => {
                db::open(Path::new(&input.val));
            }
            _ => {}
        }
        Ok(Resp{
            ret : 0,
            msg: "".into(),
        })
    }

    fn save(&self, input: SaveIn) -> BackendResult<Resp> {
        match useEnd {
            End::LMDB => {
                let store = lmdb::store();
                let crc = lmdb::crc();
                let env = crc.read().unwrap();
                let mut writer = env.write().unwrap();
                store.put(&mut writer, input.key, &Value::Str(&input.val)).unwrap();
                writer.commit().unwrap();
            }
            End::SLED => {
                let lk_store = db::store();
                let store = lk_store.lock().unwrap();
                &store.set(input.key.as_str(), input.val.as_str());
            }
            _ => {
            }
        }
        Ok(Resp {
            ret : 0,
            msg : "".to_owned(),
        })
    }

    fn get(&self, input: Str) -> BackendResult<Str> {
        match useEnd {
            End::LMDB => {
                let crc = lmdb::crc();
                let env = crc.read().unwrap();
                let reader = env.read().unwrap();
                let store = lmdb::store();
                if let Some(Value::Str(str)) = store.get(&reader, input.val).unwrap() {
                    return Ok(Str {
                        val: str.to_owned()
                    })
                } else {}
            }
            End::SLED => {
                let lk_store = db::store();
                let store = lk_store.lock().unwrap();
                let val = &store.get(input.val.as_str())?;
                if let Some(raw) = &store.get(input.val.as_str())? {
                    return Ok(
                        Str {
                            val : String::from_utf8(raw.to_vec())?,
                        }
                    )
                }
            },
            _ => {
            }
        }
        Ok (
            Str{val : "".into()}
        )
    }
}
