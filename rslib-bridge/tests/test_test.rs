use rsdroid::proto::*;
use rsdroid::{backend};
use env_logger;
#[macro_use] extern crate serial_test;

fn setup() {
    env_logger::try_init().unwrap();
    log::info!("log init");
}

fn teardown() {}

fn read_write() {
    let bd = backend::Backend::new();
    for i in 1..1000 {
        bd.save(SaveIn {
            key : format!("test_{}", i),
            val : format!("value_{}_10086", i),
        }).unwrap();
    }

    for i in 1..1000 {
        let val = bd.get(Str {
            val : format!("test_{}", i),
        }).unwrap();
        assert!(val.val.eq(
            &format!("value_{}_10086", i)
        ))

    }

}

#[test]
#[serial]
pub fn test_lmdb() {
    run_test(|| {
        let bd = backend::Backend::new();
        bd.create(OpenIn{
            path : "/tmp/lmdb".into(),
            mode : 1,
        }).unwrap();
        read_write();
    })
}

#[test]
#[serial]
fn test_sled() {
    run_test(|| {
        let bd = backend::Backend::new();
        bd.create(OpenIn{
            path : "/tmp/sled".into(),
            mode : 2,
        }).unwrap();
        read_write();
    })
}

#[test]
#[serial]
fn test_mem() {
    run_test(|| {
        let bd = backend::Backend::new();
        bd.create(OpenIn{
            path : "".into(),
            mode : 0,
        }).unwrap();
        read_write();
    })
}



fn run_test<T>(test: T) -> ()
    where T: FnOnce() -> ()
{
    setup();

    test();

    teardown();
}