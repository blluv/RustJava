use alloc::vec;

use crate::JavaClassProto;

// interface java.lang.Runnable
pub struct Runnable {}

impl Runnable {
    // TODO Create JavaInterfaceProto
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: None,
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
