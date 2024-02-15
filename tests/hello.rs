mod test_helper;

use jvm::JvmResult;

use test_helper::run_class;

#[futures_test::test]
async fn test_hello() -> JvmResult<()> {
    let hello = include_bytes!("../test_data/Hello.class");

    let result = run_class("Hello", &[("Hello", hello)], &[]).await?;
    assert_eq!(result, "Hello, world!\n");

    Ok(())
}
