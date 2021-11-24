pub mod backend;
pub mod proto;
pub mod lmdb;
pub mod db;

extern crate log;
use android_logger::Config;
use jni::JNIEnv;
use jni::objects::{JClass, JObject};
use jni::sys::{jbyteArray, jint, jstring};
use std::ffi::CString;
use std::panic::{catch_unwind, AssertUnwindSafe};
use crate::backend::Backend;
use crate::proto::{DroidBackendService};
use std::panic;
use backtrace::Backtrace;

#[no_mangle]
pub unsafe extern fn Java_com_linkedin_android_rsdroid_RustCore_greeting(env: JNIEnv, _: JClass) -> jstring {
    let world_ptr = CString::new("Hello world from Rust world").unwrap();
    let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");
    output.into_inner()
}

#[no_mangle]
pub unsafe extern fn Java_com_linkedin_android_rsdroid_RustCore_callback(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
) {
    env.call_method(callback, "onSuccess", "()V", &[])
        .unwrap();
}


#[no_mangle]
pub unsafe extern fn Java_com_linkedin_android_rsdroid_RustCore_run(
    env: JNIEnv,
    _: JClass,
    command: jint,
    args: jbyteArray,
    cb : JObject,
) {
    android_logger::init_once(Config::default().with_tag("RustNativeCore").with_min_level(log::Level::Trace));
    let backend = Backend::new();

    let result = catch_unwind(AssertUnwindSafe(|| {
        panic::set_hook(Box::new(|_| {
            let backtrace = Backtrace::new();
            log::error!("ops: {:?}", backtrace);
        }));

        let command: u32 = command as u32;
        let in_bytes = env.convert_byte_array(args).unwrap();
        return backend.run_command_bytes2_inner_ad(command, &in_bytes);
    }));

    if cb.into_inner().is_null() {
        return
    }

    match result {
        Ok(Ok(s)) => {
            let data = env.byte_array_from_slice(&s).unwrap();
            env.call_method(cb, "onSuccess", "([B)V", &[data.into()]).unwrap();
            return
        }
        _ => {
            let world_ptr = CString::new("error").unwrap();
            let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");
            env.call_method(cb, "onErr", "(ILjava/lang/String;)V", &[10.into(), output.into()]).unwrap();
            return;
        }
    }
}