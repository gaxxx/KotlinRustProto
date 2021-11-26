pub mod backend;
pub mod db;
pub mod lmdb;
pub mod proto;
mod mem;

extern crate log;

use crate::backend::Backend;
use crate::proto::{BackendResult, DroidBackendService, Resp};
use android_logger::Config;
use backtrace::Backtrace;
use jni::objects::{JByteBuffer, JClass, JObject, ReleaseMode};
use jni::sys::{jbyteArray, jint, jstring, JNI_VERSION_1_6};
use jni::{JNIEnv, JavaVM};
use std::ffi::{c_void, CString};
use std::{panic, thread};
use prost::Message;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr::slice_from_raw_parts_mut;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn JNI_OnLoad(_ : JavaVM, _: *mut c_void) -> jint {
    android_logger::init_once(
        Config::default()
            .with_tag("RustNativeCore")
            .with_min_level(log::Level::Trace),
    );

    JNI_VERSION_1_6
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_linkedin_android_rsdroid_RustCore_greeting(
    env: JNIEnv,
    _: JClass,
) -> jstring {
    let world_ptr = CString::new("Hello world from Rust world").unwrap();
    let output = env
        .new_string(world_ptr.to_str().unwrap())
        .expect("Couldn't create java string!");
    output.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_linkedin_android_rsdroid_RustCore_emptySet(
    _ : JNIEnv,
    _: JClass,
    _ : jstring,
) {
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_linkedin_android_rsdroid_RustCore_readString(
    env: JNIEnv,
    _: JClass,
    input: jstring,
) {
    env.get_string_utf_chars(input.into()).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_linkedin_android_rsdroid_RustCore_save(
    env: JNIEnv,
    _: JClass,
    key : jstring,
    value : jstring,
) {
    let store = mem::store();
    let mut lk = store.write().unwrap();
    lk.insert(
        env.get_string(key.into()).unwrap().into(),
        env.get_string(value.into()).unwrap().into(),
    );

}

#[no_mangle]
pub unsafe extern "C" fn Java_com_linkedin_android_rsdroid_RustCore_get(
    env: JNIEnv,
    _: JClass,
    key : jstring,
) -> jstring {
    let store = mem::store();
    let lk = store.read().unwrap();
    let key : String = env.get_string(key.into()).unwrap().into();
    let val = lk.get( &key).map(|s| s.clone()).unwrap_or("".to_owned());
    let output = env
        .new_string(val)
        .expect("Couldn't create java string!");
    output.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_linkedin_android_rsdroid_RustCore_emptySetB(
    _ : JNIEnv,
    _: JClass,
    _ : jbyteArray,
) {
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_linkedin_android_rsdroid_RustCore_byteArray(
    env: JNIEnv,
    _: JClass,
) -> jbyteArray {
    return env
        .byte_array_from_slice("Hello world 666".as_bytes())
        .unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_linkedin_android_rsdroid_RustCore_copyByteArray(
    env: JNIEnv,
    _: JClass,
    input: jbyteArray,
) {
    env.convert_byte_array(input).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_linkedin_android_rsdroid_RustCore_readByteArray(
    env: JNIEnv,
    _: JClass,
    input: jbyteArray,
) {
    /*
    env.get_byte_array_elements(input , ReleaseMode::NoCopyBack);
     */
    env.get_primitive_array_critical(input, ReleaseMode::NoCopyBack).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_linkedin_android_rsdroid_RustCore_writeByteArray(
    env: JNIEnv,
    _: JClass,
    input: jbyteArray,
) {
    env.set_byte_array_region(input, 0, std::mem::transmute("Hello") ).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_linkedin_android_rsdroid_RustCore_readByteBuffer(
    env: JNIEnv,
    _: JClass,
    input: JByteBuffer,
) {
    let _b = env.get_direct_buffer_address(input).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_linkedin_android_rsdroid_RustCore_writeByteBuffer(
    env: JNIEnv,
    _: JClass,
    input: JByteBuffer,
) {
    let b = env.get_direct_buffer_address(input).unwrap();
    b[2] = 'c' as u8;
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_linkedin_android_rsdroid_RustCore_empty(_ : JNIEnv, _: JClass) {}

#[no_mangle]
pub unsafe extern "C" fn Java_com_linkedin_android_rsdroid_RustCore_callback(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
) {
    env.call_method(callback, "onSuccess", "()V", &[]).unwrap();
}

unsafe fn fill_resp(env : JNIEnv, resp : jbyteArray, code : i32, s : String) {
    let inner = env.get_boolean_array_elements(resp, ReleaseMode::NoCopyBack).unwrap();
    let org_len = inner.size().unwrap() as usize;
    let mut len_trim = 0;
    if org_len > 16 {
        len_trim = org_len - 16;
    }
    let mut s_trim = s;
    if s_trim.len() > len_trim {
        s_trim = s_trim.split_at(len_trim).0.to_owned()
    }
    log::info!("ready to fill resp org:{} - dst:{} - code:{} - msg: {}", org_len, len_trim, code, s_trim);
    let resp = Resp {
        ret : code,
        msg : s_trim
    };
    let mut item = &mut *slice_from_raw_parts_mut(inner.as_ptr(), org_len);
    resp.encode(&mut item).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_linkedin_android_rsdroid_RustCore_run(
    env: JNIEnv,
    _: JClass,
    command: jint,
    args: jbyteArray,
    resp : jbyteArray
) -> jbyteArray {
    let backend = Backend::new();

    let result: thread::Result<BackendResult<Vec<u8>>> = catch_unwind(AssertUnwindSafe(|| {
        panic::set_hook(Box::new(|_| {
            let backtrace = Backtrace::new();
            log::error!("ops: {:?}", backtrace);
        }));

        let command: u32 = command as u32;
        let in_bytes = env.convert_byte_array(args).unwrap();
        backend.run_command_bytes2_inner_ad(command, &in_bytes)
    }));
    match result {
        Ok(ret)  => {
            return match ret {
                Ok(s) => env.byte_array_from_slice(&s).unwrap(),
                Err(e) => {
                    fill_resp(env, resp, 100, e.to_string());
                    env.byte_array_from_slice(&[]).unwrap()
                }
            }
        }
        Err(e) => {
            if let Ok(s) = e.downcast::<String>() {
                fill_resp(env, resp, 1000, *s)
            }
        }
    }
    env.byte_array_from_slice(&[]).unwrap()
}
