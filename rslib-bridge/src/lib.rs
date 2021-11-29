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
use jni::{JNIEnv, JavaVM, NativeMethod};
use std::ffi::{c_void};
use std::{panic, thread};
use prost::{Message};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr::{slice_from_raw_parts_mut, slice_from_raw_parts};
use prost::bytes::BufMut;
use std::mem::MaybeUninit;

// This is a simple macro named `say_hello`.
macro_rules! jmethod{

    ($jmethod: expr, $rmethod:tt, $sig: expr ) => {
        // The macro will expand into the contents of this block.
        NativeMethod {
            name : $jmethod.into(),
            sig : $sig,
            fn_ptr : $rmethod as *mut c_void,

        }
    };
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm : JavaVM, _: *mut c_void) -> jint {
    android_logger::init_once(
        Config::default()
            .with_tag("RustNativeCore")
            .with_min_level(log::Level::Warn),
    );

    let env = vm.get_env().unwrap();
    let jcls = env.find_class("com/linkedin/android/rsdroid/RustCore").unwrap();
    let methods: &[NativeMethod] = &[
        jmethod!("get", get, "(Ljava/lang/String;)Ljava/lang/String;".into()),
        jmethod!("save", save, "(Ljava/lang/String;Ljava/lang/String;)V".into()),
        jmethod!("sledSave", sled_save, "(Ljava/lang/String;Ljava/lang/String;)V".into()),
        jmethod!("run", run, "(I[B[B)[B".into()),
    ];
    env.register_native_methods(jcls, methods).unwrap();


    JNI_VERSION_1_6
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_linkedin_android_rsdroid_RustCore_greeting(
    env: JNIEnv,
    _: JClass,
) -> jstring {
    let output = env
        .new_string("Hello world from Rust world")
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
pub unsafe extern "C" fn save(
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
pub unsafe extern "C" fn sled_save(
    env: JNIEnv,
    _: JClass,
    key : jstring,
    value : jstring,
) {
    let db = db::store();
    let key =env.get_string(key.into()).unwrap();
    let value = env.get_string(value.into()).unwrap();
    db.set(
        key.to_str().unwrap(),
        value.to_str().unwrap(),
    );
}

#[no_mangle]
pub unsafe extern "C" fn get(
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

unsafe fn fill_resp(env : JNIEnv, resp : jbyteArray, code : i32, s : String) -> BackendResult<()> {
    let inner = env.get_byte_array_elements(resp, ReleaseMode::CopyBack)?;
    let org_len = inner.size().unwrap() as usize;
    let mut len_trim = 0;
    if org_len > 16 {
        len_trim = org_len - 16;
    }
    let mut s_trim = s;
    if s_trim.len() > len_trim {
        s_trim = s_trim.split_at(len_trim).0.to_owned()
    }
    let resp = Resp {
        ret : code,
        msg : s_trim
    };
    let mut buf = &mut *slice_from_raw_parts_mut(inner.as_ptr() as *mut u8, org_len);
    resp.encode(&mut buf)?;
    // set end tag for java
    buf.bytes_mut()[0] = MaybeUninit::new(10 as u8);
    buf.advance_mut(1);
    prost::encode_length_delimiter(buf.remaining_mut() - 1, &mut buf)?;
    Ok(())
}


#[no_mangle]
pub unsafe extern "C" fn Java_com_linkedin_android_rsdroid_RustCore_signature(
    env: JNIEnv,
    _: JClass,
) -> jstring {
    let sig = Backend::signature();
    let output = env
        .new_string(sig)
        .expect("Couldn't create java string!");
    output.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn run(
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
        let inner = env.get_byte_array_elements(args, ReleaseMode::NoCopyBack).unwrap();
        let in_bytes = slice_from_raw_parts(inner.as_ptr() as *const u8, inner.size().unwrap() as usize);
        backend.run_command_bytes2_inner_ad(command, &*in_bytes)
    }));
    match result {
        Ok(ret)  => {
            return match ret {
                Ok(s) => env.byte_array_from_slice(&s).unwrap(),
                Err(e) => {
                    fill_resp(env, resp, 100, e.to_string()).unwrap();
                    env.byte_array_from_slice(&[]).unwrap()
                }
            }
        }
        Err(e) => {
            if let Ok(s) = e.downcast::<String>() {
                fill_resp(env, resp, 1000, *s).unwrap();
            }
        }
    }
    env.byte_array_from_slice(&[]).unwrap()
}
