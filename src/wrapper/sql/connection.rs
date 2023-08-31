use super::{PreparedStatement, Statement};
use crate::{errors::Error, util};
use jni::{
    objects::{AutoLocal, GlobalRef, JMethodID, JObject, JValueGen},
    signature::ReturnType,
    AttachGuard, JavaVM,
};
use std::sync::Arc;

pub struct Connection {
    inner: GlobalRef,
    vm: Arc<JavaVM>,
    prepare_statement: JMethodID,
    create_statement: JMethodID,
}

impl Connection {
    pub fn from_ref(vm: Arc<JavaVM>, datasource: GlobalRef) -> Result<Self, Error> {
        let (datasource, prepare_statement, create_statement) = {
            let mut env = vm.attach_current_thread()?;
            let class = AutoLocal::new(env.find_class("java/sql/Connection")?, &env);

            let prepare_statement: jni::objects::JMethodID = env.get_method_id(
                &class,
                "prepareStatement",
                "(Ljava/lang/String;)Ljava/sql/PreparedStatement;",
            )?;

            let create_statement: jni::objects::JMethodID =
                env.get_method_id(&class, "createStatement", "()Ljava/sql/Statement;")?;

            (datasource, prepare_statement, create_statement)
        };
        Ok(Connection {
            vm,
            inner: datasource,
            prepare_statement,
            create_statement,
        })
    }

    #[cfg(not(feature = "async"))]
    pub fn prepare_statement<'parent>(
        &'parent self,
        sql: &str,
    ) -> Result<PreparedStatement<'parent>, Error> {
        let statement =
            Self::prepare_statement_ref(&self.vm, &self.inner, &self.prepare_statement, sql)?;
        Ok(PreparedStatement::from_ref(self, statement)?)
    }

    #[cfg(feature = "async")]
    pub async fn prepare_statement<'parent>(
        &'parent self,
        sql: &str,
    ) -> Result<PreparedStatement<'parent>, Error> {
        let sql: String = sql.into();
        let vm = self.vm.clone();
        let inner = self.inner.clone();
        let method = self.prepare_statement.clone();
        let statement = crate::block_on!(move || {
            Self::prepare_statement_ref(&vm, &inner, &method, sql.as_str())
        });
        Ok(PreparedStatement::from_ref(self, statement)?)
    }

    #[cfg(not(feature = "async"))]
    pub fn create_statement<'parent>(&'parent self) -> Result<Statement<'parent>, Error> {
        let statement = Self::create_statement_ref(&self.vm, &self.inner, &self.create_statement)?;
        Ok(Statement::from_ref(self, statement)?)
    }

    #[cfg(feature = "async")]
    pub async fn create_statement<'parent>(&'parent self) -> Result<Statement<'parent>, Error> {
        let vm = self.vm.clone();
        let inner = self.inner.clone();
        let method = self.create_statement.clone();
        let statement = crate::block_on!(move || Self::create_statement_ref(&vm, &inner, &method));
        Ok(Statement::from_ref(self, statement)?)
    }

    fn prepare_statement_ref(
        vm: &Arc<JavaVM>,
        inner: &GlobalRef,
        method: &JMethodID,
        sql: &str,
    ) -> Result<GlobalRef, Error> {
        let mut guard = vm.attach_current_thread()?;
        let sql: JObject<'_> = guard.new_string(sql)?.into();
        let statement = unsafe {
            guard.call_method_unchecked(
                inner,
                method,
                ReturnType::Object,
                &[JValueGen::Object(&sql).as_jni()],
            )?
        };
        guard.delete_local_ref(sql)?;
        if let JValueGen::Object(statement) = statement {
            let global_ref = guard.new_global_ref(statement)?;
            return Ok(global_ref);
        }
        return Err(Error::ImpossibleError);
    }

    fn create_statement_ref(
        vm: &Arc<JavaVM>,
        inner: &GlobalRef,
        method: &JMethodID,
    ) -> Result<GlobalRef, Error> {
        let mut guard = vm.attach_current_thread()?;
        let statement =
            unsafe { guard.call_method_unchecked(inner, method, ReturnType::Object, &[])? };
        if let JValueGen::Object(statement) = statement {
            let global_ref = guard.new_global_ref(statement)?;
            return Ok(global_ref);
        }
        return Err(Error::ImpossibleError);
    }

    pub fn env(&self) -> Result<AttachGuard, Error> {
        let guard = self.vm.attach_current_thread()?;
        Ok(guard)
    }

    pub fn vm(&self) -> &Arc<JavaVM> {
        &self.vm
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        let env = self.env();
        if let Ok(mut env) = env {
            let _ = util::auto_close(&mut env, &self.inner);
        }
    }
}
