mod test_helper;

use jvm::JvmResult;

use test_helper::run_jar;

#[futures_test::test]
async fn test_jar() -> JvmResult<()> {
    let jar = include_bytes!("../test_data/test.jar");

    let result = run_jar(jar, &[]).await?;
    assert_eq!(result, "test content\n\n");

    Ok(())
}
