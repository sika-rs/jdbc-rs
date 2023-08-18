use jni::{
    objects::{JMethodID, JObject, JValueGen},
    signature::ReturnType,
    JNIEnv,
};

use crate::{errors::Error, util};

use super::ResultSetMetaData;

pub struct ResultSet<'local> {
    inner: JObject<'local>,
    close: JMethodID,
    get_meta_data: JMethodID,
    get_row: JMethodID,
    env: JNIEnv<'local>,
}

impl<'local> ResultSet<'local> {
    pub fn from_ref(env: &'local mut JNIEnv, statement: JObject<'local>) -> Result<Self, Error> {
        let class = env.find_class("java/sql/ResultSet")?;
        let get_meta_data =
            env.get_method_id(&class, "getMetaData", "()Ljava/sql/ResultSetMetaData;")?;
        let get_row = env.get_method_id(&class, "getRow", "()I")?;

        // getMetaData
        let close = util::get_close_method_auto(env)?;
        let env = unsafe { env.unsafe_clone() };
        Ok(ResultSet {
            inner: statement,
            close,
            get_meta_data,
            get_row,
            env,
        })
    }

    pub fn get_meta_data(&mut self) -> Result<ResultSetMetaData, Error> {
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
}

impl<'a> Drop for ResultSet<'a> {
    fn drop(&mut self) {
        util::close(&mut self.env, &self.inner, &self.close)
    }
}
