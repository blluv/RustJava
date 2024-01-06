use alloc::{
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};

use java_runtime_base::{
    Array, JavaClassProto, JavaContext, JavaFieldAccessFlag, JavaFieldProto, JavaMethodFlag, JavaMethodProto, JavaResult, JvmClassInstanceHandle,
};
use jvm::JavaChar;

use crate::java::lang::String;

// class java.lang.StringBuffer
pub struct StringBuffer {}

impl StringBuffer {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, JavaMethodFlag::NONE),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_string, JavaMethodFlag::NONE),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
                    Self::append_string,
                    JavaMethodFlag::NONE,
                ),
                JavaMethodProto::new("append", "(I)Ljava/lang/StringBuffer;", Self::append_integer, JavaMethodFlag::NONE),
                JavaMethodProto::new("append", "(J)Ljava/lang/StringBuffer;", Self::append_long, JavaMethodFlag::NONE),
                JavaMethodProto::new("append", "(C)Ljava/lang/StringBuffer;", Self::append_character, JavaMethodFlag::NONE),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, JavaMethodFlag::NONE),
            ],
            fields: vec![
                JavaFieldProto::new("value", "[C", JavaFieldAccessFlag::NONE),
                JavaFieldProto::new("count", "I", JavaFieldAccessFlag::NONE),
            ],
        }
    }

    async fn init(context: &mut dyn JavaContext, mut this: JvmClassInstanceHandle<Self>) -> JavaResult<()> {
        tracing::debug!("java.lang.StringBuffer::<init>({:?})", &this);

        let array = context.jvm().instantiate_array("C", 16).await?;
        context.jvm().put_field(&mut this, "value", "[C", array)?;
        context.jvm().put_field(&mut this, "count", "I", 0)?;

        Ok(())
    }

    async fn init_with_string(
        context: &mut dyn JavaContext,
        mut this: JvmClassInstanceHandle<Self>,
        string: JvmClassInstanceHandle<String>,
    ) -> JavaResult<()> {
        tracing::debug!("java.lang.StringBuffer::<init>({:?}, {:?})", &this, &string,);

        let value_array = context.jvm().get_field(&string, "value", "[C")?;
        let length = context.jvm().array_length(&value_array)? as i32;

        context.jvm().put_field(&mut this, "value", "[C", value_array)?;
        context.jvm().put_field(&mut this, "count", "I", length)?;

        Ok(())
    }

    async fn append_string(
        context: &mut dyn JavaContext,
        mut this: JvmClassInstanceHandle<Self>,
        string: JvmClassInstanceHandle<String>,
    ) -> JavaResult<JvmClassInstanceHandle<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({:?}, {:?})", &this, &string,);

        let string = String::to_rust_string(context, &string)?;

        Self::append(context, &mut this, &string).await?;

        Ok(this)
    }

    async fn append_integer(
        context: &mut dyn JavaContext,
        mut this: JvmClassInstanceHandle<Self>,
        value: i32,
    ) -> JavaResult<JvmClassInstanceHandle<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({:?}, {:?})", &this, value);

        let digits = value.to_string();

        Self::append(context, &mut this, &digits).await?;

        Ok(this)
    }

    async fn append_long(
        context: &mut dyn JavaContext,
        mut this: JvmClassInstanceHandle<Self>,
        value: i64,
    ) -> JavaResult<JvmClassInstanceHandle<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({:?}, {:?})", &this, value);

        let digits = value.to_string();

        Self::append(context, &mut this, &digits).await?;

        Ok(this)
    }

    async fn append_character(
        context: &mut dyn JavaContext,
        mut this: JvmClassInstanceHandle<Self>,
        value: u16,
    ) -> JavaResult<JvmClassInstanceHandle<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({:?}, {:?})", &this, value);

        let value = RustString::from_utf16(&[value])?;

        Self::append(context, &mut this, &value).await?;

        Ok(this)
    }

    async fn to_string(context: &mut dyn JavaContext, this: JvmClassInstanceHandle<Self>) -> JavaResult<JvmClassInstanceHandle<String>> {
        tracing::debug!("java.lang.StringBuffer::toString({:?})", &this);

        let java_value: JvmClassInstanceHandle<Array<JavaChar>> = context.jvm().get_field(&this, "value", "[C")?;
        let count: i32 = context.jvm().get_field(&this, "count", "I")?;

        let string = context.jvm().new_class("java/lang/String", "([CII)V", (java_value, 0, count)).await?;

        Ok(string.into())
    }

    async fn ensure_capacity(context: &mut dyn JavaContext, this: &mut JvmClassInstanceHandle<Self>, capacity: usize) -> JavaResult<()> {
        let java_value_array = context.jvm().get_field(this, "value", "[C")?;
        let current_capacity = context.jvm().array_length(&java_value_array)?;

        if current_capacity < capacity {
            let old_values: Vec<JavaChar> = context.jvm().load_array(&java_value_array, 0, current_capacity)?;
            let new_capacity = capacity * 2;

            let mut java_new_value_array = context.jvm().instantiate_array("C", new_capacity).await?;
            context.jvm().put_field(this, "value", "[C", java_new_value_array.clone())?;
            context.jvm().store_array(&mut java_new_value_array, 0, old_values)?;
        }

        Ok(())
    }

    async fn append(context: &mut dyn JavaContext, this: &mut JvmClassInstanceHandle<Self>, string: &str) -> JavaResult<()> {
        let current_count: i32 = context.jvm().get_field(this, "count", "I")?;

        let value_to_add = string.encode_utf16().collect::<Vec<_>>();
        let count_to_add = value_to_add.len() as i32;

        StringBuffer::ensure_capacity(context, this, (current_count + count_to_add) as _).await?;

        let mut java_value_array = context.jvm().get_field(this, "value", "[C")?;
        context.jvm().store_array(&mut java_value_array, current_count as _, value_to_add)?;
        context.jvm().put_field(this, "count", "I", current_count + count_to_add)?;

        Ok(())
    }
}
