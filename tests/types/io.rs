use std::io::Read;

use crate::{test, wait};

test!(
    fn test_binary_stream() {
        for (db, ds) in crate::util::DB_MAP.iter() {
            if db.as_str() == "sqlite" {
                // java.sql.SQLFeatureNotSupportedException
                continue;
            }
            let bytes = "hello world".as_bytes();
            let conn = wait!(ds.get_connection()).expect("Failed to get connect.");
            let mut statement = wait!(conn.prepare_statement("select ?"))
                .expect("Failed to create statement.")
                // Set an OutputStream
                .set_binary_stream(1, bytes)
                .expect("Failed to set output stream.");
            let result = wait!(statement.execute_query()).expect("Failed to execute query.");
            assert!(result.next()?);
            // Read an InputStream
            let input: Option<jdbc::wrapper::io::InputStream> = result
                .get_binary_stream(1)
                .expect("Failed to get input stream.");
            assert!(input.is_some());
            if let Some(mut input) = input {
                let mut buf = String::new();
                let len: usize = input.read_to_string(&mut buf).unwrap_or(0);
                assert_eq!(len, bytes.len());
                assert_eq!(buf, "hello world");
            }
        }
    }
);
