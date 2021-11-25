pub mod backend;
pub mod db;
pub mod lmdb;
pub mod proto;
mod mem;

extern crate log;
use crate::backend::Backend;
use crate::proto::DroidBackendService;
use android_logger::Config;
use backtrace::Backtrace;
use jni::objects::{JByteBuffer, JClass, JObject, ReleaseMode};
use jni::sys::{jbyteArray, jint, jstring, JNI_VERSION_1_6};
use jni::{JNIEnv, JavaVM};
use std::ffi::{c_void, CString};
use std::panic;
use std::panic::{catch_unwind, AssertUnwindSafe};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn JNI_OnLoad(_ : JavaVM, _: *mut c_void) -> jint {
    android_logger::init_once(
        Config::default()
            .with_tag("RustNativeCore")
            .with_min_level(log::Level::Trace),
    );

    JNI_VERSION_1_6
    //let mut env = vm.get_env().unwrap();
    //let mut c = env.find_class("com/linkedin/android/rpc/NativeImpl/RustCore").unwrap();
    //if c.is_null() {
    //    return JNI_ERR;
    //}

    //
    //env.register_native_methods(c, &[NativeMethod {
    //    name: "Java_com_linkedin_android_rsdroid_RustCore_run".into(),
    //    sig : "(ILjava/nio/ByteBuffer;Ljava/nio/ByteBuffer;)I".into(),
    //    fn_ptr : Java_com_linkedin_android_rsdroid_RustCore_run as *mut c_void,
    //}]).unwrap();
    //
    //JNI_VERSION_1_6
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

#[no_mangle]
pub unsafe extern "C" fn Java_com_linkedin_android_rsdroid_RustCore_run(
    env: JNIEnv,
    _: JClass,
    command: jint,
    args: jbyteArray,
    cb: JObject,
) {
    android_logger::init_once(
        Config::default()
            .with_tag("RustNativeCore")
            .with_min_level(log::Level::Trace),
    );
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

    match result {
        Ok(Ok(s)) => {
            let data = env.byte_array_from_slice(&s).unwrap();
            env.call_method(cb, "onSuccess", "([B)V", &[data.into()])
                .unwrap();
            return;
        }
        _ => {
            let world_ptr = CString::new("error").unwrap();
            let output = env
                .new_string(world_ptr.to_str().unwrap())
                .expect("Couldn't create java string!");
            env.call_method(
                cb,
                "onErr",
                "(ILjava/lang/String;)V",
                &[10.into(), output.into()],
            )
            .unwrap();
            return;
        }
    }
}
