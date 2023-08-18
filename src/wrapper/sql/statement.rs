use jni::{
    objects::{AutoLocal, JMethodID, JObject, JValueGen},
    signature::{Primitive, ReturnType},
    JNIEnv,
};

use crate::{errors::Error, util};

use super::ResultSet;

pub struct PreparedStatement<'local> {
    inner: JObject<'local>,
    execute_query: JMethodID,
    execute_update: JMethodID,
    close: JMethodID,
    env: JNIEnv<'local>,
}

impl<'local> PreparedStatement<'local> {
    pub fn from_ref(env: &'local mut JNIEnv, statement: JObject<'local>) -> Result<Self, Error> {
        let class = AutoLocal::new(env.find_class("java/sql/PreparedStatement")?, env);

        let execute_query = env.get_method_id(&class, "executeQuery", "()Ljava/sql/ResultSet;")?;
        let execute_update = env.get_method_id(&class, "executeUpdate", "()I")?;

        let close = util::get_close_method_auto(env)?;

        let env = unsafe { env.unsafe_clone() };
        Ok(PreparedStatement {
            inner: statement,
            execute_query,
            execute_update,
            close,
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
}

impl<'a> Drop for PreparedStatement<'a> {
    fn drop(&mut self) {
        util::close(&mut self.env, &self.inner, &self.close)
    }
}
