use chrono::Utc;
use jdbc::{errors::Error, wrapper::sql::ResultSet, Connection};
mod util;

#[test]
fn test() -> Result<(), jdbc::errors::Error> {
    let ds = util::sqlite();
    let conn = ds.get_connection()?;

    let statement = conn.prepare_statement("select ?")?.set_string(1, "value")?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_string(1)?, "value");

    test_null(&conn)?;

    test_time(&conn)?;

    Ok(())
}

fn test_time(conn: &Connection) -> Result<(), jdbc::errors::Error> {
    let statement =
        conn.prepare_statement(r#"select strftime("%Y-%m-%d %H:%M:%f", "now") as now"#)?;
    let now_timestamp = Utc::now().timestamp_millis();

    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    let timestamp = result.get_timestamp_millis(1)?;
    assert!((now_timestamp - timestamp).abs() < 1000);
    let timestamp = result.get_timestamp_millis_by_label("now")?;
    assert!((now_timestamp - timestamp).abs() < 1000);

    if cfg!(feature = "chrono") {
        test_chrono(&result, now_timestamp)?;
    }

    Ok(())
}

#[cfg(not(feature = "chrono"))]
fn test_chrono(_: &ResultSet, _: i64) -> Result<(), jdbc::errors::Error> {
    Ok(())
}
#[cfg(feature = "chrono")]
fn test_chrono(result: &ResultSet, now_timestamp: i64) -> Result<(), jdbc::errors::Error> {
    let local = result.get_local_time(1)?;
    assert!((now_timestamp - local.timestamp_millis()).abs() < 1000);
    let local = result.get_local_time_by_label("now")?;
    assert!((now_timestamp - local.timestamp_millis()).abs() < 1000);

    let local = result.get_utc_time(1)?;
    assert!((now_timestamp - local.timestamp_millis()).abs() < 1000);
    let local = result.get_utc_time_by_label("now")?;
    assert!((now_timestamp - local.timestamp_millis()).abs() < 1000);
    Ok(())
}

fn test_null(conn: &Connection) -> Result<(), jdbc::errors::Error> {
    let statement = conn.prepare_statement("select NULL")?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert!(matches!(
        result.get_string(1),
        Err(Error::JniError(jni::errors::Error::NullPtr(_)))
    ));

    assert!(matches!(
        result.get_timestamp_millis(1),
        Err(Error::JniError(jni::errors::Error::NullPtr(_)))
    ));

    Ok(())
}
