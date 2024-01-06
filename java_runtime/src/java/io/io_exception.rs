use alloc::vec;

use java_runtime_base::JavaClassProto;

// class java.io.IOException
pub struct IOException {}

impl IOException {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: Some("java/lang/Exception"),
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
