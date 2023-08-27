use jni::{
    objects::{AutoLocal, JMethodID, JObject, JValueGen},
    signature::{Primitive, ReturnType},
    sys::jvalue,
    JNIEnv,
};

use crate::{errors::Error, util, Connection};

use super::ResultSet;

pub struct PreparedStatement<'local> {
    inner: AutoLocal<'local, JObject<'local>>,
    execute_query: JMethodID,
    execute_update: JMethodID,
    set_string: JMethodID,
    set_short: JMethodID,
    set_int: JMethodID,
    set_long: JMethodID,
    set_float: JMethodID,
    set_double: JMethodID,
    set_bool: JMethodID,
    env: JNIEnv<'local>,
    conn: &'local Connection<'local>,
}

impl<'local> PreparedStatement<'local> {
    pub fn from_ref(
        conn: &'local Connection<'local>,
        statement: JObject<'local>,
    ) -> Result<Self, Error> {
        let mut env = unsafe { conn.env() };

        let statement = AutoLocal::new(statement, &env);
        let class = AutoLocal::new(env.find_class("java/sql/PreparedStatement")?, &env);

        let execute_query = env.get_method_id(&class, "executeQuery", "()Ljava/sql/ResultSet;")?;
        let execute_update = env.get_method_id(&class, "executeUpdate", "()I")?;

        let set_string = env.get_method_id(&class, "setString", "(ILjava/lang/String;)V")?;
        let set_short = env.get_method_id(&class, "setShort", "(IS)V")?;
        let set_int = env.get_method_id(&class, "setInt", "(II)V")?;
        let set_long = env.get_method_id(&class, "setLong", "(IJ)V")?;
        let set_float = env.get_method_id(&class, "setFloat", "(IF)V")?;
        let set_double = env.get_method_id(&class, "setDouble", "(ID)V")?;
        let set_bool = env.get_method_id(&class, "setBoolean", "(IZ)V")?;

        Ok(PreparedStatement {
            inner: statement,
            execute_query,
            execute_update,
            set_string,
            set_short,
            set_int,
            set_long,
            set_float,
            set_double,
            set_bool,
            env,
            conn,
        })
    }

    pub fn execute_query(&self) -> Result<ResultSet, Error> {
        let mut env = unsafe { self.conn.env() };
        let result = unsafe {
            env.call_method_unchecked(&self.inner, self.execute_query, ReturnType::Object, &[])?
        };
        if let JValueGen::Object(result) = result {
            return Ok(ResultSet::from_ref(self.conn, result)?);
        }
        return Err(Error::ImpossibleError);
    }

    pub fn execute_update(&mut self) -> Result<i32, Error> {
        let result = unsafe {
            self.env.call_method_unchecked(
                &self.inner,
                self.execute_update,
                ReturnType::Primitive(Primitive::Int),
                &[],
            )?
        };

        if let JValueGen::Int(result) = result {
            return Ok(result);
        }
        return Err(Error::ImpossibleError);
    }

    pub fn set_string(mut self, index: i32, value: &str) -> Result<Self, Error> {
        // new String(value)
        let value: JObject<'local> = self.env.new_string(value)?.into();
        self.set_param(self.set_string, index, JValueGen::Object(&value).as_jni())?;
        // del String
        self.env.delete_local_ref(value)?;
        Ok(self)
    }
    pub fn set_short(mut self, index: i32, value: i16) -> Result<Self, Error> {
        self.set_param(self.set_short, index, jvalue { s: value })?;
        Ok(self)
    }
    pub fn set_int(mut self, index: i32, value: i32) -> Result<Self, Error> {
        self.set_param(self.set_int, index, jvalue { i: value })?;
        Ok(self)
    }
    pub fn set_long(mut self, index: i32, value: i64) -> Result<Self, Error> {
        self.set_param(self.set_long, index, jvalue { j: value })?;
        Ok(self)
    }
    pub fn set_float(mut self, index: i32, value: f32) -> Result<Self, Error> {
        self.set_param(self.set_float, index, jvalue { f: value })?;
        Ok(self)
    }
    pub fn set_double(mut self, index: i32, value: f64) -> Result<Self, Error> {
        self.set_param(self.set_double, index, jvalue { d: value })?;
        Ok(self)
    }
    pub fn set_boolean(mut self, index: i32, value: bool) -> Result<Self, Error> {
        self.set_param(self.set_bool, index, util::cast::bool_to_jvalue(value))?;
        Ok(self)
    }

    #[inline(always)]
    fn set_param(&mut self, method: JMethodID, index: i32, value: jvalue) -> Result<(), Error> {
        unsafe {
            self.env.call_method_unchecked(
                &self.inner,
                method,
                ReturnType::Primitive(Primitive::Void),
                &[jvalue { i: index }, value],
            )?;
        }
        Ok(())
    }
}

impl<'local> Drop for PreparedStatement<'local> {
    fn drop(&mut self) {
     
        let _ = util::auto_close(&mut self.env, &self.inner);
    }
}
