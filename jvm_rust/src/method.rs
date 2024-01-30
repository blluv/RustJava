use alloc::{
    boxed::Box,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};
use core::{
    fmt::{self, Debug, Formatter},
    ops::{Deref, DerefMut},
};

use classfile::{AttributeInfo, AttributeInfoCode, MethodInfo};
use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{JavaType, JavaValue, Jvm, JvmCallback, JvmResult, Method};

use crate::interpreter::Interpreter;

pub enum MethodBody {
    ByteCode(AttributeInfoCode),
    Rust(Box<dyn JvmCallback>),
}

impl MethodBody {
    pub fn from_rust(callback: Box<dyn JvmCallback>) -> Self {
        Self::Rust(callback)
    }
}

impl Debug for MethodBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MethodBody::ByteCode(_) => write!(f, "ByteCode"),
            MethodBody::Rust(_) => write!(f, "Rust"),
        }
    }
}

#[derive(Debug)]
struct MethodInner {
    name: String,
    descriptor: String,
    body: MethodBody,
    access_flags: MethodAccessFlags,
}

#[derive(Clone, Debug)]
pub struct MethodImpl {
    inner: Rc<MethodInner>,
}

impl MethodImpl {
    pub fn new(name: &str, descriptor: &str, body: MethodBody, access_flags: MethodAccessFlags) -> Self {
        Self {
            inner: Rc::new(MethodInner {
                name: name.to_string(),
                descriptor: descriptor.to_string(),
                body,
                access_flags,
            }),
        }
    }

    pub fn from_method_proto<C, Context>(proto: JavaMethodProto<C>, context: Context) -> Self
    where
        C: ?Sized + 'static,
        Context: DerefMut + Deref<Target = C> + Clone + 'static,
    {
        struct MethodProxy<C, Context>
        where
            C: ?Sized,
            Context: DerefMut + Deref<Target = C> + Clone,
        {
            body: Box<dyn java_class_proto::MethodBody<anyhow::Error, C>>,
            context: Context,
        }

        #[async_trait::async_trait(?Send)]
        impl<C, Context> JvmCallback for MethodProxy<C, Context>
        where
            C: ?Sized,
            Context: DerefMut + Deref<Target = C> + Clone,
        {
            async fn call(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> JvmResult<JavaValue> {
                let mut context = self.context.clone();

                self.body.call(jvm, &mut context, args).await
            }
        }

        Self::new(
            &proto.name,
            &proto.descriptor,
            MethodBody::Rust(Box::new(MethodProxy { body: proto.body, context })),
            proto.access_flags,
        )
    }

    pub fn from_methodinfo(method_info: MethodInfo) -> Self {
        Self {
            inner: Rc::new(MethodInner {
                name: method_info.name.to_string(),
                descriptor: method_info.descriptor.to_string(),
                body: MethodBody::ByteCode(Self::extract_body(method_info.attributes).unwrap()),
                access_flags: method_info.access_flags,
            }),
        }
    }

    fn extract_body(attributes: Vec<AttributeInfo>) -> Option<AttributeInfoCode> {
        for attribute in attributes {
            if let AttributeInfo::Code(x) = attribute {
                return Some(x);
            }
        }

        None
    }
}

#[async_trait::async_trait(?Send)]
impl Method for MethodImpl {
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    fn descriptor(&self) -> String {
        self.inner.descriptor.clone()
    }

    fn access_flags(&self) -> MethodAccessFlags {
        self.inner.access_flags
    }

    async fn run(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> JvmResult<JavaValue> {
        Ok(match &self.inner.body {
            MethodBody::ByteCode(x) => {
                let r#type = JavaType::parse(&self.inner.descriptor);
                Interpreter::run(jvm, x, args, r#type.as_method().1).await?
            }
            MethodBody::Rust(x) => x.call(jvm, args).await?,
        })
    }
}
