use alloc::vec;

use java_runtime_base::{
    Array, JavaClassProto, JavaContext, JavaFieldAccessFlag, JavaFieldProto, JavaMethodFlag, JavaMethodProto, JavaResult, JvmClassInstanceHandle,
};

// class java.io.ByteArrayInputStream
pub struct ByteArrayInputStream {}

impl ByteArrayInputStream {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: Some("java/io/InputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([B)V", Self::init, JavaMethodFlag::NONE),
                JavaMethodProto::new("available", "()I", Self::available, JavaMethodFlag::NONE),
                JavaMethodProto::new("read", "([BII)I", Self::read, JavaMethodFlag::NONE),
                JavaMethodProto::new("read", "()I", Self::read_byte, JavaMethodFlag::NONE),
                JavaMethodProto::new("close", "()V", Self::close, JavaMethodFlag::NONE),
            ],
            fields: vec![
                JavaFieldProto::new("buf", "[B", JavaFieldAccessFlag::NONE),
                JavaFieldProto::new("pos", "I", JavaFieldAccessFlag::NONE),
            ],
        }
    }

    async fn init(context: &mut dyn JavaContext, mut this: JvmClassInstanceHandle<Self>, data: JvmClassInstanceHandle<Array<i8>>) -> JavaResult<()> {
        tracing::debug!("java.lang.ByteArrayInputStream::<init>({:?}, {:?})", &this, &data);

        context.jvm().put_field(&mut this, "buf", "[B", data)?;
        context.jvm().put_field(&mut this, "pos", "I", 0)?;

        Ok(())
    }

    async fn available(context: &mut dyn JavaContext, this: JvmClassInstanceHandle<Self>) -> JavaResult<i32> {
        tracing::debug!("java.lang.ByteArrayInputStream::available({:?})", &this);

        let buf = context.jvm().get_field(&this, "buf", "[B")?;
        let pos: i32 = context.jvm().get_field(&this, "pos", "I")?;
        let buf_length = context.jvm().array_length(&buf)? as i32;

        Ok((buf_length - pos) as _)
    }

    async fn read(
        context: &mut dyn JavaContext,
        mut this: JvmClassInstanceHandle<Self>,
        b: JvmClassInstanceHandle<Array<i8>>,
        off: i32,
        len: i32,
    ) -> JavaResult<i32> {
        tracing::debug!("java.lang.ByteArrayInputStream::read({:?}, {:?}, {}, {})", &this, &b, off, len);

        let buf = context.jvm().get_field(&this, "buf", "[B")?;
        let buf_length = context.jvm().array_length(&buf)?;
        let pos: i32 = context.jvm().get_field(&this, "pos", "I")?;

        let available = (buf_length as i32 - pos) as _;
        let len_to_read = if len > available { available } else { len };
        if len_to_read == 0 {
            return Ok(0);
        }

        context
            .jvm()
            .invoke_static(
                "java/lang/System",
                "arraycopy",
                "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                (buf, pos, b, off, len_to_read),
            )
            .await?;

        context.jvm().put_field(&mut this, "pos", "I", pos + len)?;

        Ok(len)
    }

    async fn read_byte(context: &mut dyn JavaContext, mut this: JvmClassInstanceHandle<Self>) -> JavaResult<i8> {
        tracing::debug!("java.lang.ByteArrayInputStream::readByte({:?})", &this);

        let buf = context.jvm().get_field(&this, "buf", "[B")?;
        let buf_length = context.jvm().array_length(&buf)?;
        let pos: i32 = context.jvm().get_field(&this, "pos", "I")?;

        if pos as usize >= buf_length {
            return Ok(-1);
        }

        let result = context.jvm().load_byte_array(&buf, pos as _, 1)?[0];

        context.jvm().put_field(&mut this, "pos", "I", pos + 1)?;

        Ok(result)
    }

    async fn close(_: &mut dyn JavaContext, this: JvmClassInstanceHandle<ByteArrayInputStream>) -> JavaResult<()> {
        tracing::debug!("java.lang.ByteArrayInputStream::close({:?})", &this);

        Ok(())
    }
}
