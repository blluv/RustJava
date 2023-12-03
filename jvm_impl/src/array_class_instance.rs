use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use jvm::{ArrayClass, ArrayClassInstance, Class, ClassInstance, JavaType, JavaValue, JvmResult};

use crate::array_class::ArrayClassImpl;

pub struct ArrayClassInstanceImpl {
    class_name: String,
    length: usize,
    elements: Vec<JavaValue>,
}

impl ArrayClassInstanceImpl {
    pub fn new(class: &ArrayClassImpl, length: usize) -> Self {
        let element_type = class.element_type_name();
        let default_value = JavaType::parse(element_type).default();

        Self {
            class_name: class.name().to_string(),
            length,
            elements: vec![default_value; length],
        }
    }
}

impl ClassInstance for ArrayClassInstanceImpl {
    fn class_name(&self) -> &str {
        &self.class_name
    }

    fn as_array_instance_mut(&mut self) -> Option<&mut dyn ArrayClassInstance> {
        Some(self)
    }
}

impl ArrayClassInstance for ArrayClassInstanceImpl {
    fn store(&mut self, offset: usize, values: &[JavaValue]) -> JvmResult<()> {
        anyhow::ensure!(offset + values.len() <= self.length, "Array index out of bounds");

        self.elements[offset..offset + values.len()].clone_from_slice(values);

        Ok(())
    }
}
