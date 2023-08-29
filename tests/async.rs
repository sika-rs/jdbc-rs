#[macro_use]
extern crate lazy_static;
mod util;

#[cfg(feature = "async")]
mod test_async {
    use jdbc::{DataSource, IStatement};

    use crate::util;

    #[test]
    fn test() {
        #[cfg(feature = "async-std")]
        {
            async_std::task::spawn(async {
                let _ = test_async().await;
            });
        }
        #[cfg(feature = "tokio")]
        {
            tokio_test::block_on(async {
                let _ = test_async().await;
            });
        }
    }

    async fn test_async() -> Result<(), jdbc::errors::Error> {
        let ds = util::sqlite();
        let ds1 = ds.clone();

        #[cfg(feature = "async-std")]
        {
            let j1 = async_std::task::spawn(async move { query(ds).await });
            let j2 = async_std::task::spawn(async move { query(ds1).await });
            j1.await?;
            j2.await?;
        }

        #[cfg(feature = "tokio")]
        {
            // Task 1
            let j1 = tokio::spawn(async move { query(ds).await });
            // Task 2
            let j2 = tokio::spawn(async move { query(ds1).await });
            j1.await??;
            j2.await??;
        }
        Ok(())
    }

    async fn query(ds: DataSource) -> Result<(), jdbc::errors::Error> {
        let conn: jdbc::Connection = ds.get_connection().await?;
        conn.create_statement()
            .await?
            .execute_update("create table test(id primary key,name VARCHAR(255))")
            .await?;

        let count = conn
            .prepare_statement(r#"insert into test(id,name) values(1,"Tom")"#)
            .await?
            .execute_update()
            .await?;
        assert_eq!(count, 1);

        let statement = conn
            .prepare_statement("select name from test where id=1")
            .await?;
        let result = statement.execute_query().await?;
        assert!(result.next()?);
        let value = result.get_string(1)?.unwrap_or("".into());
        assert_eq!(value, "Tom");

        Ok(())
    }
}
