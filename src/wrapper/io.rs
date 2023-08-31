use std::{
    io::{Read, Write},
    sync::Arc,
};

use jni::{
    objects::{GlobalRef, JByteArray, JObject, JValueGen, ReleaseMode},
    JavaVM,
};
use log::error;

use crate::{errors, util};

#[derive(Debug)]
pub struct OutputStream {
    inner: GlobalRef,
    vm: Arc<JavaVM>,
}

impl AsRef<JObject<'static>> for OutputStream {
    fn as_ref(&self) -> &JObject<'static> {
        &self.inner
    }
}
impl OutputStream {
    pub fn new(inner: GlobalRef, vm: Arc<JavaVM>) -> Self {
        Self { inner, vm }
    }
    #[inline]
    fn write_jni(&self, buf: &[u8]) -> Result<(), errors::Error> {
        let mut env = self.vm.attach_current_thread()?;
        let array = util::cast::vec_to_bytes_array(&mut env, buf)?;
        env.call_method(&self.inner, "write", "([B)V", &[JValueGen::Object(&array)])?;
        env.delete_local_ref(array).unwrap();
        Ok(())
    }
    #[inline]
    fn flush_jni(&self) -> Result<(), errors::Error> {
        let mut env = self.vm.attach_current_thread()?;
        env.call_method(&self.inner, "flush", "()V", &[])?;
        Ok(())
    }
    #[inline]
    pub fn close(&mut self) -> Result<(), errors::Error> {
        let mut env = self.vm.attach_current_thread()?;

        env.call_method(&self.inner, "close", "()V", &[])?;
        Ok(())
    }
}

impl Write for OutputStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Err(e) = self.write_jni(buf) {
            error!("{:#?}", e);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if let Err(e) = self.flush_jni() {
            error!("{:#?}", e);
        }
        Ok(())
    }
}

impl Drop for OutputStream {
    fn drop(&mut self) {
        if let Err(e) = self.close() {
            error!("{:#?}", e);
        }
    }
}

#[derive(Debug)]
pub struct InputStream {
    inner: GlobalRef,
    vm: Arc<JavaVM>,
}

impl AsRef<JObject<'static>> for InputStream {
    fn as_ref(&self) -> &JObject<'static> {
        &self.inner
    }
}

impl InputStream {
    pub fn new(inner: GlobalRef, vm: Arc<JavaVM>) -> Self {
        Self { inner, vm }
    }

    fn read_jni(&self, buf: &mut [u8]) -> Result<usize, errors::Error> {
        let mut env = self.vm.attach_current_thread()?;
        // define a byte array
        let array: JByteArray = env.new_byte_array(buf.len() as i32)?;
        // read bytes
        let len = env.call_method(&self.inner, "read", "([B)I", &[JValueGen::Object(&array)])?;
        let mut len = util::cast::value_cast_i32(len)?;
        if len == -1 {
            len = 0;
        }
        unsafe {
            let array = env.get_array_elements(&array, ReleaseMode::NoCopyBack)?;
            let mut index = 0;
            for byte in array.iter() {
                buf[index] = *byte as u8;
                index += 1;
            }
        }
        Ok(len as usize)
    }
    #[inline]
    fn close(&self) -> Result<(), errors::Error> {
        let mut env = self.vm.attach_current_thread()?;
        env.call_method(&self.inner, "close", "()V", &[])?;
        Ok(())
    }
}

impl Read for InputStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.read_jni(buf);
        match len {
            Ok(u) => Ok(u),
            Err(err) => {
                error!("{:#?}", err);
                Ok(0)
            }
        }
    }
}

impl Drop for InputStream {
    fn drop(&mut self) {
        if let Err(e) = self.close() {
            error!("{:#?}", e);
        }
    }
}
