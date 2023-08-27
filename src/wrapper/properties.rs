use jni::{
    errors::Error,
    objects::{AutoLocal, JMethodID, JObject, JValueGen},
    signature::ReturnType,
    JNIEnv,
};

use crate::util;
pub struct Properties<'a> {
    inner: JObject<'a>,
    env: JNIEnv<'a>,
    set_prop: JMethodID,
}

impl<'a> Properties<'a> {
    pub fn new(env: &mut JNIEnv<'a>) -> Result<Self, Error> {
        let class = AutoLocal::new(env.find_class("java/util/Properties")?, env);
        let set_prop = env.get_method_id(
            &class,
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
        )?;

        let init = env.get_method_id(&class, "<init>", "()V")?;

        let properties = unsafe { env.new_object_unchecked(class, init, &[])? };

        let env = unsafe { env.unsafe_clone() };
        Ok(Properties {
            inner: properties,
            set_prop,
            env,
        })
    }

    pub fn set_property(&mut self, key: &str, value: &str) -> Result<(), Error> {
        let key: JObject = self.env.new_string(key)?.into();
        let value: JObject = self.env.new_string(value)?.into();
        let return_val: JValueGen<JObject<'_>> = unsafe {
            self.env.call_method_unchecked(
                &self.inner,
                self.set_prop,
                ReturnType::Object,
                &[
                    JValueGen::Object(&key).as_jni(),
                    JValueGen::Object(&value).as_jni(),
                ],
            )?
        };
        self.env.delete_local_ref(key)?;
        self.env.delete_local_ref(value)?;
        util::delete_value(&mut self.env, return_val)?;
        Ok(())
    }
}
impl<'a> Into<JObject<'a>> for Properties<'a> {
    fn into(self) -> JObject<'a> {
        self.inner
    }
}
