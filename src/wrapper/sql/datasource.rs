use super::connection::Connection;
use crate::errors::Error;
use jni::{
    objects::{AutoLocal, JMethodID, JObject, JValueGen},
    signature::ReturnType,
    JNIEnv,
};
pub struct DataSource<'local> {
    inner: AutoLocal<'local, JObject<'local>>,
    env: JNIEnv<'local>,
    get_conn: JMethodID,
}

impl<'local> DataSource<'local> {
    pub fn from_ref(env: &mut JNIEnv<'local>, datasource: JObject<'local>) -> Result<Self, Error> {
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

    pub fn get_connection(&mut self) -> Result<Connection<'local>, Error> {
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
