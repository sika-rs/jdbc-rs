use std::{collections::HashMap, sync::Arc};

pub use jni::JNIVersion;
use jni::{
    objects::{GlobalRef, JObject},
    InitArgsBuilder, JavaVM,
};

use crate::{
    errors::InitError,
    wrapper::{
        hikari::{HikariConfig, HikariDataSource},
        properties::Properties,
    },
    DataSource,
};

type FactoryFn =
    Box<dyn Fn(&JavaVM, &HashMap<String, String>) -> Result<GlobalRef, InitError> + 'static>;

pub struct Builder {
    properties: HashMap<String, String>,
    pool_type: PoolType,
    factory: Option<FactoryFn>,
    vm: Option<Arc<JavaVM>>,
}

pub enum PoolType {
    HikariCP,
    Custom,
}

fn hikari(vm: &JavaVM, properties: &HashMap<String, String>) -> Result<GlobalRef, InitError> {
    let mut env = vm.attach_current_thread()?;
    let mut props = Properties::new(&mut env)?;
    for (key, value) in properties {
        props.set_property(key.as_str(), value.as_str())?;
    }
    let config = HikariConfig::new(&mut env, props)?;
    let datasource = HikariDataSource::new(&mut env, config)?;
    let datasource: JObject = datasource.into();
    let global_ref = env.new_global_ref(datasource)?;

    Ok(global_ref)
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            properties: HashMap::new(),
            pool_type: PoolType::HikariCP,
            factory: None,
            vm: None,
        }
    }

    pub fn pool_type(mut self, pool_type: PoolType) -> Self {
        self.pool_type = pool_type;
        self
    }

    pub fn factory(mut self, factory: FactoryFn) -> Self {
        self.factory = Some(factory);
        self
    }

    pub fn vm(mut self, vm: Arc<JavaVM>) -> Self {
        self.vm = Some(vm);
        self
    }

    pub fn property(mut self, k: &str, v: &str) -> Self {
        self.properties.insert(k.to_owned(), v.to_owned());
        self
    }

    pub fn jdbc_url(mut self, url: &str) -> Self {
        self.properties.insert("jdbcUrl".to_owned(), url.to_owned());
        self
    }

    pub fn driver_class(mut self, url: &str) -> Self {
        self.properties
            .insert("driverClassName".to_owned(), url.to_owned());
        self
    }

    pub fn username(mut self, username: &str) -> Self {
        self.properties
            .insert("username".to_owned(), username.to_owned());
        self
    }

    pub fn password(mut self, password: &str) -> Self {
        self.properties
            .insert("password".to_owned(), password.to_owned());
        self
    }

    pub fn build(self) -> Result<DataSource, InitError> {
        let vm = {
            if let Some(vm) = self.vm {
                vm
            } else {
                Arc::new(JvmBuilder::new().build()?)
            }
        };

        let datasource = {
            match self.pool_type {
                PoolType::HikariCP => hikari(&vm, &self.properties),
                PoolType::Custom => {
                    if let Some(factory) = self.factory {
                        (*factory)(&vm, &self.properties)
                    } else {
                        Err(InitError::NoFactory)
                    }
                }
            }
        }?;
        {
            check_datasource(&vm, &datasource)?;
        }
        let datasource = DataSource::new(vm, datasource)?;
        Ok(datasource)
    }
}

fn check_datasource(vm: &Arc<JavaVM>, datasource: &GlobalRef) -> Result<(), InitError> {
    let mut env = vm.attach_current_thread()?;
    let class = env.find_class("javax/sql/DataSource")?;
    let object = &*datasource;
    let is_ds = env.is_instance_of(object, class)?;
    if !is_ds {
        return Err(InitError::IsNotDatasource);
    }
    Ok(())
}

pub struct JvmBuilder {
    version: JNIVersion,
    classpath: String,
    xmx: u32,
    xms: u32,
    vm_options: Vec<String>,
}

impl JvmBuilder {
    pub fn version(mut self, version: JNIVersion) -> Self {
        self.version = version;
        self
    }

    pub fn xmx_mb(mut self, xmx: u32) -> Self {
        if self.xms > xmx {
            self.xms = xmx;
        }
        self.xmx = xmx;
        self
    }

    pub fn xms_mb(mut self, xms: u32) -> Self {
        if xms > self.xmx {
            self.xmx = xms;
        }
        self.xms = xms;
        self
    }

    pub fn classpath(mut self, classpath: &str) -> Self {
        self.classpath = classpath.to_owned();
        self
    }

    pub fn vm_option(mut self, option: &str) -> Self {
        self.vm_options.push(option.to_owned());
        self
    }

    pub fn new() -> Self {
        JvmBuilder {
            version: JNIVersion::V8,
            classpath: String::from("./libs/"),
            xmx: 72,
            xms: 72,
            vm_options: Vec::new(),
        }
    }

    pub fn build(self) -> Result<JavaVM, InitError> {
        let mut vm_builder = InitArgsBuilder::new()
            .version(self.version)
            .option(format!("-Xmx{}m", self.xmx))
            .option(format!("-Xms{}m", self.xms));
        let libs = libs(self.classpath.as_str());
        if libs.len() > 0 {
            let option = format!("-Djava.class.path={}", libs.join(";"));
            vm_builder = vm_builder.option(option);
        }
        for option in self.vm_options {
            vm_builder = vm_builder.option(option)
        }
        let jvm_args = vm_builder.build()?;

        let jvm = JavaVM::new(jvm_args)?;

        Ok(jvm)
    }
}

fn libs(dir: &str) -> Vec<String> {
    let mut libs = Vec::new();
    if let Ok(dir) = std::fs::read_dir(dir) {
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
            }
        }
    }
    return libs;
}
