use alloc::vec;

use java_runtime_base::{JavaClassProto, JavaMethodFlag, JavaMethodProto, JavaResult, JvmClassInstanceHandle};
use jvm::Jvm;

// class java.util.GregorianCalendar
pub struct GregorianCalendar {}

impl GregorianCalendar {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: Some("java/util/Calendar"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "()V", Self::init, JavaMethodFlag::NONE)],
            fields: vec![],
        }
    }

    async fn init(_: &mut Jvm, this: JvmClassInstanceHandle<Self>) -> JavaResult<()> {
        tracing::warn!("stub java.util.GregorianCalendar::<init>({:?})", &this);

        Ok(())
    }
}
