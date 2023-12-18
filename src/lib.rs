pub mod next_word_predictor;
pub mod trie;

use self::next_word_predictor::predictor::Predictor;
use self::trie::Trie;

use jni::objects::{JClass, JString};
use jni::sys::jlong;
use jni::JNIEnv;

/*
This script contains JNI-based methods that are used to interface
Java and Rust code. These methods are declared in a Java class, with the package name
visible in the method name, and implemented here, in Rust.
The general form
*/


#[no_mangle]
pub extern "C" fn Java_com_shubham0204_text_1predictor_NextWordPredictor_createNativeInstance(
    mut env: JNIEnv,
    _: JClass,
    filepath: JString,
) -> jlong {
    let corpus_filepath: String = env
        .get_string(&filepath)
        .expect("Could not open filepath in createNativeInstance")
        .into();
    let mut predictor = Predictor::new();
    predictor.load(&corpus_filepath);
    Box::into_raw(Box::new(predictor)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_shubham0204_text_1predictor_NextWordPredictor_deleteNativeInstance(
    _: JNIEnv,
    _: JClass,
    object_ptr: jlong,
) {
    let _ptr = unsafe { Box::from_raw(object_ptr as *mut Predictor) };
}

#[no_mangle]
pub extern "C" fn Java_com_shubham0204_text_1predictor_NextWordPredictor_predictToken<'a>(
    mut env: JNIEnv<'a>,
    _: JClass<'a>,
    object_ptr: jlong,
    token: JString<'a>,
) -> JString<'a> {
    let token: String = env
        .get_string(&token)
        .expect("Could not open token in predictToken")
        .into();
    let ptr = unsafe { &mut *(object_ptr as *mut Predictor) };
    let output = env
        .new_string(ptr.predict(&token))
        .expect("Could not create output string");
    output
}

#[no_mangle]
pub extern "C" fn Java_com_shubham0204_text_1predictor_WordAutoCompletion_createNativeInstance(
    mut env: JNIEnv,
    _: JClass,
    filepath: JString,
) -> jlong {
    let corpus_filepath: String = env
        .get_string(&filepath)
        .expect("Could not open filepath in createNativeInstance")
        .into();
    let mut trie = Trie::new();
    trie.load(&corpus_filepath);
    Box::into_raw(Box::new(trie)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_shubham0204_text_1predictor_WordAutoCompletion_deleteNativeInstance(
    _: JNIEnv,
    _: JClass,
    object_ptr: jlong,
) {
    let _ptr = unsafe { Box::from_raw(object_ptr as *mut Trie) };
}

#[no_mangle]
pub extern "C" fn Java_com_shubham0204_text_1predictor_WordAutoCompletion_predictWord<'a>(
    mut env: JNIEnv<'a>,
    _: JClass<'a>,
    object_ptr: jlong,
    phrase: JString<'a>,
) -> JString<'a> {
    let phrase: String = env
        .get_string(&phrase)
        .expect("Could not open token in predictToken")
        .into();
    let ptr = unsafe { &mut *(object_ptr as *mut Trie) };
    let output = env
        .new_string(ptr.predict(&phrase))
        .expect("Could not create output string");
    output
}
