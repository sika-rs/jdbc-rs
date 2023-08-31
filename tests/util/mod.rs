use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;

use jdbc::JvmBuilder;
use jdbc::{Builder, DataSource};
use jni::JavaVM;

#[allow(dead_code)]
pub fn sqlite() -> Arc<DataSource> {
    DB_MAP.get("sqlite").expect("Can not get sqlite.").clone()
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

fn read_all_db() -> HashMap<String, Arc<DataSource>> {
    let mut toml = String::new();
    let _ = std::fs::File::open("test_db.toml")
        .expect("Read toml error.")
        .read_to_string(&mut toml);
    let toml: HashMap<String, HashMap<String, String>> = toml::from_str(&toml).unwrap();
    let mut map = HashMap::new();
    for (db, config) in toml.iter() {
        let mut builder = Builder::new().vm(VM.clone());
        for (key, value) in config.iter() {
            builder = builder.property(key, value);
        }
        let ds = builder
            .build()
            .expect(format!("Init database error. DB:{}", db).as_str());
        map.insert(db.clone(), Arc::new(ds));
    }
    map
}

lazy_static! {
    pub static ref VM: Arc<JavaVM> = Arc::new(vm());
    pub static ref DB_MAP: HashMap<String, Arc<DataSource>> = read_all_db();
}

#[macro_export]
#[cfg(feature = "async-std")]
macro_rules! block_on {
    ($($t:tt)*) => {
      async_std::task::block_on(async {
        $($t)*
      });
    };
}
#[macro_export]
#[cfg(feature = "tokio")]
macro_rules! block_on {
    ($($t:tt)*) => {
      tokio_test::block_on(async {
        $($t)*
      });
    };
}

#[cfg(feature = "async")]
#[macro_export]
macro_rules! test {
    (fn $fn:tt() { $($t:tt)* }) => {
         #[test]
         fn $fn() {
            async fn async_fn() -> Result<(), jdbc::errors::Error> {
                $($t)*
                Ok(())
            }

            #[cfg(feature = "tokio")]
            tokio_test::block_on(async {
                async_fn().await.expect("");
            });
            #[cfg(feature = "async-std")]
            async_std::task::block_on(async {
                async_fn().await.expect("");
            });
        }
    };
}

#[cfg(feature = "async")]
#[macro_export]
macro_rules! wait {
    ($e:expr) => {
        $e.await
    };
}
#[cfg(not(feature = "async"))]
#[macro_export]
macro_rules! wait {
    ($e:expr) => {
        $e
    };
}
#[cfg(not(feature = "async"))]
#[macro_export]
macro_rules! test {
    (fn $fn:tt() { $($t:tt)* }) => {
         #[test]
         fn $fn() -> Result<(), jdbc::errors::Error> {
            $($t)*
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! init {
    () => {
        #[macro_use]
        extern crate lazy_static;
        lazy_static! {
            pub static ref DS: std::sync::Arc<jdbc::DataSource> = crate::util::sqlite();
        }
    };
}
