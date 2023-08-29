use std::ops::Deref;

use jni::objects::{JByteArray, JObject, JValueGen};

#[macro_use]
extern crate lazy_static;
mod util;

#[cfg(not(feature = "async"))]
#[test]
fn test() -> Result<(), jdbc::errors::Error> {
    let ds = util::sqlite();
    let conn = ds.get_connection()?;

    let statement = conn
        .prepare_statement("select ? as value")?
        .set_short(1, i16::MAX)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_short(1)?, Some(i16::MAX));
    assert_eq!(result.get_short_by_label("value")?, Some(i16::MAX));

    let statement = conn
        .prepare_statement("select ? as value")?
        .set_int(1, i32::MAX)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_int(1)?, Some(i32::MAX));
    assert_eq!(result.get_int_by_label("value")?, Some(i32::MAX));

    let statement = conn
        .prepare_statement("select ? as value")?
        .set_long(1, i64::MAX)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_long(1)?, Some(i64::MAX));
    assert_eq!(result.get_long_by_label("value")?, Some(i64::MAX));

    let statement = conn
        .prepare_statement("select ? as value")?
        .set_float(1, f32::MAX)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_float(1)?, Some(f32::MAX));
    assert_eq!(result.get_float_by_label("value")?, Some(f32::MAX));

    let statement = conn
        .prepare_statement("select ? as value")?
        .set_double(1, f64::MAX)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_double(1)?, Some(f64::MAX));
    assert_eq!(result.get_double_by_label("value")?, Some(f64::MAX));

    let statement = conn
        .prepare_statement("select ? as value")?
        .set_boolean(1, true)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_boolean(1)?, Some(true));
    assert_eq!(result.get_boolean_by_label("value")?, Some(true));

    // Byte
    let statement = conn
        .prepare_statement("select ? as value")?
        .set_byte(1, u8::MAX)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_byte(1)?, Some(u8::MAX));
    assert_eq!(result.get_byte_by_label("value")?, Some(u8::MAX));

    Ok(())
}

#[cfg(not(feature = "async"))]
#[test]
fn test_null() -> Result<(), jdbc::errors::Error> {
    let ds = util::sqlite();
    let conn = ds.get_connection()?;

    let statement = conn.prepare_statement("select NULL")?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_short(1)?, None);
    assert_eq!(result.get_int(1)?, None);
    assert_eq!(result.get_long(1)?, None);
    assert_eq!(result.get_float(1)?, None);
    assert_eq!(result.get_double(1)?, None);
    assert_eq!(result.get_boolean(1)?, None);
    assert_eq!(result.get_byte(1)?, None);
    assert_eq!(result.was_null()?, true);
    Ok(())
}

#[test]
fn test_byte() -> Result<(), jdbc::errors::Error> {
    let mut env = util::VM.attach_current_thread()?;

    // Java Bytes to Rust Vec<u8>
    let string = "hello world";
    let java_string: JObject = env.new_string(string)?.into();
    let bytes = env.call_method(&java_string, "getBytes", "()[B", &[])?;
    env.delete_local_ref(java_string)?;
    let bytes = jdbc::util::cast::value_case_bytes(&mut env, bytes)?;
    assert_eq!(bytes, string.as_bytes());

    // Rust Vec<u8> to Java Bytes
    let byte_ref: &[i8] =
        unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const i8, bytes.len()) };
    let array: JByteArray = env.new_byte_array(bytes.len() as i32)?;
    env.set_byte_array_region(&array, 0, byte_ref)?;
    let java_string = env.new_object(
        "java/lang/String",
        "([B)V",
        &[JValueGen::Object(array.deref())],
    )?;
    let java_string = jdbc::util::cast::obj_cast_string(&mut env, java_string)?;
    assert_eq!(java_string, "hello world");

    Ok(())
}
