use super::connection::Connection;
use crate::errors::Error;
use jni::{
    objects::{AutoLocal, JMethodID, JObject, JValueGen},
    signature::ReturnType,
    AttachGuard, JNIEnv,
};
pub struct DataSource<'local, 'obj> {
    inner: &'obj JObject<'local>,
    env: JNIEnv<'local>,
    get_conn: JMethodID,
}

impl<'local, 'obj_ref> DataSource<'local, 'obj_ref> {
    pub fn from_ref(
        env: &mut JNIEnv<'local>,
        datasource: &'obj_ref JObject<'local>,
    ) -> Result<Self, Error> {
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

    pub fn get_connection(
        &mut self,
        guard: AttachGuard<'local>,
    ) -> Result<Connection<'local>, Error> {
        let conn = unsafe {
            self.env
                .call_method_unchecked(&self.inner, self.get_conn, ReturnType::Object, &[])
        }?;

        if let JValueGen::Object(obj) = conn {
            return Ok(Connection::from_ref(guard, obj)?);
        }
        return Err(Error::JniError(jni::errors::Error::WrongJValueType(
            "unknown",
            "java.sql.Connection",
        )));
    }
}
