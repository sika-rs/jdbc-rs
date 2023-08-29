use super::ResultSet;
use crate::{errors::Error, util, Connection};
use async_trait::async_trait;
use jni::{
    objects::{AutoLocal, GlobalRef, JMethodID, JObject, JValueGen},
    signature::{Primitive, ReturnType},
    sys::jvalue,
    JavaVM,
};
use std::sync::Arc;

#[cfg(not(feature = "async"))]
pub trait IStatement {
    // fn execute(&mut self) -> Result<bool, Error>;
    fn execute_query(&self, sql: &str) -> Result<ResultSet, Error>;
    fn execute_update(&mut self, sql: &str) -> Result<i32, Error>;
}

#[cfg(feature = "async")]
#[async_trait]
pub trait IStatement {
    // async fn execute(&mut self) -> Result<bool, Error>;
    async fn execute_query(&self, sql: &str) -> Result<ResultSet, Error>;
    async fn execute_update(&mut self, sql: &str) -> Result<i32, Error>;
}

pub struct Statement<'local> {
    inner: GlobalRef,
    conn: &'local Connection,
    execute_query_sql: JMethodID,
    execute_update_sql: JMethodID,
}

impl<'local> Statement<'local> {
    pub fn from_ref(conn: &'local Connection, statement: GlobalRef) -> Result<Self, Error> {
        let mut env = conn.env()?;
        let class = AutoLocal::new(env.find_class("java/sql/Statement")?, &env);
        let execute_query_sql = env.get_method_id(
            &class,
            "executeQuery",
            "(Ljava/lang/String;)Ljava/sql/ResultSet;",
        )?;
        let execute_update_sql =
            env.get_method_id(&class, "executeUpdate", "(Ljava/lang/String;)I")?;

        Ok(Self {
            inner: statement,
            conn,
            execute_query_sql,
            execute_update_sql,
        })
    }

    fn execute_query_sql(
        vm: &Arc<JavaVM>,
        inner: &GlobalRef,
        method: &JMethodID,
        sql: &str,
    ) -> Result<GlobalRef, Error> {
        let mut env = vm.attach_current_thread()?;
        let sql: JObject = env.new_string(sql)?.into();
        let result = unsafe {
            env.call_method_unchecked(
                inner,
                method,
                ReturnType::Object,
                &[JValueGen::Object(&sql).as_jni()],
            )?
        };
        env.delete_local_ref(sql)?;
        if let JValueGen::Object(statement) = result {
            let global_ref = env.new_global_ref(statement)?;
            return Ok(global_ref);
        }
        return Err(Error::ImpossibleError);
    }

    fn execute_update_sql(
        vm: &Arc<JavaVM>,
        inner: &GlobalRef,
        method: &JMethodID,
        sql: &str,
    ) -> Result<i32, Error> {
        let mut env = vm.attach_current_thread()?;
        let sql: JObject = env.new_string(sql)?.into();
        let result = unsafe {
            env.call_method_unchecked(
                inner,
                method,
                ReturnType::Primitive(Primitive::Int),
                &[JValueGen::Object(&sql).as_jni()],
            )?
        };
        env.delete_local_ref(sql)?;
        if let JValueGen::Int(result) = result {
            return Ok(result);
        }
        return Err(Error::ImpossibleError);
    }
}

#[cfg(feature = "async")]
#[async_trait]
impl<'local> IStatement for Statement<'local> {
    async fn execute_query(&self, sql: &str) -> Result<ResultSet, Error> {
        let vm = self.conn.vm().clone();
        let inner = self.inner.clone();
        let method = self.execute_query_sql.clone();
        let sql = sql.to_string();
        let result_ref = crate::block_on!(move || {
            Self::execute_query_sql(&vm, &inner, &method, sql.as_str())
        });
        return Ok(ResultSet::from_ref(self.conn, result_ref)?);
    }
    async fn execute_update(&mut self, sql: &str) -> Result<i32, Error> {
        let vm = self.conn.vm().clone();
        let inner = self.inner.clone();
        let method = self.execute_update_sql.clone();
        let sql = sql.to_string();
        let count = crate::block_on!(move || {
            Self::execute_update_sql(&vm, &inner, &method, sql.as_str())
        });
        return Ok(count);
    }
}

