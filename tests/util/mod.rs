use jni::{InitArgsBuilder, JNIVersion, JavaVM};
use std::fs;
pub fn test_vm() -> JavaVM {
    let libs = format!("-Djava.class.path={}", libs().join(";"));

    let jvm_args = InitArgsBuilder::new()
        .version(JNIVersion::V8)
        // .option("-Xcheck:jni")
        .option("-Xmx50m")
        // .option("-Djava.ext.dirs=./libs/sqlite-jdbc-3.42.0.0.jar")
        .option(libs)
        .build()
        .unwrap_or_else(|e| panic!("{:#?}", e));
    let jvm = JavaVM::new(jvm_args).unwrap_or_else(|e| panic!("{:#?}", e));
    return jvm;
}

fn libs() -> Vec<String> {
    let out_dir = concat!(env!("OUT_DIR"), "/libs");
    let mut libs = Vec::new();
    if let Ok(dir) = fs::read_dir(out_dir) {
        for lib in dir {
            if let Ok(lib) = lib {
                let path = lib.path();
                if path.is_dir() {
                    continue;
                }

                let path = path.as_os_str().to_str();
                if let Some(path) = path {
                    libs.push(path.to_string())
                }
                println!("{}", lib.path().as_os_str().to_str().unwrap());
            }
        }
    }
    return libs;
}
