use std::sync::Arc;

use jdbc::JvmBuilder;
use jdbc::{Builder, DataSource};
use jni::JavaVM;

#[allow(dead_code)]
pub fn sqlite() -> DataSource {
    Builder::new()
        .vm(VM.clone())
        .jdbc_url("jdbc:sqlite::memory:")
        .build()
        .expect("init datasource error.")
}

#[allow(dead_code)]
pub fn vm() -> JavaVM {
    let libs = concat!(env!("OUT_DIR"), "/libs");

    let vm = JvmBuilder::new()
        .classpath(libs)
        .vm_option("-Duser.timezone=UTC")
        .xmx_mb(128)
        .build()
        .expect("init jvm error.");
    vm
}

lazy_static! {
    pub static ref VM: Arc<JavaVM> = Arc::new(vm());
}

#[macro_export]
macro_rules! init {
    () => {
        #[macro_use]
        extern crate lazy_static;
        lazy_static! {
            static ref DS: std::sync::Arc<jdbc::DataSource> =
                std::sync::Arc::new(crate::util::sqlite());
        }
    };
}
