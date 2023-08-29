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
        .xmx_mb(16)
        .build()
        .expect("init jvm error.");
    vm
}

lazy_static! {
    static ref VM: Arc<JavaVM> = Arc::new(vm());
}
