use std::ops::{Deref, DerefMut};

use jni::{
    objects::{AutoLocal, JMethodID, JObject, JValueGen},
    signature::ReturnType,
    AttachGuard, JNIEnv,
};

use crate::{errors::Error, util};

use super::PreparedStatement;

pub struct Connection<'local> {
    inner: AutoLocal<'local, JObject<'local>>,
    guard: AttachGuard<'local>,
    prepare_statement: JMethodID,
}

impl<'local> Connection<'local> {
    pub fn from_ref(
        mut guard: AttachGuard<'local>,
        datasource: JObject<'local>,
    ) -> Result<Self, Error> {
        let env = guard.deref_mut();
        let datasource = AutoLocal::new(datasource, &env);

        let class = AutoLocal::new(env.find_class("java/sql/Connection")?, &env);
        let prepare_statement: jni::objects::JMethodID = env.get_method_id(
            &class,
            "prepareStatement",
            "(Ljava/lang/String;)Ljava/sql/PreparedStatement;",
        )?;

        Ok(Connection {
            inner: datasource,
            guard,
            prepare_statement,
        })
    }

    pub fn prepare_statement<'parent>(
        &'parent self,
        sql: &str,
    ) -> Result<PreparedStatement<'parent>, Error> {
        let mut env = unsafe { self.env() };
        let sql: JObject<'_> = env.new_string(sql)?.into();
        let statement = unsafe {
            env.call_method_unchecked(
                &self.inner,
                self.prepare_statement,
                ReturnType::Object,
                &[JValueGen::Object(&sql).as_jni()],
            )?
        };
        env.delete_local_ref(sql)?;
        if let JValueGen::Object(statement) = statement {
            return Ok(PreparedStatement::from_ref(self, statement)?);
        }
        return Err(Error::ImpossibleError);
    }

    pub unsafe fn env(&self) -> JNIEnv {
        let env = (&self.guard).deref();
        let env = env.unsafe_clone();
        env
    }
}

impl<'local> Drop for Connection<'local> {
    fn drop(&mut self) {
        let env = self.guard.deref_mut();
        let _ = util::auto_close(env, &self.inner);
    }
}
