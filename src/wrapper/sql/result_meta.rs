use jni::{
    objects::{AutoLocal, JMethodID, JObject, JValueGen},
    signature::ReturnType,
    sys::jvalue,
    JNIEnv,
};

use crate::{errors::Error, util};

pub struct ResultSetMetaData<'local> {
    inner: AutoLocal<'local, JObject<'local>>,
    get_column_count: JMethodID,
    get_column_name: JMethodID,
    env: JNIEnv<'local>,
}

impl<'local> ResultSetMetaData<'local> {
    pub fn from_ref(env: &mut JNIEnv<'local>, statement: JObject<'local>) -> Result<Self, Error> {
        let statement = AutoLocal::new(statement, env);

        let class = AutoLocal::new(env.find_class("java/sql/ResultSetMetaData")?, &env);
        let get_column_count = env.get_method_id(&class, "getColumnCount", "()I")?;

        let get_column_name =
            env.get_method_id(&class, "getColumnName", "(I)Ljava/lang/String;")?;

        let env = unsafe { env.unsafe_clone() };
        Ok(ResultSetMetaData {
            inner: statement,
            get_column_count,
            get_column_name,
            env,
        })
    }

    pub fn get_column_count(&mut self) -> Result<i32, Error> {
        return util::call::get_int(&mut self.env, &self.inner, &self.get_column_count);
    }

    pub fn get_column_name(&mut self, column: i32) -> Result<String, Error> {
        let name = unsafe {
            self.env.call_method_unchecked(
                &self.inner,
                self.get_column_name,
                ReturnType::Object,
                &[jvalue { i: column }],
            )?
        };
        if let JValueGen::Object(name) = name {
            return util::cast::obj_cast_string(&mut self.env, name);
        }
        return Err(Error::ImpossibleError);
    }

    pub fn get_columns_name(&mut self) -> Result<Vec<String>, Error> {
        let mut columns = Vec::new();
        let count = self.get_column_count()?;
        for i in 1..count + 1 {
            columns.push(self.get_column_name(i)?);
        }
        Ok(columns)
    }
}
