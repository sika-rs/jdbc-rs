use std::sync::Arc;

use jdbc::JvmBuilder;
use jdbc::{Builder, Datasource};
use jni::JavaVM;

#[allow(dead_code)]
#[cfg(feature = "hikari")]
pub fn sqlite() -> Datasource {
    Builder::new()
        .vm(Arc::new(vm()))
        .jdbc_url("jdbc:sqlite::memory:")
        .build()
        .expect("init datasource error.")
}

#[allow(dead_code)]
pub fn vm() -> JavaVM {
    let libs = concat!(env!("OUT_DIR"), "/libs");

    let vm = JvmBuilder::new()
        .classpath(libs)
        .build()
        .expect("init jvm error.");
    vm
}
