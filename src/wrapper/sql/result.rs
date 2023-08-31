use bigdecimal::BigDecimal;
use jni::{
    objects::{AutoLocal, GlobalRef, JMethodID, JObject, JValueGen},
    signature::{Primitive, ReturnType},
    sys::jvalue,
    AttachGuard, JNIEnv,
};

use crate::{errors::Error, util, wrapper::io::InputStream, Connection};

use super::ResultSetMetaData;

pub struct ResultSet<'local> {
    inner: GlobalRef,
    get_meta_data: JMethodID,
    get_row: JMethodID,
    next: JMethodID,
    was_null: JMethodID,
    get_string: (JMethodID, JMethodID),
    get_short: (JMethodID, JMethodID),
    get_int: (JMethodID, JMethodID),
    get_long: (JMethodID, JMethodID),
    get_float: (JMethodID, JMethodID),
    get_double: (JMethodID, JMethodID),
    get_boolean: (JMethodID, JMethodID),
    get_date: (JMethodID, JMethodID),
    get_byte: (JMethodID, JMethodID),
    get_bytes: (JMethodID, JMethodID),
    get_big_decimal: (JMethodID, JMethodID),
    get_binary_stream: (JMethodID, JMethodID),
    env: AttachGuard<'local>,
    conn: &'local Connection,
}

impl<'local> ResultSet<'local> {
    pub fn from_ref(conn: &'local Connection, statement: GlobalRef) -> Result<Self, Error> {
        let mut env = conn.env()?;

        let class = AutoLocal::new(env.find_class("java/sql/ResultSet")?, &env);
        let get_meta_data =
            env.get_method_id(&class, "getMetaData", "()Ljava/sql/ResultSetMetaData;")?;
        let get_row = env.get_method_id(&class, "getRow", "()I")?;
        let next = env.get_method_id(&class, "next", "()Z")?;

        let was_null = env.get_method_id(&class, "wasNull", "()Z")?;

        let get_string = env.get_method_id(&class, "getString", "(I)Ljava/lang/String;")?;
        let get_string_by_label = env.get_method_id(
            &class,
            "getString",
            "(Ljava/lang/String;)Ljava/lang/String;",
        )?;

        let get_short = env.get_method_id(&class, "getShort", "(I)S")?;
        let get_short_by_label = env.get_method_id(&class, "getShort", "(Ljava/lang/String;)S")?;

        let get_int = env.get_method_id(&class, "getInt", "(I)I")?;
        let get_int_by_label = env.get_method_id(&class, "getInt", "(Ljava/lang/String;)I")?;

        let get_long = env.get_method_id(&class, "getLong", "(I)J")?;
        let get_long_by_label = env.get_method_id(&class, "getLong", "(Ljava/lang/String;)J")?;

        let get_float = env.get_method_id(&class, "getFloat", "(I)F")?;
        let get_float_by_label = env.get_method_id(&class, "getFloat", "(Ljava/lang/String;)F")?;

        let get_double = env.get_method_id(&class, "getDouble", "(I)D")?;
        let get_double_by_label =
            env.get_method_id(&class, "getDouble", "(Ljava/lang/String;)D")?;

        let get_boolean = env.get_method_id(&class, "getBoolean", "(I)Z")?;
        let get_boolean_by_label =
            env.get_method_id(&class, "getBoolean", "(Ljava/lang/String;)Z")?;

        let get_byte = env.get_method_id(&class, "getByte", "(I)B")?;
        let get_byte_by_label = env.get_method_id(&class, "getByte", "(Ljava/lang/String;)B")?;

        let get_bytes = env.get_method_id(&class, "getBytes", "(I)[B")?;
        let get_bytes_by_label = env.get_method_id(&class, "getBytes", "(Ljava/lang/String;)[B")?;

        let get_big_decimal =
            env.get_method_id(&class, "getBigDecimal", "(I)Ljava/math/BigDecimal;")?;
        let get_big_decimal_by_label = env.get_method_id(
            &class,
            "getBigDecimal",
            "(Ljava/lang/String;)Ljava/math/BigDecimal;",
        )?;

        let get_binary_stream =
            env.get_method_id(&class, "getBinaryStream", "(I)Ljava/io/InputStream;")?;
        let get_binary_stream_by_label = env.get_method_id(
            &class,
            "getBinaryStream",
            "(Ljava/lang/String;)Ljava/io/InputStream;",
        )?;

        let get_date = env.get_method_id(&class, "getTimestamp", "(I)Ljava/sql/Timestamp;")?;
        let get_date_by_label = env.get_method_id(
            &class,
            "getTimestamp",
            "(Ljava/lang/String;)Ljava/sql/Timestamp;",
        )?;

        Ok(ResultSet {
            inner: statement,
            get_meta_data,
            get_row,
            next,
            was_null,
            get_string: (get_string, get_string_by_label),
            get_short: (get_short, get_short_by_label),
            get_int: (get_int, get_int_by_label),
            get_long: (get_long, get_long_by_label),
            get_float: (get_float, get_float_by_label),
            get_double: (get_double, get_double_by_label),
            get_boolean: (get_boolean, get_boolean_by_label),
            get_date: (get_date, get_date_by_label),
            get_byte: (get_byte, get_byte_by_label),
            get_bytes: (get_bytes, get_bytes_by_label),
            get_big_decimal: (get_big_decimal, get_big_decimal_by_label),
            get_binary_stream: (get_binary_stream, get_binary_stream_by_label),
            env,
            conn,
        })
    }

