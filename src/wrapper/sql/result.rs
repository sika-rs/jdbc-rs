use jni::{
    objects::{AutoLocal, JMethodID, JObject, JValueGen},
    signature::{Primitive, ReturnType},
    sys::jvalue,
    JNIEnv,
};

use crate::{errors::Error, util, Connection};

use super::ResultSetMetaData;

pub struct ResultSet<'local> {
    inner: AutoLocal<'local, JObject<'local>>,
    get_meta_data: JMethodID,
    get_row: JMethodID,
    next: JMethodID,
    get_string: (JMethodID, JMethodID),
    get_short: (JMethodID, JMethodID),
    get_int: (JMethodID, JMethodID),
    get_long: (JMethodID, JMethodID),
    get_float: (JMethodID, JMethodID),
    get_double: (JMethodID, JMethodID),
    get_boolean: (JMethodID, JMethodID),
    get_date: (JMethodID, JMethodID),
    env: JNIEnv<'local>,
    conn: &'local Connection<'local>,
}

impl<'local> ResultSet<'local> {
    pub fn from_ref(
        conn: &'local Connection<'local>,
        statement: JObject<'local>,
    ) -> Result<Self, Error> {
        let mut env = unsafe { conn.env() };

        let statement = AutoLocal::new(statement, &env);
        let class = AutoLocal::new(env.find_class("java/sql/ResultSet")?, &env);
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

        let get_date = env.get_method_id(&class, "getDate", "(I)Ljava/sql/Date;")?;
        let get_date_by_label =
            env.get_method_id(&class, "getDate", "(Ljava/lang/String;)Ljava/sql/Date;")?;

        Ok(ResultSet {
            inner: statement,
            get_meta_data,
            get_row,
            next,
            get_string: (get_string, get_string_by_label),
            get_short: (get_short, get_short_by_label),
            get_int: (get_int, get_int_by_label),
            get_long: (get_long, get_long_by_label),
            get_float: (get_float, get_float_by_label),
            get_double: (get_double, get_double_by_label),
            get_boolean: (get_boolean, get_boolean_by_label),
            get_date: (get_date, get_date_by_label),
            env,
            conn,
        })
    }

