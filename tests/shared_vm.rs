use std::sync::Arc;

use jdbc::{errors::InitError, Builder, JvmBuilder};
use jni::errors::{JniError, StartJvmError};

mod util;

#[test]
fn test_create() -> Result<(), jdbc::errors::InitError> {
    // create a jvm
    let vm = util::vm();

    assert!(matches!(
        // Create jvm again
        JvmBuilder::new().build(),
        Err(InitError::StartJvmError(StartJvmError::Create(
            jni::errors::Error::JniCall(JniError::AlreadyCreated),
        )))
    ));

    assert!(matches!(
        // Create Datasource
        Builder::new().jdbc_url("jdbc:sqlite::memory:").build(),
        Err(InitError::StartJvmError(StartJvmError::Create(
            jni::errors::Error::JniCall(JniError::AlreadyCreated),
        )))
    ));

    let vm = Arc::new(vm);

    // Shared VM
    let _ = Builder::new()
        .jdbc_url("jdbc:sqlite::memory:")
        .vm(vm.clone())
        .build()?;
    let _ = Builder::new()
        .jdbc_url("jdbc:sqlite::memory:")
        .vm(vm.clone())
        .build()?;

    Ok(())
}
