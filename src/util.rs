use jni::{
    errors::Error,
    objects::{JObject, JValueGen},
    signature::ReturnType,
    JNIEnv,
};
use log::error;

#[inline(always)]
pub fn delete_value<'a>(env: &mut JNIEnv<'a>, val: JValueGen<JObject<'_>>) -> Result<(), Error> {
    if let JValueGen::Object(obj) = val {
        env.delete_local_ref(obj)?
    }
    Ok(())
}

#[inline(always)]
pub fn to_string<'a>(env: &mut JNIEnv<'a>, obj: &JObject<'a>) -> Result<String, Error> {
    let string_obj = env.call_method(obj, "toString", "()Ljava/lang/String;", &[])?;
    let string = cast::value_cast_string(env, string_obj)?;
    Ok(string)
}

#[inline(always)]
pub fn auto_close<'a>(env: &mut JNIEnv<'a>, obj: &JObject<'a>) -> Result<(), Error> {
    let autoclose = env.find_class("java/lang/AutoCloseable")?;
    if !env.is_instance_of(obj, &autoclose)? {
        return Ok(());
    }
    let method = env.get_method_id(autoclose, "close", "()V")?;

    let data = unsafe { env.call_method_unchecked(obj, method, ReturnType::Object, &[]) };
    if let Err(err) = data {
        error!("Resource closing failed. {}", err);
    }
    Ok(())
}

#[inline(always)]
pub fn get_class_name<'a>(env: &mut JNIEnv<'a>, obj: &JObject<'a>) -> Result<String, Error> {
    let obj_class = JObject::from(env.get_object_class(obj)?);
    let class = env.find_class("java/lang/Class")?;
    let method = env.get_method_id(&class, "getName", "()Ljava.lang.String;")?;

    let name = unsafe { env.call_method_unchecked(&obj_class, method, ReturnType::Object, &[])? };

    env.delete_local_ref(obj_class)?;
    env.delete_local_ref(class)?;
    match name {
        JValueGen::Object(name) => cast::obj_cast_string(env, name),
        _ => Err(Error::JavaException),
    }
}

pub mod cast {
    use std::sync::Arc;

    use bigdecimal::BigDecimal;
    use jni::errors::Error;
    use jni::objects::{AutoLocal, JByteArray, ReleaseMode};
    use jni::signature::{Primitive, ReturnType};
    use jni::sys::{jvalue, JNI_TRUE};
    use jni::JavaVM;
    use jni::{
        objects::{JObject, JString, JValueGen},
        JNIEnv,
    };

    pub fn bool_to_jvalue(value: bool) -> jvalue {
        let value = match value {
            true => jni::sys::JNI_TRUE,
            false => jni::sys::JNI_FALSE,
        };
        jvalue { z: value }
    }

    pub fn u8_to_jvalue(value: u8) -> jvalue {
        jvalue { b: value as i8 }
    }

    pub fn jvalue_to_bool(value: jvalue) -> bool {
        unsafe {
            match value {
                jvalue { z } => z == jni::sys::JNI_TRUE,
            }
        }
    }

