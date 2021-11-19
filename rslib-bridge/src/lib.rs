#[macro_use]
extern crate jni;
use jni::JNIEnv;
use jni::objects::{JClass, JString, JObject};
use jni::sys::{jbyteArray, jint, jlong, jobjectArray, jarray, jstring};
use std::ffi::CString;

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