    pub fn get_meta_data(&self) -> Result<ResultSetMetaData<'local>, Error> {
        let mut env = unsafe { self.conn.env() };
        let result = unsafe {
            env.call_method_unchecked(&self.inner, self.get_meta_data, ReturnType::Object, &[])?
        };
        if let JValueGen::Object(result) = result {
            return Ok(ResultSetMetaData::from_ref(self.conn, result)?);
        }
        return Err(Error::ImpossibleError);
    }

    pub fn get_row(&self) -> Result<i32, Error> {
        let mut env = unsafe { self.conn.env() };
        return util::call::get_int(&mut env, &self.inner, &self.get_row);
    }

    pub fn next(&self) -> Result<bool, Error> {
        let mut env = unsafe { self.conn.env() };
        return util::call::get_bool(&mut env, &self.inner, &self.next);
    }

    pub fn get_string(&self, index: i32) -> Result<String, Error> {
        let method = &self.get_string.0;
        let mut env = unsafe { self.conn.env() };
        let value = self.use_index(method, index, ReturnType::Object)?;
        return util::cast::value_cast_string(&mut env, value).map_err(Error::from);
    }
    pub fn get_string_by_label(&self, label: &str) -> Result<String, Error> {
        let mut env = unsafe { self.conn.env() };
        let value = self.use_label(&self.get_string.1, label, ReturnType::Object)?;
        return util::cast::value_cast_string(&mut env, value).map_err(Error::from);
    }

    pub fn get_short(&self, index: i32) -> Result<i16, Error> {
        let method = &self.get_short.0;
        let value = self.use_index(method, index, ReturnType::Primitive(Primitive::Short))?;
        return util::cast::value_cast_i16(value).map_err(Error::from);
    }
    pub fn get_short_by_label(&self, label: &str) -> Result<i16, Error> {
        let method = &self.get_short.1;
        let value = self.use_label(method, label, ReturnType::Primitive(Primitive::Short))?;
        return util::cast::value_cast_i16(value).map_err(Error::from);
    }

    pub fn get_int(&self, index: i32) -> Result<i32, Error> {
        let method = &self.get_int.0;
        let value = self.use_index(method, index, ReturnType::Primitive(Primitive::Int))?;
        return util::cast::value_cast_i32(value).map_err(Error::from);
    }
    pub fn get_int_by_label(&self, label: &str) -> Result<i32, Error> {
        let method = &self.get_int.1;
        let value = self.use_label(method, label, ReturnType::Primitive(Primitive::Int))?;
        return util::cast::value_cast_i32(value).map_err(Error::from);
    }

    pub fn get_long(&self, index: i32) -> Result<i64, Error> {
        let method = &self.get_long.0;
        let value = self.use_index(method, index, ReturnType::Primitive(Primitive::Long))?;
        return util::cast::value_cast_i64(value).map_err(Error::from);
    }
    pub fn get_long_by_label(&self, label: &str) -> Result<i64, Error> {
        let method = &self.get_long.1;
        let value = self.use_label(method, label, ReturnType::Primitive(Primitive::Long))?;
        return util::cast::value_cast_i64(value).map_err(Error::from);
    }

    pub fn get_float(&self, index: i32) -> Result<f32, Error> {
        let method = &self.get_float.0;
        let value = self.use_index(method, index, ReturnType::Primitive(Primitive::Float))?;
        return util::cast::value_cast_f32(value).map_err(Error::from);
    }
    pub fn get_float_by_label(&self, label: &str) -> Result<f32, Error> {
        let method = &self.get_float.1;
        let value = self.use_label(method, label, ReturnType::Primitive(Primitive::Float))?;
        return util::cast::value_cast_f32(value).map_err(Error::from);
    }

    pub fn get_double(&self, index: i32) -> Result<f64, Error> {
        let method = &self.get_double.0;
        let value = self.use_index(method, index, ReturnType::Primitive(Primitive::Double))?;
        return util::cast::value_cast_f64(value).map_err(Error::from);
    }

    pub fn get_double_by_label(&self, label: &str) -> Result<f64, Error> {
        let method = &self.get_double.1;
        let value = self.use_label(method, label, ReturnType::Primitive(Primitive::Double))?;
        return util::cast::value_cast_f64(value).map_err(Error::from);
    }

    pub fn get_boolean(&self, index: i32) -> Result<bool, Error> {
        let method = &self.get_boolean.0;
        let value = self.use_index(method, index, ReturnType::Primitive(Primitive::Boolean))?;
        return util::cast::value_cast_bool(value).map_err(Error::from);
    }

    pub fn get_boolean_by_label(&self, label: &str) -> Result<bool, Error> {
        let method = &self.get_boolean.1;
        let value = self.use_label(method, label, ReturnType::Primitive(Primitive::Boolean))?;
        return util::cast::value_cast_bool(value).map_err(Error::from);
    }

    pub fn get_timestamp_millis(&self, index: i32) -> Result<i64, Error> {
        let method = &self.get_date.0;
        let value = self.use_index(method, index, ReturnType::Object)?;
        let mut env = unsafe { self.conn.env() };
        return util::cast::value_cast_timestamp_millis(&mut env, value).map_err(Error::from);
    }
    pub fn get_timestamp_millis_by_label(&self, label: &str) -> Result<i64, Error> {
        let method = &self.get_date.1;
        let value = self.use_label(method, label, ReturnType::Object)?;
        let mut env = unsafe { self.conn.env() };
        return util::cast::value_cast_timestamp_millis(&mut env, value).map_err(Error::from);
    }

    #[cfg(feature = "chrono")]
    pub fn get_local_time(&self, index: i32) -> Result<chrono::DateTime<chrono::Local>, Error> {
        use chrono::{DateTime, Local, TimeZone};
        let timestamp = self.get_timestamp_millis(index)?;
        let datetime: DateTime<Local> = Local
            .timestamp_millis_opt(timestamp)
            .earliest()
            .ok_or(Error::ImpossibleError)?;
        Ok(datetime)
    }
    #[cfg(feature = "chrono")]
    pub fn get_local_time_by_label(
        &self,
        label: &str,
    ) -> Result<chrono::DateTime<chrono::Local>, Error> {
        use chrono::{DateTime, Local, TimeZone};
        let timestamp = self.get_timestamp_millis_by_label(label)?;
        let datetime: DateTime<Local> = Local
            .timestamp_millis_opt(timestamp)
            .earliest()
            .ok_or(Error::ImpossibleError)?;
        Ok(datetime)
    }

    #[cfg(feature = "chrono")]
    pub fn get_utc_time(&self, index: i32) -> Result<chrono::DateTime<chrono::Utc>, Error> {
        use chrono::{DateTime, TimeZone, Utc};
        let timestamp = self.get_timestamp_millis(index)?;
        let datetime: DateTime<Utc> = Utc
            .timestamp_millis_opt(timestamp)
            .earliest()
            .ok_or(Error::ImpossibleError)?;
        Ok(datetime)
    }
    #[cfg(feature = "chrono")]
    pub fn get_utc_time_by_label(
        &self,
        label: &str,
    ) -> Result<chrono::DateTime<chrono::Utc>, Error> {
        use chrono::{DateTime, TimeZone, Utc};
        let timestamp = self.get_timestamp_millis_by_label(label)?;
        let datetime: DateTime<Utc> = Utc
            .timestamp_millis_opt(timestamp)
            .earliest()
            .ok_or(Error::ImpossibleError)?;
        Ok(datetime)
    }

    fn use_index(
        &self,
        method: &JMethodID,
        index: i32,
        r_type: ReturnType,
    ) -> Result<JValueGen<JObject<'_>>, Error> {
        let mut env = unsafe { self.conn.env() };
        unsafe {
            env.call_method_unchecked(&self.inner, method, r_type, &[jvalue { i: index }])
                .map_err(Error::from)
        }
    }

    fn use_label(
        &self,
        method: &JMethodID,
        label: &str,
        r_type: ReturnType,
    ) -> Result<JValueGen<JObject<'_>>, Error> {
        let mut env = unsafe { self.conn.env() };
        let label: JObject<'_> = env.new_string(label)?.into();
        let value = unsafe {
            env.call_method_unchecked(
                &self.inner,
                method,
                r_type,
                &[JValueGen::Object(&label).as_jni()],
            )
            .map_err(Error::from)?
        };
        env.delete_local_ref(label)?;
        Ok(value)
    }
}

impl<'a> Drop for ResultSet<'a> {
    fn drop(&mut self) {
        let _ = util::auto_close(&mut self.env, &self.inner);
    }
}
