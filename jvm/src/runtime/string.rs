use alloc::{boxed::Box, rc::Rc, string::String, vec::Vec};
use core::cell::RefCell;

use crate::{ClassInstance, ClassInstanceRef, JavaChar, JavaValue, Jvm, JvmResult};

pub struct JavaLangString {
    pub instance: Rc<RefCell<Box<dyn ClassInstance>>>,
}

impl JavaLangString {
    pub async fn new(jvm: &mut Jvm, string: &str) -> JvmResult<Self> {
        let chars = string.chars().map(|x| JavaValue::Char(x as _)).collect::<Vec<_>>();

        let array = jvm.instantiate_array("C", chars.len()).await?;
        jvm.store_array(&array, 0, chars)?;

        let instance = jvm.instantiate_class("java/lang/String").await?;
        jvm.invoke_virtual(&instance, "java/lang/String", "<init>", "([C)V", [JavaValue::Object(Some(array))])
            .await?;

        Ok(Self { instance })
    }

    pub fn from_instance(instance: Rc<RefCell<Box<dyn ClassInstance>>>) -> Self {
        // TODO validity
        Self { instance }
    }

    pub fn to_string(&self, jvm: &mut Jvm) -> JvmResult<String> {
        let array: ClassInstanceRef = jvm.get_field(&self.instance, "value", "[C")?;

        let chars: Vec<JavaChar> = jvm.load_array(&array, 0, jvm.array_length(&array)?)?;

        let string = chars.into_iter().map(|x| x as u8 as char).collect::<String>(); // TODO proper encoding conversion

        Ok(string)
    }
}
