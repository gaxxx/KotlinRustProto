mod backend;
mod proto;

#[macro_use]
extern crate jni;
use jni::JNIEnv;
use jni::objects::{JClass, JString, JObject};
use jni::sys::{jbyteArray, jint, jlong, jobjectArray, jarray, jstring};
use std::ffi::CString;
use anyhow;
use std::panic::{catch_unwind, AssertUnwindSafe};
use core::result;
use crate::backend::Backend;
use crate::proto::{DroidBackendService, HelloOut, HelloIn};
use prost::Message;

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
    let mut backend = Backend::new();

    let result = catch_unwind(AssertUnwindSafe(|| {
        let command: u32 = command as u32;
        let in_bytes = env.convert_byte_array(args).unwrap();
        return backend.run_command_bytes2_inner_ad(command, &in_bytes);
    }));


    match result {
        Ok(Ok(_s)) => {
            let data = env.byte_array_from_slice(&_s).unwrap();
            env.call_method(cb, "onSuccess", "([B)V", &[data.into()]);
            return
        }
        _ => {
            let world_ptr = CString::new("error").unwrap();
            let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");
            env.call_method(cb, "onErr", "(ILjava/lang/String;)V", &[10.into(), output.into()]);
            return;
        }
    }
}