use std::sync::Arc;

use errors::Error;
use jni::{objects::GlobalRef, JavaVM};

mod builder;
pub mod errors;
pub mod util;
pub mod wrapper;

pub use builder::*;
use wrapper::sql;

#[derive(Debug, Clone)]
pub struct Datasource {
    vm: Arc<JavaVM>,
    inner: GlobalRef,
}

pub use wrapper::sql::Connection;

impl Datasource {
    pub fn new(vm: JavaVM, inner: GlobalRef) -> Self {
        Datasource {
            vm: Arc::new(vm),
            inner,
        }
    }

    pub fn get_connection(&self) -> Result<sql::Connection, Error> {
        let mut env = self.vm.attach_current_thread()?;
        let ds_ref = &*self.inner;
        let mut datasource = sql::DataSource::from_ref(&mut env, ds_ref)?;
        let conn = datasource.get_connection(env)?;
        Ok(conn)
    }
}
