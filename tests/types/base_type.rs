// java base type
use jni::objects::{JByteArray, JObject, JValueGen};
use std::ops::Deref;

use crate::{test, wait};

#[macro_export]
macro_rules! test_base_type {
    ($conn:tt,$set:tt,$get:tt,$get_label:tt,$val:expr) => {
        let mut statement = wait!($conn.prepare_statement("select ? as value"))?.$set(1, $val)?;
        let result = wait!(statement.execute_query())?;
        assert_eq!(result.next()?, true);
        assert_eq!(result.$get(1)?, Some($val));
        assert_eq!(result.$get_label("value")?, Some($val));

        let mut statement = wait!($conn.prepare_statement("select NULL"))?;
        let result = wait!(statement.execute_query())?;
        assert_eq!(result.next()?, true);
        assert_eq!(result.$get(1)?, None);
        assert_eq!(result.was_null()?, true);
    };
}

test!(
    fn test() {
        for (db, ds) in crate::util::DB_MAP.iter() {
            let conn = wait!(ds.get_connection())?;
            test_base_type!(conn, set_short, get_short, get_short_by_label, i16::MAX);
            test_base_type!(conn, set_int, get_int, get_int_by_label, i32::MAX);
            test_base_type!(conn, set_long, get_long, get_long_by_label, i64::MAX);

            if db.starts_with("mysql") {
                test_base_type!(
                    conn,
                    set_float,
                    get_float,
                    get_float_by_label,
                    0.33333333333333f32
                );
            } else {
                test_base_type!(conn, set_float, get_float, get_float_by_label, f32::MAX);
            }

            test_base_type!(conn, set_double, get_double, get_double_by_label, f64::MAX);
            test_base_type!(conn, set_boolean, get_boolean, get_boolean_by_label, true);
            test_base_type!(conn, set_byte, get_byte, get_byte_by_label, u8::MAX);
        }
    }
);

test!(
    fn test_byte() {
        // Java Bytes to Rust Vec<u8>
        let mut env = crate::util::VM.attach_current_thread()?;
        let string = "hello world";
        let java_string: JObject = env.new_string(string)?.into();
        let bytes = env.call_method(&java_string, "getBytes", "()[B", &[])?;
        env.delete_local_ref(java_string)?;
        let bytes = jdbc::util::cast::value_cast_bytes(&mut env, bytes)?;
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
    }
);
