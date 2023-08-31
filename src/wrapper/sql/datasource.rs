use std::sync::Arc;

use crate::errors::Error;
use jni::{
    objects::{AutoLocal, GlobalRef, JMethodID, JValueGen},
    signature::ReturnType,
    JavaVM,
};

use super::connection::Connection;

pub struct DataSource {
    vm: Arc<JavaVM>,
    inner: GlobalRef,
    get_conn: JMethodID,
}

unsafe impl Send for DataSource {}

impl DataSource {
    pub fn new(vm: Arc<JavaVM>, inner: GlobalRef) -> Result<Self, jni::errors::Error> {
        let get_conn = {
            let mut env = vm.attach_current_thread()?;
            let class = AutoLocal::new(env.find_class("javax/sql/DataSource")?, &env);
            let get_conn: jni::objects::JMethodID =
                env.get_method_id(&class, "getConnection", "()Ljava/sql/Connection;")?;
            get_conn
        };
        Ok(DataSource {
            vm,
            inner,
            get_conn,
        })
    }

    #[cfg(not(feature = "async"))]
    pub fn get_connection(&self) -> Result<Connection, Error> {
        let conn_ref = Self::get_connection_ref(&self.vm, &self.inner, &self.get_conn)?;
        return Ok(Connection::from_ref(self.vm.clone(), conn_ref)?);
    }

    #[cfg(feature = "async")]
    pub async fn get_connection(&self) -> Result<Connection, Error> {
        let vm = self.vm.clone();
        let ds_ref = self.inner.clone();
        let method = self.get_conn.clone();
        let conn_ref = crate::block_on!(move || Self::get_connection_ref(&vm, &ds_ref, &method));
        return Ok(Connection::from_ref(self.vm.clone(), conn_ref)?);
    }

    fn get_connection_ref(
        vm: &Arc<JavaVM>,
        ds_ref: &GlobalRef,
        method: &JMethodID,
    ) -> Result<GlobalRef, Error> {
        let mut guard = vm.attach_current_thread()?;

        let conn = unsafe { guard.call_method_unchecked(ds_ref, method, ReturnType::Object, &[]) }?;
        if let JValueGen::Object(obj) = conn {
            let global = guard.new_global_ref(obj)?;
            return Ok(global);
        }
        return Err(Error::ImpossibleError);
    }
}
