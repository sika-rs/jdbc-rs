use jni::{
    objects::{AutoLocal, JMethodID, JObject, JValueGen},
    signature::ReturnType,
    sys::jvalue,
};

use crate::{errors::Error, util, Connection};

pub struct ResultSetMetaData<'local> {
    inner: AutoLocal<'local, JObject<'local>>,
    get_column_count: JMethodID,
    get_column_name: JMethodID,
    conn: &'local Connection,
}

impl<'local> ResultSetMetaData<'local> {
    pub fn from_ref(conn: &'local Connection, statement: JObject<'local>) -> Result<Self, Error> {
        let mut env = conn.env()?;

        let statement = AutoLocal::new(statement, &env);

        let class = AutoLocal::new(env.find_class("java/sql/ResultSetMetaData")?, &env);
        let get_column_count = env.get_method_id(&class, "getColumnCount", "()I")?;

        let get_column_name =
            env.get_method_id(&class, "getColumnName", "(I)Ljava/lang/String;")?;

        Ok(ResultSetMetaData {
            inner: statement,
            get_column_count,
            get_column_name,
            conn,
        })
    }

    pub fn get_column_count(&self) -> Result<i32, Error> {
        let mut env = self.conn.env()?;
        return util::call::get_int(&mut env, &self.inner, &self.get_column_count);
    }

    pub fn get_column_name(&self, column: i32) -> Result<String, Error> {
        let mut env = self.conn.env()?;
        let name = unsafe {
            env.call_method_unchecked(
                &self.inner,
                self.get_column_name,
                ReturnType::Object,
                &[jvalue { i: column }],
            )?
        };
        if let JValueGen::Object(name) = name {
            return util::cast::obj_cast_string(&mut env, name).map_err(Error::from);
        }
        return Err(Error::ImpossibleError);
    }

    pub fn get_columns_name(&self) -> Result<Vec<String>, Error> {
        let mut columns = Vec::new();
        let count = self.get_column_count()?;
        for i in 1..count + 1 {
            columns.push(self.get_column_name(i)?);
        }
        Ok(columns)
    }
}
