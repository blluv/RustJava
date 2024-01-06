use alloc::vec;

use java_runtime_base::{JavaMethodFlag, JavaMethodProto, JavaResult, JvmClassInstanceHandle};
use jvm::Jvm;

use crate::{java::lang::String, JavaClassProto, JavaContext};

// class java.io.PrintStream
pub struct PrintStream {}

impl PrintStream {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: Some("java/io/OutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, JavaMethodFlag::NONE),
                JavaMethodProto::new("println", "(Ljava/lang/String;)V", Self::println, JavaMethodFlag::NONE),
            ],
            fields: vec![],
        }
    }

    async fn init(_: &mut Jvm, _: &JavaContext, this: JvmClassInstanceHandle<Self>) -> JavaResult<()> {
        tracing::warn!("stub java.lang.PrintStream::<init>({:?})", &this);

        Ok(())
    }

    async fn println(
        jvm: &mut Jvm,
        context: &JavaContext,
        this: JvmClassInstanceHandle<Self>,
        str: JvmClassInstanceHandle<String>,
    ) -> JavaResult<()> {
        tracing::warn!("stub java.lang.PrintStream::println({:?}, {:?})", &this, &str);

        let rust_str = String::to_rust_string(jvm, &str)?;
        context.println(&rust_str);

        Ok(())
    }
}
