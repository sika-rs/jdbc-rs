use super::properties::Properties;
use crate::errors::Error;
use jni::{
    objects::{JObject, JValueGen},
    JNIEnv,
};
pub struct HikariConfig<'a>(JObject<'a>);

impl<'a> HikariConfig<'a> {
    pub fn new(env: &mut JNIEnv<'a>, properties: Properties) -> Result<Self, Error> {
        let properties = properties.into();
        let config = env.new_object(
            "com/zaxxer/hikari/HikariConfig",
            "(Ljava/util/Properties;)V",
            &[JValueGen::Object(&properties)],
        )?;
        env.delete_local_ref(properties)?;
        Ok(HikariConfig(config))
    }
}
impl<'a> Into<JObject<'a>> for HikariConfig<'a> {
    fn into(self) -> JObject<'a> {
        self.0
    }
}

pub struct HikariDataSource<'a>(JObject<'a>);

impl<'a> HikariDataSource<'a> {
    pub fn new(env: &mut JNIEnv<'a>, config: HikariConfig) -> Result<Self, Error> {
        let config = config.0;
        let datasource = env.new_object(
            "com/zaxxer/hikari/HikariDataSource",
            "(Lcom/zaxxer/hikari/HikariConfig;)V",
            &[JValueGen::Object(&config)],
        )?;
        env.delete_local_ref(config)?;
        Ok(HikariDataSource(datasource))
    }
}
impl<'a> Into<JObject<'a>> for HikariDataSource<'a> {
    fn into(self) -> JObject<'a> {
        self.0
    }
}
