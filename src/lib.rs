extern crate jni;

use jni::objects::{JClass, JString};
use jni::sys::{jint, jstring};
use jni::JNIEnv;

fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_RustLibrary_rustyAdd(
    // Java environment.
    _env: JNIEnv,
    // Static class which owns this method.
    _class: JClass,
    a: jint,
    b: jint,
) -> i32 {
    if a < 0 || b < 0 {
        panic!(); // Handle this differently in production
    }

    println!("INTS ARE: {}, {}", &a, &b);

    add(a as u32, b as u32) as i32
}

fn sort_string(string: &str) -> String {
    let mut chars: Vec<_> = string.chars().collect();

    chars.sort_unstable();

    let sorted: String = chars.iter().collect();

    sorted
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_RustLibrary_sortLetters(
    // Java environment.
    env: JNIEnv,
    // Static class which owns this method.
    _class: JClass,
    // The string which must be sorted
    input: JString,
) -> jstring {
    let input: String = env.get_string(input).unwrap().into();

    let result = sort_string(&input);

    let output = env.new_string(result).unwrap();

    output.into_inner()
}
