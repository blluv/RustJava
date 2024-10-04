use alloc::{boxed::Box, sync::Arc};

use async_lock::RwLock;

use crate::{
    runtime::{JavaLangClass, JavaLangClassLoader},
    ClassDefinition, ClassInstance, Jvm, Result,
};

#[derive(Clone)]
pub struct Class {
    pub definition: Box<dyn ClassDefinition>,
    java_class: Arc<RwLock<Option<Box<dyn ClassInstance>>>>,
}

impl Class {
    pub fn new(definition: Box<dyn ClassDefinition>, java_class: Option<Box<dyn ClassInstance>>) -> Self {
        Self {
            definition,
            java_class: Arc::new(RwLock::new(java_class)),
        }
    }

    #[async_recursion::async_recursion]
    pub async fn java_class(&self, jvm: &Jvm) -> Result<Box<dyn ClassInstance>> {
        let java_class = self.java_class.read().await;
        if let Some(x) = &*java_class {
            Ok(x.clone())
        } else {
            drop(java_class);

            // class registered while bootstrapping might not have java/lang/Class, so instantiate it lazily
            let java_class = JavaLangClass::from_rust_class(jvm, self.definition.clone(), None).await?;

            self.java_class.write().await.replace(java_class.clone());

            Ok(java_class)
        }
    }
}

#[async_trait::async_trait]
pub trait BootstrapClassLoader: Sync + Send {
    async fn load_class(&self, jvm: &Jvm, name: &str) -> Result<Option<Box<dyn ClassDefinition>>>;
}

#[async_trait::async_trait]
pub trait ClassLoaderWrapper: Sync + Send {
    async fn load_class(&self, jvm: &Jvm, name: &str) -> Result<Option<Class>>;
}

pub struct BootstrapClassLoaderWrapper<'a> {
    bootstrap_class_loader: &'a dyn BootstrapClassLoader,
}

impl<'a> BootstrapClassLoaderWrapper<'a> {
    pub fn new(bootstrap_class_loader: &'a dyn BootstrapClassLoader) -> Self {
        Self { bootstrap_class_loader }
    }
}

#[async_trait::async_trait]
impl ClassLoaderWrapper for BootstrapClassLoaderWrapper<'_> {
    async fn load_class(&self, jvm: &Jvm, name: &str) -> Result<Option<Class>> {
        let definition = self.bootstrap_class_loader.load_class(jvm, name).await?;
        if let Some(definition) = definition {
            let java_class = jvm.register_class(definition.clone(), None).await?;

            Ok(Some(Class {
                definition,
                java_class: Arc::new(RwLock::new(java_class)),
            }))
        } else {
            Ok(None)
        }
    }
}

pub struct JavaClassLoaderWrapper {
    class_loader: Box<dyn ClassInstance>,
}

impl JavaClassLoaderWrapper {
    pub fn new(class_loader: Box<dyn ClassInstance>) -> Self {
        Self { class_loader }
    }
}

#[async_trait::async_trait]
impl ClassLoaderWrapper for JavaClassLoaderWrapper {
    async fn load_class(&self, jvm: &Jvm, name: &str) -> Result<Option<Class>> {
        let class = JavaLangClassLoader::load_class(jvm, &self.class_loader, name).await?;

        if let Some(class) = class {
            let definition = JavaLangClass::to_rust_class(jvm, &class).await?;
            Ok(Some(Class {
                definition,
                java_class: Arc::new(RwLock::new(Some(class))),
            }))
        } else {
            Ok(None)
        }
    }
}
