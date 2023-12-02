use jvm::{ClassDefinition, ClassLoader, JavaValue, Jvm, Method, MethodBody};

struct FileClassLoader {
    class_name: String,
    data: Vec<u8>,
}

impl FileClassLoader {
    pub fn new(class_name: &str, data: &[u8]) -> Self {
        Self {
            class_name: class_name.to_string(),
            data: data.to_vec(),
        }
    }
}

impl ClassLoader for FileClassLoader {
    fn load(&mut self, class_name: &str) -> anyhow::Result<Option<ClassDefinition>> {
        if class_name == self.class_name {
            let class = ClassDefinition::from_classfile(&self.data)?;

            Ok(Some(class))
        } else {
            Ok(None)
        }
    }
}

struct TestLocalClassLoader {}

impl TestLocalClassLoader {
    fn java_lang_string_init() -> JavaValue {
        JavaValue::Void
    }
}

impl ClassLoader for TestLocalClassLoader {
    fn load(&mut self, class_name: &str) -> anyhow::Result<Option<ClassDefinition>> {
        if class_name == "java/lang/String" {
            let class = ClassDefinition::new(
                "java/lang/String",
                vec![Method::new("<init>", "()V", MethodBody::Rust(Box::new(Self::java_lang_string_init)))],
                vec![],
            );

            Ok(Some(class))
        } else {
            Ok(None)
        }
    }
}

#[test]
fn test_hello() -> anyhow::Result<()> {
    let hello = include_bytes!("../test_data/Hello.class");

    let mut jvm = Jvm::new();
    jvm.add_class_loader(FileClassLoader::new("Hello", hello));
    jvm.add_class_loader(TestLocalClassLoader {});

    jvm.invoke_static_method("Hello", "main", "([Ljava/lang/String;)V")?;

    Ok(())
}
