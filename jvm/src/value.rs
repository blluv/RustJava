use alloc::{boxed::Box, rc::Rc};
use core::cell::RefCell;

use crate::{class_instance::ClassInstance, ClassInstanceRef};

pub type JavaChar = u16;

#[derive(Clone, Debug)]
pub enum JavaValue {
    Void,
    Boolean(bool),
    Byte(i8),
    Char(JavaChar),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Object(Option<Rc<RefCell<Box<dyn ClassInstance>>>>),
}

impl From<JavaValue> for bool {
    fn from(x: JavaValue) -> Self {
        match x {
            JavaValue::Boolean(x) => x,
            _ => panic!("Expected boolean, got {:?}", x),
        }
    }
}

impl From<JavaValue> for i8 {
    fn from(x: JavaValue) -> Self {
        match x {
            JavaValue::Byte(x) => x,
            _ => panic!("Expected byte, got {:?}", x),
        }
    }
}

impl From<JavaValue> for JavaChar {
    fn from(x: JavaValue) -> Self {
        match x {
            JavaValue::Char(x) => x,
            _ => panic!("Expected char, got {:?}", x),
        }
    }
}

impl From<JavaValue> for i16 {
    fn from(x: JavaValue) -> Self {
        match x {
            JavaValue::Short(x) => x,
            _ => panic!("Expected short, got {:?}", x),
        }
    }
}

impl From<JavaValue> for i32 {
    fn from(x: JavaValue) -> Self {
        match x {
            JavaValue::Int(x) => x,
            _ => panic!("Expected int, got {:?}", x),
        }
    }
}

impl From<JavaValue> for i64 {
    fn from(x: JavaValue) -> Self {
        match x {
            JavaValue::Long(x) => x,
            _ => panic!("Expected long, got {:?}", x),
        }
    }
}

impl From<JavaValue> for f32 {
    fn from(x: JavaValue) -> Self {
        match x {
            JavaValue::Float(x) => x,
            _ => panic!("Expected float, got {:?}", x),
        }
    }
}

impl From<JavaValue> for f64 {
    fn from(x: JavaValue) -> Self {
        match x {
            JavaValue::Double(x) => x,
            _ => panic!("Expected double, got {:?}", x),
        }
    }
}

impl From<JavaValue> for Option<ClassInstanceRef> {
    fn from(x: JavaValue) -> Self {
        match x {
            JavaValue::Object(x) => x,
            _ => panic!("Expected object, got {:?}", x),
        }
    }
}

impl From<JavaValue> for ClassInstanceRef {
    fn from(x: JavaValue) -> Self {
        match x {
            JavaValue::Object(x) => x.unwrap(),
            _ => panic!("Expected object, got {:?}", x),
        }
    }
}

impl From<JavaValue> for () {
    fn from(x: JavaValue) -> Self {
        match x {
            JavaValue::Void => (),
            _ => panic!("Expected void, got {:?}", x),
        }
    }
}

impl From<bool> for JavaValue {
    fn from(x: bool) -> Self {
        JavaValue::Boolean(x)
    }
}

impl From<i8> for JavaValue {
    fn from(x: i8) -> Self {
        JavaValue::Byte(x)
    }
}

impl From<JavaChar> for JavaValue {
    fn from(x: JavaChar) -> Self {
        JavaValue::Char(x)
    }
}

impl From<i16> for JavaValue {
    fn from(x: i16) -> Self {
        JavaValue::Short(x)
    }
}

impl From<i32> for JavaValue {
    fn from(x: i32) -> Self {
        JavaValue::Int(x)
    }
}

impl From<i64> for JavaValue {
    fn from(x: i64) -> Self {
        JavaValue::Long(x)
    }
}

impl From<f32> for JavaValue {
    fn from(x: f32) -> Self {
        JavaValue::Float(x)
    }
}

impl From<f64> for JavaValue {
    fn from(x: f64) -> Self {
        JavaValue::Double(x)
    }
}

impl From<ClassInstanceRef> for JavaValue {
    fn from(x: ClassInstanceRef) -> Self {
        JavaValue::Object(Some(x))
    }
}

impl From<Option<ClassInstanceRef>> for JavaValue {
    fn from(x: Option<ClassInstanceRef>) -> Self {
        JavaValue::Object(x)
    }
}
