use crate::{test, wait};

test!(
    fn test_bytes() {
        for (_, ds) in crate::util::DB_MAP.iter() {
            let conn = wait!(ds.get_connection())?;
            let mut statement =
                wait!(conn.prepare_statement("select ?"))?.set_string(1, "value")?;
            let result = wait!(statement.execute_query())?;
            assert_eq!(result.next()?, true);
            assert_eq!(result.get_string(1)?, Some("value".into()));

            // bytes
            let mut statement = wait!(conn.prepare_statement("select ? as value"))?
                .set_bytes(1, "hello world".as_bytes())?;
            let result = wait!(statement.execute_query())?;
            assert_eq!(result.next()?, true);
            assert_eq!(result.get_bytes(1)?, Some("hello world".into()));
        }
    }
);
