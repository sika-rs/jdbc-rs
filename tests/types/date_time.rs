use chrono::Utc;

use crate::{test, wait};

test!(
    fn test_bytes() {
        for (db, ds) in crate::util::DB_MAP.iter() {
            let sql = {
                match db.as_str() {
                    "sqlite" => r#"select strftime("%Y-%m-%d %H:%M:%f", "now") as now"#,
                    _ => "select now() as now",
                }
            };
            let conn = wait!(ds.get_connection())?;
            let mut statement = wait!(conn.prepare_statement(sql))?;
            let now_timestamp = Utc::now().timestamp_millis();
            let result = wait!(statement.execute_query())?;

            // Timestamp
            assert_eq!(result.next()?, true);
            let timestamp = result.get_timestamp_millis(1)?.unwrap_or(0);
            assert!((now_timestamp - timestamp).abs() < 1000);
            let timestamp = result.get_timestamp_millis_by_label("now")?.unwrap_or(0);
            assert!((now_timestamp - timestamp).abs() < 1000);

            // chrono DateTime<Local>
            let local = result.get_local_time(1)?.unwrap();
            assert!((now_timestamp - local.timestamp_millis()).abs() < 1000);
            let local = result.get_local_time_by_label("now")?.unwrap();
            assert!((now_timestamp - local.timestamp_millis()).abs() < 1000);

            // chrono DateTime<Utc>
            let local = result.get_utc_time(1)?.unwrap();
            assert!((now_timestamp - local.timestamp_millis()).abs() < 1000);
            let local = result.get_utc_time_by_label("now")?.unwrap();
            assert!((now_timestamp - local.timestamp_millis()).abs() < 1000);

            // NULL
            let mut statement = wait!(conn.prepare_statement("select NULL"))?;
            let result = wait!(statement.execute_query())?;
            assert_eq!(result.next()?, true);
            assert_eq!(result.get_local_time(1)?, None);
            assert_eq!(result.get_utc_time(1)?, None);
        }
    }
);