    pub fn value_cast_string<'a>(
        env: &mut JNIEnv<'a>,
        obj: JValueGen<JObject<'a>>,
    ) -> Result<String, Error> {
        if let JValueGen::Object(obj) = obj {
            return obj_cast_string(env, obj);
        }
        Err(Error::JavaException)
    }

    pub fn value_cast_timestamp_millis<'a>(
        env: &mut JNIEnv<'a>,
        obj: JValueGen<JObject<'a>>,
    ) -> Result<i64, Error> {
        if let JValueGen::Object(obj) = obj {
            let class = AutoLocal::new(env.find_class("java/util/Date")?, &env);
            let method = env.get_method_id(&class, "getTime", "()J")?;
            unsafe {
                let value = env.call_method_unchecked(
                    &obj,
                    method,
                    ReturnType::Primitive(Primitive::Long),
                    &[],
                )?;
                env.delete_local_ref(obj)?;
                return value_cast_i64(value);
            }
        }
        return Err(Error::JavaException);
    }

    pub fn value_cast_bytes<'a>(
        env: &mut JNIEnv<'a>,
        obj: JValueGen<JObject<'a>>,
    ) -> Result<Vec<u8>, Error> {
        if let jni::objects::JValueGen::Object(obj) = obj {
            let array = JByteArray::from(obj);
            return array_cast_bytes(env, array);
        }
        Ok(Vec::new())
    }
    pub fn array_cast_bytes<'a>(
        env: &mut JNIEnv<'a>,
        obj: JByteArray<'a>,
    ) -> Result<Vec<u8>, Error> {
        unsafe {
            let vec = {
                let array = env.get_array_elements(&obj, ReleaseMode::NoCopyBack)?;
                let len = array.len();
                let mut vec = Vec::with_capacity(len);
                for byte in array.iter() {
                    vec.push(*byte as u8);
                }
                vec
            };
            env.delete_local_ref(obj)?;
            return Ok(vec);
        }
    }

    pub fn value_cast_big_decimal<'a>(
        env: &mut JNIEnv<'a>,
        obj: JValueGen<JObject<'a>>,
    ) -> Result<BigDecimal, Error> {
        if let jni::objects::JValueGen::Object(obj) = obj {
            let string = super::to_string(env, &obj)?;
            env.delete_local_ref(obj)?;
            return Ok(string.parse().unwrap_or(BigDecimal::from(0)));
        }

        Ok(BigDecimal::from(0))
    }
    pub fn value_cast_input_stream<'a>(
        env: &mut JNIEnv<'a>,
        obj: JValueGen<JObject<'a>>,
        vm: Arc<JavaVM>,
    ) -> Result<InputStream, Error> {
        let obj = {
            match obj {
                JValueGen::Object(obj) => obj,
                _ => JObject::null(),
            }
        };
        let obj = env.new_global_ref(obj)?;
        Ok(InputStream::new(obj, vm))
    }

    use crate::value_cast;
    use crate::wrapper::io::InputStream;
    value_cast!(JValueGen::Byte, u8, value_cast_u8);
    value_cast!(JValueGen::Char, u16, value_cast_char);
    value_cast!(JValueGen::Bool, bool, value_cast_bool);
    value_cast!(JValueGen::Short, i16, value_cast_i16);
    value_cast!(JValueGen::Int, i32, value_cast_i32);
    value_cast!(JValueGen::Long, i64, value_cast_i64);
    value_cast!(JValueGen::Float, f32, value_cast_f32);
    value_cast!(JValueGen::Double, f64, value_cast_f64);

    #[macro_export]
    macro_rules! value_cast {
        (JValueGen::Bool,$return_type:tt,$fun_name:ident) => {
            pub fn $fun_name<'a>(obj: JValueGen<JObject<'a>>) -> Result<$return_type, Error> {
                if let JValueGen::Bool(val) = obj {
                    return Ok(val == JNI_TRUE);
                }
                Err(Error::JavaException)
            }
        };
        (JValueGen::Byte,$return_type:tt,$fun_name:ident) => {
            pub fn $fun_name<'a>(obj: JValueGen<JObject<'a>>) -> Result<$return_type, Error> {
                if let JValueGen::Byte(val) = obj {
                    return Ok(val as u8);
                }
                Err(Error::JavaException)
            }
        };
        ($type:path,$return_type:tt,$fun_name:ident) => {
            pub fn $fun_name<'a>(obj: JValueGen<JObject<'a>>) -> Result<$return_type, Error> {
                if let $type(val) = obj {
                    return Ok(val);
                }
                Err(Error::JavaException)
            }
        };
    }

    pub fn obj_cast_string<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Result<String, Error> {
        let name = JString::from(obj);
        let name_str = env.get_string(&name)?;
        let string = String::from(name_str);
        env.delete_local_ref(name)?;
        Ok(string)
    }

    pub fn vec_to_bytes_array<'a>(
        env: &mut JNIEnv<'a>,
        bytes: &[u8],
    ) -> Result<JByteArray<'a>, Error> {
        let byte_ref: &[i8] =
            unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const i8, bytes.len()) };
        let array: JByteArray = env.new_byte_array(bytes.len() as i32)?;
        env.set_byte_array_region(&array, 0, byte_ref)?;
        Ok(array)
    }
}

pub mod call {
    use jni::{
        objects::{JMethodID, JObject, JValueGen},
        signature::{Primitive, ReturnType},
        JNIEnv,
    };

    use crate::errors::Error;

    #[inline(always)]
    pub fn get_int<'a>(
        env: &mut JNIEnv<'a>,
        obj: &JObject<'a>,
        method: &JMethodID,
    ) -> Result<i32, Error> {
        let int = unsafe {
            env.call_method_unchecked(obj, method, ReturnType::Primitive(Primitive::Int), &[])?
        };
        if let JValueGen::Int(count) = int {
            return Ok(count);
        }
        return Err(Error::ImpossibleError);
    }

    #[inline(always)]
    pub fn get_bool<'a>(
        env: &mut JNIEnv<'a>,
        obj: &JObject<'a>,
        method: &JMethodID,
    ) -> Result<bool, Error> {
        let bool = unsafe {
            env.call_method_unchecked(obj, method, ReturnType::Primitive(Primitive::Boolean), &[])?
        };
        if let JValueGen::Bool(bool) = bool {
            return Ok(bool > 0);
        }
        return Err(Error::ImpossibleError);
    }
}
