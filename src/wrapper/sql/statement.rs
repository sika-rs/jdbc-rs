use jni::{
    objects::{AutoLocal, JMethodID, JObject, JValueGen},
    signature::{Primitive, ReturnType},
    sys::jvalue,
    JNIEnv,
};

use crate::{errors::Error, util};

use super::ResultSet;

pub struct PreparedStatement<'local> {
    inner: AutoLocal<'local, JObject<'local>>,
    execute_query: JMethodID,
    execute_update: JMethodID,
    close: JMethodID,
    set_string: JMethodID,
    set_int: JMethodID,
    env: JNIEnv<'local>,
}

impl<'local> PreparedStatement<'local> {
    pub fn from_ref(env: &mut JNIEnv<'local>, statement: JObject<'local>) -> Result<Self, Error> {
        let statement = AutoLocal::new(statement, env);
        let class = AutoLocal::new(env.find_class("java/sql/PreparedStatement")?, env);

        let execute_query = env.get_method_id(&class, "executeQuery", "()Ljava/sql/ResultSet;")?;
        let execute_update = env.get_method_id(&class, "executeUpdate", "()I")?;

        let set_string = env.get_method_id(&class, "setString", "(ILjava/lang/String;)V")?;
        let set_int = env.get_method_id(&class, "setInt", "(II)V")?;

        let close = util::get_close_method_auto(env)?;

        let env = unsafe { env.unsafe_clone() };
        Ok(PreparedStatement {
            inner: statement,
            execute_query,
            execute_update,
            close,
            set_string,
            set_int,
            env,
        })
    }

    pub fn execute_query(&mut self) -> Result<ResultSet, Error> {
        let result = unsafe {
            self.env.call_method_unchecked(
                &self.inner,
                self.execute_query,
                ReturnType::Object,
                &[],
            )?
        };
        if let JValueGen::Object(result) = result {
            return Ok(ResultSet::from_ref(&mut self.env, result)?);
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

    pub fn set_string(&mut self, index: i32, value: &str) -> Result<(), Error> {
        // new String(value)
        let value: JObject<'local> = self.env.new_string(value)?.into();
        self.set_param(self.set_string, index, JValueGen::Object(&value).as_jni())?;
        // del String
        self.env.delete_local_ref(value)?;
        Ok(())
    }
    pub fn set_int(&mut self, index: i32, value: i32) -> Result<(), Error> {
        self.set_param(self.set_int, index, jvalue { i: value })?;
        Ok(())
    }

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

impl<'a> Drop for PreparedStatement<'a> {
    fn drop(&mut self) {
        util::close(&mut self.env, &self.inner, &self.close);
    }
}
