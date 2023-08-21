use jni::{
    objects::{AutoLocal, JMethodID, JObject, JValueGen},
    signature::ReturnType,
    JNIEnv,
};

use crate::{errors::Error, util};

use super::PreparedStatement;

pub struct Connection<'local> {
    inner: AutoLocal<'local, JObject<'local>>,
    env: JNIEnv<'local>,
    prepare_statement: JMethodID,
    close: JMethodID,
}

impl<'local> Connection<'local> {
    pub fn from_ref(env: &mut JNIEnv<'local>, datasource: JObject<'local>) -> Result<Self, Error> {
        let datasource = AutoLocal::new(datasource, env);

        let class = AutoLocal::new(env.find_class("java/sql/Connection")?, env);
        let prepare_statement: jni::objects::JMethodID = env.get_method_id(
            &class,
            "prepareStatement",
            "(Ljava/lang/String;)Ljava/sql/PreparedStatement;",
        )?;

        let close = util::get_close_method_auto(env)?;

        let env = unsafe { env.unsafe_clone() };
        Ok(Connection {
            inner: datasource,
            env,
            prepare_statement,
            close,
        })
    }

    pub fn prepare_statement(&mut self, sql: &str) -> Result<PreparedStatement<'local>, Error> {
        let sql: JObject<'_> = self.env.new_string(sql)?.into();
        let statement = unsafe {
            self.env.call_method_unchecked(
                &self.inner,
                self.prepare_statement,
                ReturnType::Object,
                &[JValueGen::Object(&sql).as_jni()],
            )?
        };
        self.env.delete_local_ref(sql)?;
        if let JValueGen::Object(statement) = statement {
            return Ok(PreparedStatement::from_ref(&mut self.env, statement)?);
        }
        return Err(Error::ImpossibleError);
    }
}

impl<'local> Drop for Connection<'local> {
    fn drop(&mut self) {
        util::close(&mut self.env, &self.inner, &self.close)
    }
}
