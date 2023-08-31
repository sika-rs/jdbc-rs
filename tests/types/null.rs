use crate::{test, wait};

test!(
    fn test_bytes() {
        for (_, ds) in crate::util::DB_MAP.iter() {
            let conn = wait!(ds.get_connection())?;
            let mut statement = wait!(conn.prepare_statement("select NULL"))?;
            let result = wait!(statement.execute_query())?;
            assert_eq!(result.next()?, true);
            assert_eq!(result.get_string(1)?, None);
            assert_eq!(result.get_timestamp_millis(1)?, None);
            assert_eq!(result.get_bytes(1)?, None);
            assert!(result.get_binary_stream(1)?.is_none());
            assert_eq!(result.was_null()?, true);
        }
    }
);
