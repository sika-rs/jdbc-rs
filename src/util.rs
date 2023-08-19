use jni::{
    objects::{AutoLocal, JMethodID, JObject, JString, JValueGen},
    signature::ReturnType,
    JNIEnv,
};
use log::error;

use crate::errors::Error;

#[inline(always)]
pub fn delete_value<'a>(env: &mut JNIEnv<'a>, val: JValueGen<JObject<'_>>) -> Result<(), Error> {
    if let JValueGen::Object(obj) = val {
        env.delete_local_ref(obj)?
    }
    Ok(())
}

#[inline(always)]
pub fn get_close_method_auto<'a>(env: &mut JNIEnv<'a>) -> Result<JMethodID, Error> {
    let closeable = AutoLocal::new(env.find_class("java/lang/AutoCloseable")?, env);
    let close: jni::objects::JMethodID = env.get_method_id(&closeable, "close", "()V")?;
    Ok(close)
}

#[inline(always)]
pub fn close<'a>(env: &mut JNIEnv<'a>, obj: &JObject<'a>, method: &JMethodID) {
    let data = unsafe { env.call_method_unchecked(obj, method, ReturnType::Object, &[]) };
    if let Err(err) = data {
        error!("Resource closing failed. {}", err);
    }
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
        JValueGen::Object(name) => get_string(env, name),
        _ => Err(Error::ImpossibleError),
    }
}

pub fn get_string<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Result<String, Error> {
    let name = JString::from(obj);
    let name_str = env.get_string(&name)?;
    let string = String::from(name_str);
    Ok(string)
}

pub mod cast {
    use jni::{
        objects::{JObject, JString, JValueGen},
        JNIEnv,
    };

    use crate::errors::Error;

    pub fn value_cast_string<'a>(
        env: &mut JNIEnv<'a>,
        obj: JValueGen<JObject<'a>>,
    ) -> Result<String, Error> {
        if let JValueGen::Object(obj) = obj {
            return obj_cast_string(env, obj);
        }
        Err(Error::WrongType)
    }

    use crate::value_cast;
    value_cast!(JValueGen::Int, i32, value_cast_i32);
    value_cast!(JValueGen::Long, i64, value_cast_i64);
    value_cast!(JValueGen::Float, f32, value_cast_f32);
    value_cast!(JValueGen::Double, f64, value_cast_f64);

    #[macro_export]
    macro_rules! value_cast {
        ($type:path,$return_type:tt,$fun_name:ident) => {
            pub fn $fun_name<'a>(obj: JValueGen<JObject<'a>>) -> Result<$return_type, Error> {
                if let $type(val) = obj {
                    return Ok(val);
                }
                Err(Error::WrongType)
            }
        };
    }

    pub fn obj_cast_string<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Result<String, Error> {
        let name = JString::from(obj);
        let name_str = env.get_string(&name)?;
        let string = String::from(name_str);
        Ok(string)
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
