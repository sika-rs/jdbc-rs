use jni::{InitArgsBuilder, JNIVersion, JavaVM};

pub fn test_vm() -> JavaVM {
    let jvm_args = InitArgsBuilder::new()
        .version(JNIVersion::V8)
        // .option("-Xcheck:jni")
        .option("-Xmx50m")
        // .option("-Djava.ext.dirs=./libs/sqlite-jdbc-3.42.0.0.jar")
        .option(r#"-Djava.class.path=C:\Users\hututu\Desktop\rust\sqlx\sqlx-jdbc\libs\sqlite-jdbc-3.42.0.0.jar;C:\Users\hututu\Desktop\rust\sqlx\sqlx-jdbc\libs\HikariCP-4.0.3.jar;C:\Users\hututu\Desktop\rust\sqlx\sqlx-jdbc\libs\slf4j-api-2.0.7.jar"#)
        .build()
        .unwrap_or_else(|e| panic!("{:#?}", e));
    let jvm = JavaVM::new(jvm_args).unwrap_or_else(|e| panic!("{:#?}", e));
    return jvm;
}