    pub fn get_meta_data(&self) -> Result<ResultSetMetaData<'local>, Error> {
        let mut env = self.conn.env()?;
        let result = unsafe {
            env.call_method_unchecked(&self.inner, self.get_meta_data, ReturnType::Object, &[])?
        };
        if let JValueGen::Object(result) = result {
            return Ok(ResultSetMetaData::from_ref(self.conn, result)?);
        }
        return Err(Error::ImpossibleError);
    }

    pub fn get_row(&self) -> Result<i32, Error> {
        let mut env = self.conn.env()?;
        return util::call::get_int(&mut env, &self.inner, &self.get_row);
    }

    pub fn next(&self) -> Result<bool, Error> {
        let mut env = self.conn.env()?;
        return util::call::get_bool(&mut env, &self.inner, &self.next);
    }

    fn was_null_inner<'a>(&self, env: &'a mut JNIEnv<'local>) -> Result<bool, Error> {
        let value = util::call::get_bool(env, &self.inner, &self.was_null)?;
        Ok(value)
    }

    pub fn was_null<'a>(&self) -> Result<bool, Error> {
        let mut env = self.conn.env()?;
        let value = util::call::get_bool(&mut env, &self.inner, &self.was_null)?;
        Ok(value)
    }

    pub fn get_string(&self, index: i32) -> Result<Option<String>, Error> {
        let method = &self.get_string.0;
        self.use_index(method, index, ReturnType::Object, |env, value| {
            return util::cast::value_cast_string(env, value).map_err(Error::from);
        })
    }

    pub fn get_string_by_label(&self, label: &str) -> Result<Option<String>, Error> {
        let method = &self.get_string.1;
        self.use_label(method, label, ReturnType::Object, |env, value| {
            return util::cast::value_cast_string(env, value).map_err(Error::from);
        })
    }

    pub fn get_short(&self, index: i32) -> Result<Option<i16>, Error> {
        let method = &self.get_short.0;
        let r_type = ReturnType::Primitive(Primitive::Short);
        self.use_index(method, index, r_type, |_: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_i16(value).map_err(Error::from);
        })
    }
    pub fn get_short_by_label(&self, label: &str) -> Result<Option<i16>, Error> {
        let method = &self.get_short.1;
        let r_type = ReturnType::Primitive(Primitive::Short);
        self.use_label(method, label, r_type, |_: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_i16(value).map_err(Error::from);
        })
    }

    pub fn get_int(&self, index: i32) -> Result<Option<i32>, Error> {
        let method = &self.get_int.0;
        let r_type = ReturnType::Primitive(Primitive::Int);
        self.use_index(method, index, r_type, |_: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_i32(value).map_err(Error::from);
        })
    }

    pub fn get_int_by_label(&self, label: &str) -> Result<Option<i32>, Error> {
        let method = &self.get_int.1;
        let r_type = ReturnType::Primitive(Primitive::Int);
        self.use_label(method, label, r_type, |_: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_i32(value).map_err(Error::from);
        })
    }

    pub fn get_long(&self, index: i32) -> Result<Option<i64>, Error> {
        let method = &self.get_long.0;
        let r_type = ReturnType::Primitive(Primitive::Long);
        self.use_index(method, index, r_type, |_: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_i64(value).map_err(Error::from);
        })
    }
    pub fn get_long_by_label(&self, label: &str) -> Result<Option<i64>, Error> {
        let method = &self.get_long.1;
        let r_type = ReturnType::Primitive(Primitive::Long);
        self.use_label(method, label, r_type, |_: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_i64(value).map_err(Error::from);
        })
    }

    pub fn get_float(&self, index: i32) -> Result<Option<f32>, Error> {
        let method = &self.get_float.0;
        let r_type = ReturnType::Primitive(Primitive::Float);
        self.use_index(method, index, r_type, |_: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_f32(value).map_err(Error::from);
        })
    }

    pub fn get_float_by_label(&self, label: &str) -> Result<Option<f32>, Error> {
        let method = &self.get_float.1;
        let r_type = ReturnType::Primitive(Primitive::Float);
        self.use_label(method, label, r_type, |_: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_f32(value).map_err(Error::from);
        })
    }

    pub fn get_double(&self, index: i32) -> Result<Option<f64>, Error> {
        let method = &self.get_double.0;
        let r_type = ReturnType::Primitive(Primitive::Double);
        self.use_index(method, index, r_type, |_: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_f64(value).map_err(Error::from);
        })
    }

    pub fn get_double_by_label(&self, label: &str) -> Result<Option<f64>, Error> {
        let method = &self.get_double.1;
        let r_type = ReturnType::Primitive(Primitive::Double);
        self.use_label(method, label, r_type, |_: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_f64(value).map_err(Error::from);
        })
    }

    pub fn get_boolean(&self, index: i32) -> Result<Option<bool>, Error> {
        let method = &self.get_boolean.0;
        let r_type = ReturnType::Primitive(Primitive::Boolean);
        self.use_index(method, index, r_type, |_: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_bool(value).map_err(Error::from);
        })
    }

    pub fn get_boolean_by_label(&self, label: &str) -> Result<Option<bool>, Error> {
        let method = &self.get_boolean.1;
        let r_type = ReturnType::Primitive(Primitive::Boolean);
        self.use_label(method, label, r_type, |_: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_bool(value).map_err(Error::from);
        })
    }

    pub fn get_byte(&self, index: i32) -> Result<Option<u8>, Error> {
        let method = &self.get_byte.0;
        let r_type = ReturnType::Primitive(Primitive::Byte);
        self.use_index(method, index, r_type, |_: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_u8(value).map_err(Error::from);
        })
    }

    pub fn get_byte_by_label(&self, label: &str) -> Result<Option<u8>, Error> {
        let method = &self.get_byte.1;
        let r_type = ReturnType::Primitive(Primitive::Byte);
        self.use_label(method, label, r_type, |_: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_u8(value).map_err(Error::from);
        })
    }

    pub fn get_bytes(&self, index: i32) -> Result<Option<Vec<u8>>, Error> {
        let method = &self.get_bytes.0;
        let r_type = ReturnType::Array;
        self.use_index(method, index, r_type, |env: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_bytes(env, value).map_err(Error::from);
        })
    }

    pub fn get_bytes_by_label(&self, label: &str) -> Result<Option<Vec<u8>>, Error> {
        let method = &self.get_bytes.1;
        let r_type = ReturnType::Array;
        self.use_label(method, label, r_type, |env: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_bytes(env, value).map_err(Error::from);
        })
    }

    pub fn get_big_decimal(&self, index: i32) -> Result<Option<BigDecimal>, Error> {
        let method = &self.get_big_decimal.0;
        let r_type = ReturnType::Object;
        self.use_index(method, index, r_type, |env: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_big_decimal(env, value).map_err(Error::from);
        })
    }

    pub fn get_big_decimal_by_label(&self, label: &str) -> Result<Option<BigDecimal>, Error> {
        let method = &self.get_big_decimal.1;
        let r_type = ReturnType::Object;
        self.use_label(method, label, r_type, |env: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_big_decimal(env, value).map_err(Error::from);
        })
    }

    pub fn get_binary_stream(&self, index: i32) -> Result<Option<InputStream>, Error> {
        let method = &self.get_binary_stream.0;
        let r_type = ReturnType::Object;
        let vm = self.conn.vm().clone();
        self.use_index(method, index, r_type, |env: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_input_stream(env, value, vm).map_err(Error::from);
        })
    }

    pub fn get_binary_stream_by_label(&self, label: &str) -> Result<Option<InputStream>, Error> {
        let method = &self.get_binary_stream.1;
        let r_type = ReturnType::Object;
        let vm = self.conn.vm().clone();
        self.use_label(method, label, r_type, |env: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_input_stream(env, value, vm).map_err(Error::from);
        })
    }

    pub fn get_timestamp_millis(&self, index: i32) -> Result<Option<i64>, Error> {
        let method = &self.get_date.0;
        let r_type = ReturnType::Object;
        self.use_index(method, index, r_type, |env: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_timestamp_millis(env, value).map_err(Error::from);
        })
    }

    pub fn get_timestamp_millis_by_label(&self, label: &str) -> Result<Option<i64>, Error> {
        let method = &self.get_date.1;
        let r_type = ReturnType::Object;
        self.use_label(method, label, r_type, |env: &mut JNIEnv<'_>, value| {
            return util::cast::value_cast_timestamp_millis(env, value).map_err(Error::from);
        })
    }

    #[cfg(feature = "chrono")]
    pub fn get_local_time(
        &self,
        index: i32,
    ) -> Result<Option<chrono::DateTime<chrono::Local>>, Error> {
        use chrono::{DateTime, Local, TimeZone};
        let timestamp = self.get_timestamp_millis(index)?;
        if let Some(timestamp) = timestamp {
            let datetime: DateTime<Local> = Local
                .timestamp_millis_opt(timestamp)
                .earliest()
                .ok_or(Error::ImpossibleError)?;
            Ok(Some(datetime))
        } else {
            Ok(None)
        }
    }
    #[cfg(feature = "chrono")]
    pub fn get_local_time_by_label(
        &self,
        label: &str,
    ) -> Result<Option<chrono::DateTime<chrono::Local>>, Error> {
        use chrono::{DateTime, Local, TimeZone};
        let timestamp = self.get_timestamp_millis_by_label(label)?;
        if let Some(timestamp) = timestamp {
            let datetime: DateTime<Local> = Local
                .timestamp_millis_opt(timestamp)
                .earliest()
                .ok_or(Error::ImpossibleError)?;
            Ok(Some(datetime))
        } else {
            Ok(None)
        }
    }

    #[cfg(feature = "chrono")]
    pub fn get_utc_time(&self, index: i32) -> Result<Option<chrono::DateTime<chrono::Utc>>, Error> {
        use chrono::{DateTime, TimeZone, Utc};
        let timestamp = self.get_timestamp_millis(index)?;
        if let Some(timestamp) = timestamp {
            let datetime: DateTime<Utc> = Utc
                .timestamp_millis_opt(timestamp)
                .earliest()
                .ok_or(Error::ImpossibleError)?;
            Ok(Some(datetime))
        } else {
            Ok(None)
        }
    }

    #[cfg(feature = "chrono")]
    pub fn get_utc_time_by_label(
        &self,
        label: &str,
    ) -> Result<Option<chrono::DateTime<chrono::Utc>>, Error> {
        use chrono::{DateTime, TimeZone, Utc};
        let timestamp = self.get_timestamp_millis_by_label(label)?;
        if let Some(timestamp) = timestamp {
            let datetime: DateTime<Utc> = Utc
                .timestamp_millis_opt(timestamp)
                .earliest()
                .ok_or(Error::ImpossibleError)?;
            Ok(Some(datetime))
        } else {
            Ok(None)
        }
    }

    fn use_index<'a, T, F>(
        &self,
        method: &JMethodID,
        index: i32,
        r_type: ReturnType,
        f: F,
    ) -> Result<Option<T>, Error>
    where
        F: FnOnce(&mut JNIEnv<'local>, JValueGen<JObject<'local>>) -> Result<T, Error>,
    {
        let mut env = self.conn.env()?;
        // read value
        let value = unsafe {
            env.call_method_unchecked(&self.inner, method, r_type, &[jvalue { i: index }])
                .map_err(Error::from)
        }?;
        if self.was_null_inner(&mut env)? {
            return Ok(None);
        }
        // not null,convert type.
        let v = f(&mut env, value)?;
        return Ok(Some(v));
    }

    fn use_label<'a, T, F>(
        &self,
        method: &JMethodID,
        label: &str,
        r_type: ReturnType,
        f: F,
    ) -> Result<Option<T>, Error>
    where
        F: FnOnce(&mut JNIEnv<'local>, JValueGen<JObject<'local>>) -> Result<T, Error>,
    {
        let mut env = self.conn.env()?;

        let label: JObject<'_> = env.new_string(label)?.into();
        // read value
        let value = unsafe {
            env.call_method_unchecked(
                &self.inner,
                method,
                r_type,
                &[JValueGen::Object(&label).as_jni()],
            )
            .map_err(Error::from)
        }?;
        env.delete_local_ref(label)?;
        let was_null = self.was_null_inner(&mut env)?;
        if was_null {
            return Ok(None);
        }
        // not null,convert type.
        let v = f(&mut env, value)?;
        return Ok(Some(v));
    }
}

impl<'a> Drop for ResultSet<'a> {
    fn drop(&mut self) {
        let _ = util::auto_close(&mut self.env, &self.inner);
    }
}