#[cfg(not(feature = "async"))]
#[async_trait]
impl<'local> IStatement for Statement<'local> {
    fn execute_query(&self, sql: &str) -> Result<ResultSet, Error> {
        let result_ref =
            Self::execute_query_sql(self.conn.vm(), &self.inner, &self.execute_query_sql, sql)?;
        ResultSet::from_ref(self.conn, result_ref)
    }
    fn execute_update(&mut self, sql: &str) -> Result<i32, Error> {
        Self::execute_update_sql(self.conn.vm(), &self.inner, &self.execute_update_sql, sql)
    }
}

pub struct PreparedStatement<'local> {
    inner: GlobalRef,
    statement: Statement<'local>,
    set_string: JMethodID,
    set_short: JMethodID,
    set_int: JMethodID,
    set_long: JMethodID,
    set_float: JMethodID,
    set_double: JMethodID,
    set_bool: JMethodID,
    set_byte: JMethodID,
    set_bytes: JMethodID,
    set_big_decimal: JMethodID,
    execute_query: JMethodID,
    execute_update: JMethodID,
    conn: &'local Connection,
}

impl<'local> PreparedStatement<'local> {
    pub fn from_ref(conn: &'local Connection, inner: GlobalRef) -> Result<Self, Error> {
        let mut env = conn.env()?;

        let class = AutoLocal::new(env.find_class("java/sql/PreparedStatement")?, &env);

        let set_string = env.get_method_id(&class, "setString", "(ILjava/lang/String;)V")?;
        let set_short = env.get_method_id(&class, "setShort", "(IS)V")?;
        let set_int = env.get_method_id(&class, "setInt", "(II)V")?;
        let set_long = env.get_method_id(&class, "setLong", "(IJ)V")?;
        let set_float = env.get_method_id(&class, "setFloat", "(IF)V")?;
        let set_double = env.get_method_id(&class, "setDouble", "(ID)V")?;
        let set_bool = env.get_method_id(&class, "setBoolean", "(IZ)V")?;
        let set_byte = env.get_method_id(&class, "setByte", "(IB)V")?;
        let set_bytes = env.get_method_id(&class, "setBytes", "(I[B)V")?;

        let set_big_decimal =
            env.get_method_id(&class, "setBigDecimal", "(ILjava/math/BigDecimal;)V")?;

        let execute_query = env.get_method_id(&class, "executeQuery", "()Ljava/sql/ResultSet;")?;
        let execute_update = env.get_method_id(&class, "executeUpdate", "()I")?;

        let statement = Statement::from_ref(conn, inner.clone())?;

        Ok(PreparedStatement {
            inner,
            statement,
            set_string,
            set_short,
            set_int,
            set_long,
            set_float,
            set_double,
            set_bool,
            set_byte,
            set_bytes,
            set_big_decimal,
            execute_query,
            execute_update,
            conn,
        })
    }

    #[cfg(not(feature = "async"))]
    pub fn execute_query(&self) -> Result<ResultSet, Error> {
        let result_ref =
            Self::execute_query_inner(self.conn.vm(), &self.inner, &self.execute_query)?;
        ResultSet::from_ref(self.conn, result_ref)
    }
    #[cfg(not(feature = "async"))]
    pub fn execute_update(&mut self) -> Result<i32, Error> {
        Self::execute_update_inner(self.conn.vm(), &self.inner, &self.execute_update)
    }

    #[cfg(feature = "async")]
    pub async fn execute_query(&self) -> Result<ResultSet, Error> {
        let vm = self.conn.vm().clone();
        let inner = self.inner.clone();
        let method = self.execute_query.clone();
        let result_ref = crate::block_on!(move || Self::execute_query_inner(&vm, &inner, &method));
        return Ok(ResultSet::from_ref(self.conn, result_ref)?);
    }
    #[cfg(feature = "async")]
    pub async fn execute_update(&mut self) -> Result<i32, Error> {
        let vm = self.conn.vm().clone();
        let inner = self.inner.clone();
        let method = self.execute_update.clone();
        let count = crate::block_on!(move || Self::execute_update_inner(&vm, &inner, &method));
        return Ok(count);
    }

    fn execute_query_inner(
        vm: &Arc<JavaVM>,
        inner: &GlobalRef,
        method: &JMethodID,
    ) -> Result<GlobalRef, Error> {
        let mut env = vm.attach_current_thread()?;
        let result = unsafe { env.call_method_unchecked(inner, method, ReturnType::Object, &[])? };
        if let JValueGen::Object(statement) = result {
            let global_ref = env.new_global_ref(statement)?;
            return Ok(global_ref);
        }
        return Err(Error::ImpossibleError);
    }

