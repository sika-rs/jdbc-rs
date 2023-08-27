use jdbc::{Builder, Datasource};

#[cfg(feature = "hikari")]
pub fn sqlite() -> Datasource {
    let libs = concat!(env!("OUT_DIR"), "/libs");
    Builder::new()
        .classpath(libs)
        .jdbc_url("jdbc:sqlite::memory:")
        .build()
        .expect("init datasource error.")
}
