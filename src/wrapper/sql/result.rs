use jni::{
    objects::{AutoLocal, JMethodID, JObject, JValueGen},
    signature::{Primitive, ReturnType},
    sys::jvalue,
    JNIEnv,
};

use crate::{errors::Error, util};

use super::ResultSetMetaData;

pub struct ResultSet<'local> {
    inner: AutoLocal<'local, JObject<'local>>,
    close: JMethodID,
    get_meta_data: JMethodID,
    get_row: JMethodID,
    next: JMethodID,
    get_string: (JMethodID, JMethodID),
    get_int: (JMethodID, JMethodID),
    get_long: (JMethodID, JMethodID),
    get_float: (JMethodID, JMethodID),
    get_double: (JMethodID, JMethodID),
    env: JNIEnv<'local>,
}

impl<'local> ResultSet<'local> {
    pub fn from_ref(env: &'local mut JNIEnv, statement: JObject<'local>) -> Result<Self, Error> {
        let statement = AutoLocal::new(statement, env);
        let class = env.find_class("java/sql/ResultSet")?;
        let get_meta_data =
            env.get_method_id(&class, "getMetaData", "()Ljava/sql/ResultSetMetaData;")?;
        let get_row = env.get_method_id(&class, "getRow", "()I")?;
        let next = env.get_method_id(&class, "next", "()Z")?;

        let get_string = env.get_method_id(&class, "getString", "(I)Ljava/lang/String;")?;
        let get_string_by_label = env.get_method_id(
            &class,
            "getString",
            "(Ljava/lang/String;)Ljava/lang/String;",
        )?;
        let get_int = env.get_method_id(&class, "getInt", "(I)I")?;
        let get_int_by_label = env.get_method_id(&class, "getInt", "(Ljava/lang/String;)I")?;

        let get_long = env.get_method_id(&class, "getLong", "(I)J")?;
        let get_long_by_label = env.get_method_id(&class, "getLong", "(Ljava/lang/String;)J")?;

        let get_float = env.get_method_id(&class, "getFloat", "(I)F")?;
        let get_float_by_label = env.get_method_id(&class, "getFloat", "(Ljava/lang/String;)F")?;

        let get_double = env.get_method_id(&class, "getDouble", "(I)D")?;
        let get_double_by_label =
            env.get_method_id(&class, "getDouble", "(Ljava/lang/String;)D")?;

        // getMetaData
        let close = util::get_close_method_auto(env)?;
        let env = unsafe { env.unsafe_clone() };
        Ok(ResultSet {
            inner: statement,
            close,
            get_meta_data,
            get_row,
            next,
            get_string: (get_string, get_string_by_label),
            get_int: (get_int, get_int_by_label),
            get_long: (get_long, get_long_by_label),
            get_float: (get_float, get_float_by_label),
            get_double: (get_double, get_double_by_label),
            env,
        })
    }

    pub fn get_meta_data(&mut self) -> Result<ResultSetMetaData<'local>, Error> {
        let result = unsafe {
            self.env.call_method_unchecked(
                &self.inner,
                self.get_meta_data,
                ReturnType::Object,
                &[],
            )?
        };
        if let JValueGen::Object(result) = result {
            return Ok(ResultSetMetaData::from_ref(&mut self.env, result)?);
        }
        return Err(Error::ImpossibleError);
    }

    pub fn get_row(&mut self) -> Result<i32, Error> {
        return util::call::get_int(&mut self.env, &self.inner, &self.get_row);
    }

    pub fn next(&mut self) -> Result<bool, Error> {
        return util::call::get_bool(&mut self.env, &self.inner, &self.next);
    }

    pub fn get_string(&mut self, index: i32) -> Result<String, Error> {
        let method = self.get_string.0;
        let value = use_index!(self, method, index, ReturnType::Object);
        return util::cast::value_cast_string(&mut self.env, value);
    }
    pub fn get_string_by_label(&mut self, label: &str) -> Result<String, Error> {
        let method = self.get_string.1;
        let value = use_label!(self, method, label, ReturnType::Object);
        return util::cast::value_cast_string(&mut self.env, value);
    }

    pub fn get_int(&mut self, index: i32) -> Result<i32, Error> {
        let method = self.get_int.0;
        let value = use_index!(self, method, index, ReturnType::Primitive(Primitive::Int));
        return util::cast::value_cast_i32(value);
    }
    pub fn get_int_by_label(&mut self, label: &str) -> Result<i32, Error> {
        let method = self.get_int.1;
        let value = use_label!(self, method, label, ReturnType::Primitive(Primitive::Int));
        return util::cast::value_cast_i32(value);
    }

    pub fn get_long(&mut self, index: i32) -> Result<i64, Error> {
        let method: JMethodID = self.get_long.0;
        let value = use_index!(self, method, index, ReturnType::Primitive(Primitive::Long));
        return util::cast::value_cast_i64(value);
    }
    pub fn get_long_by_label(&mut self, label: &str) -> Result<i64, Error> {
        let method = self.get_long.1;
        let value = use_label!(self, method, label, ReturnType::Primitive(Primitive::Long));
        return util::cast::value_cast_i64(value);
    }

    pub fn get_float(&mut self, index: i32) -> Result<f32, Error> {
        let method: JMethodID = self.get_float.0;
        let value = use_index!(self, method, index, ReturnType::Primitive(Primitive::Float));
        return util::cast::value_cast_f32(value);
    }
    pub fn get_float_by_label(&mut self, label: &str) -> Result<f32, Error> {
        let method = self.get_float.1;
        let value = use_label!(self, method, label, ReturnType::Primitive(Primitive::Float));
        return util::cast::value_cast_f32(value);
    }

    pub fn get_double(&mut self, index: i32) -> Result<f64, Error> {
        let method: JMethodID = self.get_double.0;
        let value = use_index!(
            self,
            method,
            index,
            ReturnType::Primitive(Primitive::Double)
        );
        return util::cast::value_cast_f64(value);
    }
    pub fn get_double_by_label(&mut self, label: &str) -> Result<f64, Error> {
        let method = self.get_double.1;
        let value = use_label!(
            self,
            method,
            label,
            ReturnType::Primitive(Primitive::Double)
        );
        return util::cast::value_cast_f64(value);
    }
}

use crate::use_index;
use crate::use_label;

#[macro_export]
macro_rules! use_index {
    ($self:ident,$method:ident,$index:ident,$return_type:expr) => {
        unsafe {
            $self.env.call_method_unchecked(
                &$self.inner,
                $method,
                $return_type,
                &[jvalue { i: $index }],
            )?
        }
    };
}
#[macro_export]
macro_rules! use_label {
    ($self:ident,$method:ident,$label:ident,$return_type:expr) => {{
        let label: JObject<'local> = $self.env.new_string($label)?.into();
        let value = unsafe {
            $self.env.call_method_unchecked(
                &$self.inner,
                $method,
                $return_type,
                &[JValueGen::Object(&label).as_jni()],
            )?
        };
        $self.env.delete_local_ref(label)?;
        value
    }};
}

impl<'a> Drop for ResultSet<'a> {
    fn drop(&mut self) {
        util::close(&mut self.env, &self.inner, &self.close)
    }
}
