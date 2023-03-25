use std::collections::HashMap;
use std::vec;

use jni::objects::{JClass, JMap, JObject, JString, JValue};
use jni::strings::JNIString;
use jni::sys::{jarray, jclass, jint, jobject, jstring};
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

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_RustLibrary_rustyClass(
    // Java environment.
    env: JNIEnv,
    // Static class which owns this method.
    _class: JClass,
) -> jobject {
    // Find the class by name.
    let class_name = "RustyClass";
    let class = env.find_class(class_name).unwrap();

    // Create a new object of the class.
    let object = env.alloc_object(class).unwrap();

    // Set the value of a field in the object.
    let field_name = "message";
    let field_signature = "Ljava/lang/String;";

    let field_name = JNIString::from(field_name);
    let message = env.new_string("RUSTY_JNI_CLASS").unwrap();

    env.set_field(
        object,
        field_name,
        field_signature,
        //JValue::from("RUSTY_JNI_CLASS".to_owned()),
        JValue::Object(JObject::from(message)),
    )
    .unwrap();

    // Return the new object as a jobject.
    object.into_inner()
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_RustLibrary_rustyAcceptReturnClass(
    // Java environment.
    env: JNIEnv,
    // Static class which owns this method.
    _class: JClass,
    input: JString,
) -> jobject {
    let mut input_parsed: String = env.get_string(input).unwrap().into();

    input_parsed = input_parsed + "_" + "RUSTY_JNI_CLASS";

    // Find the class by name.
    let class_name = "RustyAcceptReturnClass";
    let class = env.find_class(class_name).unwrap();

    // Create a new object of the class.
    let object = env.alloc_object(class).unwrap();

    // Set the value of a field in the object.
    let field_name = "message";
    let field_signature = "Ljava/lang/String;";

    let field_name = JNIString::from(field_name);
    let message = env.new_string(input_parsed).unwrap();

    env.set_field(
        object,
        field_name,
        field_signature,
        JValue::Object(JObject::from(message)),
    )
    .unwrap();

    // Return the new object as a jobject.
    object.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_RustLibrary_rustyArray(env: JNIEnv, _: JClass) -> jarray {
    // Create a string array of length 3
    let array_length = 3;
    let array_class = env.find_class("java/lang/String").unwrap();
    let message = env.new_string("FOOBAR").unwrap();
    let initial_value = JObject::from(JString::from(message)).into();
    let jarray = env
        .new_object_array(array_length, array_class, initial_value)
        .unwrap();

    // Set the elements of the array
    let strings = ["ONE", "TWO", "THREE"];
    for (i, s) in strings.iter().enumerate() {
        let jstring = env.new_string(s).unwrap();
        env.set_object_array_element(jarray, i as jint, jstring.into())
            .unwrap();
    }

    jarray
}

/*
#[no_mangle]
pub extern "system" fn Java_RustLibrary_rustyHashMap<'local>(
    env: JNIEnv<'local>,
    _: JClass<'local>,
) -> JObject<'local> {
    // Create a new HashMap to hold key-value pairs
    let mut map = HashMap::new();

    // Populate the map with key-value pairs
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");

    // Convert the HashMap to a Java Map object
    let jmap = env.new_object("java/util/HashMap", "()V", &[]).unwrap();
    for (key, value) in map.iter() {
        let jkey = env.new_string(key).unwrap().into_inner();
        let jvalue = env.new_string(value).unwrap().into_inner();
        env.call_method(
            jmap,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[JObject::from(jkey).into(), JObject::from(jvalue).into()],
        )
        .unwrap();
    }

    // Return the Java Map object
    jmap.into_inner().into()
}
*/
