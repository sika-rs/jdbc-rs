use super::connection::Connection;
use crate::errors::Error;
use jni::{
    objects::{AutoLocal, JMethodID, JObject, JValueGen},
    signature::ReturnType,
    JNIEnv,
};
pub struct DataSource<'a> {
    inner: AutoLocal<'a, JObject<'a>>,
    env: JNIEnv<'a>,
    get_conn: JMethodID,
}

impl<'a> DataSource<'a> {
    pub fn from_ref(env: &mut JNIEnv<'a>, datasource: JObject<'a>) -> Result<Self, Error> {
        let datasource = AutoLocal::new(datasource, env);
        let class = AutoLocal::new(env.find_class("javax/sql/DataSource")?, env);
        let get_conn: jni::objects::JMethodID =
            env.get_method_id(&class, "getConnection", "()Ljava/sql/Connection;")?;
        let env = unsafe { env.unsafe_clone() };
        Ok(DataSource {
            inner: datasource,
            env,
            get_conn,
        })
    }

    pub fn get_connection(&mut self) -> Result<Connection, Error> {
        let conn = unsafe {
            self.env
                .call_method_unchecked(&self.inner, self.get_conn, ReturnType::Object, &[])
        }?;

        if let JValueGen::Object(obj) = conn {
            return Ok(Connection::from_ref(&mut self.env, obj)?);
        }
        return Err(Error::JniError(jni::errors::Error::WrongJValueType(
            "unknown",
            "java.sql.Connection",
        )));
    }
}