    fn execute_update_inner(
        vm: &Arc<JavaVM>,
        inner: &GlobalRef,
        method: &JMethodID,
    ) -> Result<i32, Error> {
        let mut env = vm.attach_current_thread()?;
        let result = unsafe {
            env.call_method_unchecked(inner, method, ReturnType::Primitive(Primitive::Int), &[])?
        };
        if let JValueGen::Int(result) = result {
            return Ok(result);
        }
        return Err(Error::ImpossibleError);
    }

    pub fn set_string(mut self, index: i32, value: &str) -> Result<Self, Error> {
        let env = self.conn.env()?;
        // new String(value)
        let value: JObject<'local> = env.new_string(value)?.into();
        self.set_param(self.set_string, index, JValueGen::Object(&value).as_jni())?;
        // del String
        env.delete_local_ref(value)?;
        Ok(self)
    }
    pub fn set_short(mut self, index: i32, value: i16) -> Result<Self, Error> {
        self.set_param(self.set_short, index, jvalue { s: value })?;
        Ok(self)
    }
    pub fn set_int(mut self, index: i32, value: i32) -> Result<Self, Error> {
        self.set_param(self.set_int, index, jvalue { i: value })?;
        Ok(self)
    }
    pub fn set_long(mut self, index: i32, value: i64) -> Result<Self, Error> {
        self.set_param(self.set_long, index, jvalue { j: value })?;
        Ok(self)
    }
    pub fn set_float(mut self, index: i32, value: f32) -> Result<Self, Error> {
        self.set_param(self.set_float, index, jvalue { f: value })?;
        Ok(self)
    }
    pub fn set_double(mut self, index: i32, value: f64) -> Result<Self, Error> {
        self.set_param(self.set_double, index, jvalue { d: value })?;
        Ok(self)
    }
    pub fn set_boolean(mut self, index: i32, value: bool) -> Result<Self, Error> {
        self.set_param(self.set_bool, index, util::cast::bool_to_jvalue(value))?;
        Ok(self)
    }

    pub fn set_byte(mut self, index: i32, value: u8) -> Result<Self, Error> {
        self.set_param(self.set_byte, index, util::cast::u8_to_jvalue(value))?;
        Ok(self)
    }

    pub fn set_bytes(mut self, index: i32, value: &[u8]) -> Result<Self, Error> {
        let mut env = self.conn.env()?;
        let array = util::cast::vec_to_bytes_array(&mut env, value)?;
        self.set_param(self.set_bytes, index, JValueGen::Object(&array).as_jni())?;
        env.delete_local_ref(array)?;
        Ok(self)
    }

    pub fn set_big_decimal(
        mut self,
        index: i32,
        value: &bigdecimal::BigDecimal,
    ) -> Result<Self, Error> {
        let mut env = self.conn.env()?;
        let num = value.to_string();
        let java_string = env.new_string(num)?;
        let java_num = env.new_object(
            "java/math/BigDecimal",
            "(Ljava/lang/String;)V",
            &[JValueGen::Object(&java_string)],
        )?;
        self.set_param(
            self.set_big_decimal,
            index,
            JValueGen::Object(&java_num).as_jni(),
        )?;
        env.delete_local_ref(java_string)?;
        env.delete_local_ref(java_num)?;
        Ok(self)
    }

    #[inline(always)]
    fn set_param(&mut self, method: JMethodID, index: i32, value: jvalue) -> Result<(), Error> {
        let mut env = self.conn.env()?;
        unsafe {
            env.call_method_unchecked(
                &self.inner,
                method,
                ReturnType::Primitive(Primitive::Void),
                &[jvalue { i: index }, value],
            )?;
        }
        Ok(())
    }
}

#[cfg(feature = "async")]
#[async_trait]
impl<'local> IStatement for PreparedStatement<'local> {
    async fn execute_query(&self, sql: &str) -> Result<ResultSet, Error> {
        self.statement.execute_query(sql).await
    }
    async fn execute_update(&mut self, sql: &str) -> Result<i32, Error> {
        self.statement.execute_update(sql).await
    }
}

#[cfg(not(feature = "async"))]
impl<'local> IStatement for PreparedStatement<'local> {
    fn execute_query(&self, sql: &str) -> Result<ResultSet, Error> {
        self.statement.execute_query(sql)
    }
    fn execute_update(&mut self, sql: &str) -> Result<i32, Error> {
        self.statement.execute_update(sql)
    }
}

impl<'local> Drop for PreparedStatement<'local> {
    fn drop(&mut self) {
        let env = self.conn.env();
        if let Ok(mut env) = env {
            let _ = util::auto_close(&mut env, &self.inner);
        }
    }
}
