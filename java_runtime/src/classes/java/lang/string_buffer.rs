use alloc::{
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{runtime::JavaLangString, Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{classes::java::lang::String, RuntimeClassProto, RuntimeContext};

// class java.lang.StringBuffer
pub struct StringBuffer {}

impl StringBuffer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_string, Default::default()),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
                    Self::append_string,
                    Default::default(),
                ),
                JavaMethodProto::new("append", "(I)Ljava/lang/StringBuffer;", Self::append_integer, Default::default()),
                JavaMethodProto::new("append", "(J)Ljava/lang/StringBuffer;", Self::append_long, Default::default()),
                JavaMethodProto::new("append", "(C)Ljava/lang/StringBuffer;", Self::append_character, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("value", "[C", Default::default()),
                JavaFieldProto::new("count", "I", Default::default()),
            ],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::<init>({:?})", &this);

        let array = jvm.instantiate_array("C", 16).await?;
        jvm.put_field(&mut this, "value", "[C", array).await?;
        jvm.put_field(&mut this, "count", "I", 0).await?;

        Ok(())
    }

    async fn init_with_string(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::<init>({:?}, {:?})", &this, &string,);

        let value_array = jvm.get_field(&string, "value", "[C").await?;
        let length = jvm.array_length(&value_array)? as i32;

        jvm.put_field(&mut this, "value", "[C", value_array).await?;
        jvm.put_field(&mut this, "count", "I", length).await?;

        Ok(())
    }

    async fn append_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({:?}, {:?})", &this, &string,);

        let string = JavaLangString::to_rust_string(jvm, string.into()).await?;

        Self::append(jvm, &mut this, &string).await?;

        Ok(this)
    }

    async fn append_integer(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({:?}, {:?})", &this, value);

        let digits = value.to_string();

        Self::append(jvm, &mut this, &digits).await?;

        Ok(this)
    }

    async fn append_long(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i64) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({:?}, {:?})", &this, value);

        let digits = value.to_string();

        Self::append(jvm, &mut this, &digits).await?;

        Ok(this)
    }

    async fn append_character(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: u16) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({:?}, {:?})", &this, value);

        let value = RustString::from_utf16(&[value]).unwrap();

        Self::append(jvm, &mut this, &value).await?;

        Ok(this)
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.StringBuffer::toString({:?})", &this);

        let java_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;

        let string = jvm.new_class("java/lang/String", "([CII)V", (java_value, 0, count)).await?;

        Ok(string.into())
    }

    async fn ensure_capacity(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, capacity: usize) -> Result<()> {
        let java_value_array = jvm.get_field(this, "value", "[C").await?;
        let current_capacity = jvm.array_length(&java_value_array)?;

        if current_capacity < capacity {
            let old_values: Vec<JavaChar> = jvm.load_array(&java_value_array, 0, current_capacity)?;
            let new_capacity = capacity * 2;

            let mut java_new_value_array = jvm.instantiate_array("C", new_capacity).await?;
            jvm.put_field(this, "value", "[C", java_new_value_array.clone()).await?;
            jvm.store_array(&mut java_new_value_array, 0, old_values)?;
        }

        Ok(())
    }

    async fn append(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, string: &str) -> Result<()> {
        let current_count: i32 = jvm.get_field(this, "count", "I").await?;

        let value_to_add = string.encode_utf16().collect::<Vec<_>>();
        let count_to_add = value_to_add.len() as i32;

        StringBuffer::ensure_capacity(jvm, this, (current_count + count_to_add) as _).await?;

        let mut java_value_array = jvm.get_field(this, "value", "[C").await?;
        jvm.store_array(&mut java_value_array, current_count as _, value_to_add)?;
        jvm.put_field(this, "count", "I", current_count + count_to_add).await?;

        Ok(())
    }
}
