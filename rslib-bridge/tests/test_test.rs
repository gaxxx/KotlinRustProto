use rsdroid::proto::*;
use rsdroid::{backend, db, lmdb};
use env_logger;
use std::path::Path;

fn setup() {
    env_logger::init();
    log::info!("log init");
    db::open(Path::new("/tmp/test"));
}

fn teardown() {}

#[test]
fn test_read() {
    run_test(|| {
        let bd = backend::Backend::new();
        let ret = bd.get(Str{
            val : "test".into(),
        });
        assert!(ret.is_ok());
    })
}

#[test]
fn test_write() {
    run_test(|| {
        let bd = backend::Backend::new();
        let ret = bd.save(SaveIn {
            key : "key".into(),
            val : "value".into()
        });
        assert!(ret.is_ok());
        let ret = bd.get(Str{
            val : "key".into(),
        }).unwrap();
        assert!(ret.val.eq("value"));
    })
}

fn run_test<T>(test: T) -> ()
    where T: FnOnce() -> ()
{
    setup();

    test();

    teardown();
}