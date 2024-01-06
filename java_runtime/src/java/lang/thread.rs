use alloc::{boxed::Box, format, string::String, vec};
use core::time::Duration;

use java_runtime_base::{
    JavaClassProto, JavaError, JavaFieldAccessFlag, JavaFieldProto, JavaMethodFlag, JavaMethodProto, JavaResult, JvmClassInstanceHandle,
};
use jvm::{JavaValue, Jvm, JvmCallback};

use crate::java::lang::Runnable;

// class java.lang.Thread
pub struct Thread {}

impl Thread {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/Runnable;)V", Self::init, JavaMethodFlag::NONE),
                JavaMethodProto::new("start", "()V", Self::start, JavaMethodFlag::NONE),
                JavaMethodProto::new("sleep", "(J)V", Self::sleep, JavaMethodFlag::NATIVE),
                JavaMethodProto::new("yield", "()V", Self::r#yield, JavaMethodFlag::NATIVE),
                JavaMethodProto::new("setPriority", "(I)V", Self::set_priority, JavaMethodFlag::NONE),
            ],
            fields: vec![JavaFieldProto::new("target", "Ljava/lang/Runnable;", JavaFieldAccessFlag::NONE)],
        }
    }

    async fn init(jvm: &mut Jvm, mut this: JvmClassInstanceHandle<Self>, target: JvmClassInstanceHandle<Runnable>) -> JavaResult<()> {
        tracing::debug!("Thread::<init>({:?}, {:?})", &this, &target);

        jvm.put_field(&mut this, "target", "Ljava/lang/Runnable;", target)?;

        Ok(())
    }

    async fn start(jvm: &mut Jvm, this: JvmClassInstanceHandle<Self>) -> JavaResult<()> {
        tracing::debug!("Thread::start({:?})", &this);

        struct ThreadStartProxy {
            thread_id: String,
            runnable: JvmClassInstanceHandle<Runnable>,
        }

        #[async_trait::async_trait(?Send)]
        impl JvmCallback for ThreadStartProxy {
            #[tracing::instrument(name = "thread", fields(thread = self.thread_id), skip_all)]
            async fn call(&self, jvm: &mut Jvm, _: Box<[JavaValue]>) -> Result<JavaValue, JavaError> {
                tracing::trace!("Thread start");

                jvm.invoke_virtual(&self.runnable, "java/lang/Runnable", "run", "()V", []).await?;

                Ok(JavaValue::Void)
            }
        }

        let runnable = jvm.get_field(&this, "target", "Ljava/lang/Runnable;")?;

        jvm.platform().spawn(Box::new(ThreadStartProxy {
            thread_id: format!("{:?}", &runnable),
            runnable,
        }));

        Ok(())
    }

    async fn sleep(jvm: &mut Jvm, duration: i64) -> JavaResult<i32> {
        tracing::debug!("Thread::sleep({:?})", duration);

        jvm.platform().sleep(Duration::from_millis(duration as _)).await;

        Ok(0)
    }

    async fn r#yield(jvm: &mut Jvm) -> JavaResult<i32> {
        tracing::debug!("Thread::yield()");
        jvm.platform().r#yield().await;

        Ok(0)
    }

    async fn set_priority(_: &mut Jvm, this: JvmClassInstanceHandle<Thread>, new_priority: i32) -> JavaResult<()> {
        tracing::warn!("stub java.lang.Thread::setPriority({:?}, {:?})", &this, new_priority);

        Ok(())
    }
}
