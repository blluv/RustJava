use alloc::{boxed::Box, format, vec};

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{runtime::JavaLangString, ClassInstance, ClassInstanceRef, Jvm, Result};

use crate::{classes::java::lang::String, RuntimeClassProto, RuntimeContext};

// class java.lang.Object
pub struct Object;

impl Object {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Object",
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getClass", "()Ljava/lang/Class;", Self::get_class, Default::default()),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, Default::default()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
                JavaMethodProto::new("clone", "()Ljava/lang/Object;", Self::java_clone, MethodAccessFlags::NATIVE),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new("notify", "()V", Self::notify, MethodAccessFlags::NATIVE),
                JavaMethodProto::new("notifyAll", "()V", Self::notify_all, MethodAccessFlags::NATIVE),
                JavaMethodProto::new("wait", "(J)V", Self::wait_long, Default::default()),
                JavaMethodProto::new("wait", "(JI)V", Self::wait_long_int, Default::default()),
                JavaMethodProto::new("wait", "()V", Self::wait, Default::default()),
                JavaMethodProto::new("finalize", "()V", Self::finalize, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Object::<init>({:?})", &this);

        Ok(())
    }

    async fn get_class(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.Object::getClass({:?})", &this);

        // TODO can we get class directly?
        let this: Box<dyn ClassInstance> = this.into();
        let class_name = this.class_definition().name();

        let class = jvm.resolve_class(&class_name).await?.java_class(jvm).await?;

        Ok(class.into())
    }

    async fn hash_code(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.Object::hashCode({:?})", &this);

        let rust_this: Box<dyn ClassInstance> = this.into();

        Ok(rust_this.hash_code())
    }

    async fn equals(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.Object::equals({:?}, {:?})", &this, &other);

        if other.is_null() {
            return Ok(false);
        }

        let rust_this: Box<dyn ClassInstance> = this.into();
        let rust_other: Box<dyn ClassInstance> = other.into();

        rust_this.equals(&*rust_other)
    }

    async fn java_clone(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::warn!("stub java.lang.Object::clone({:?})", &this);

        Ok(None.into())
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Object::toString({:?})", &this);

        let class = jvm.invoke_virtual(&this, "getClass", "()Ljava/lang/Class;", ()).await?;
        let class_name = jvm.invoke_virtual(&class, "getName", "()Ljava/lang/String;", ()).await?;
        let class_name_rust = JavaLangString::to_rust_string(jvm, &class_name).await?;

        let hash_code: i32 = jvm.invoke_virtual(&this, "hashCode", "()I", ()).await?;

        let result = format!("{}@{:x}", class_name_rust, hash_code);

        Ok(JavaLangString::from_rust_string(jvm, &result).await?.into())
    }

    async fn notify(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::warn!("stub java.lang.Object::notify({:?})", &this);

        Ok(())
    }

    async fn notify_all(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::warn!("stub java.lang.Object::notifyAll({:?})", &this);

        Ok(())
    }

    async fn wait_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, millis: i64) -> Result<()> {
        tracing::debug!("java.lang.Object::wait({:?}, {:?})", &this, millis);

        let _: () = jvm.invoke_virtual(&this, "wait", "(JI)V", (millis, 0)).await?;

        Ok(())
    }

    async fn wait_long_int(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, millis: i64, nanos: i32) -> Result<()> {
        tracing::warn!("stub java.lang.Object::wait({:?}, {:?}, {:?})", &this, millis, nanos);

        Ok(())
    }

    async fn wait(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Object::wait({:?})", &this);

        let _: () = jvm.invoke_virtual(&this, "wait", "(JI)V", (0, 0)).await?;

        Ok(())
    }

    async fn finalize(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::warn!("stub java.lang.Object::finalize({:?})", &this);

        Ok(())
    }
}
