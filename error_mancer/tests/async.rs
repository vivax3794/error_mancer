use error_mancer::prelude::*;

async fn foo() -> i32 {
    10
}

#[errors]
#[derive(PartialEq, Eq)]
async fn async_works() -> Result<i32, _> {
    Ok(foo().await)
}

#[tokio::test]
async fn test_async() {
    assert_eq!(async_works().await, Ok(10));
}
