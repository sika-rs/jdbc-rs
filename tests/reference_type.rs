#[cfg(not(feature = "async"))]
use chrono::Utc;

#[macro_use]
extern crate lazy_static;
mod util;

#[cfg(not(feature = "async"))]
#[test]
fn test() -> Result<(), jdbc::errors::Error> {
    let ds = util::sqlite();
    let conn = ds.get_connection()?;

    let statement = conn.prepare_statement("select ?")?.set_string(1, "value")?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_string(1)?, Some("value".into()));

    Ok(())
}

#[cfg(not(feature = "async"))]
#[test]
fn test_time() -> Result<(), jdbc::errors::Error> {
    let ds = util::sqlite();
    let conn = ds.get_connection()?;
    let statement =
        conn.prepare_statement(r#"select strftime("%Y-%m-%d %H:%M:%f", "now") as now"#)?;
    let now_timestamp = Utc::now().timestamp_millis();
    let result = statement.execute_query()?;

    assert_eq!(result.next()?, true);
    let timestamp = result.get_timestamp_millis(1)?.unwrap_or(0);
    assert!((now_timestamp - timestamp).abs() < 1000);
    let timestamp = result.get_timestamp_millis_by_label("now")?.unwrap_or(0);
    assert!((now_timestamp - timestamp).abs() < 1000);

    Ok(())
}

#[cfg(not(feature = "async"))]
#[test]
fn test_null() -> Result<(), jdbc::errors::Error> {
    let ds = util::sqlite();
    let conn = ds.get_connection()?;

    let statement = conn.prepare_statement("select NULL")?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);

    assert_eq!(result.get_string(1)?, None);
    assert_eq!(result.get_timestamp_millis(1)?, None);

    assert_eq!(result.was_null()?, true);
    Ok(())
}

#[cfg(not(feature = "async"))]
#[cfg(feature = "chrono")]
#[test]
fn test_chrono() -> Result<(), jdbc::errors::Error> {
    let ds = util::sqlite();
    let conn = ds.get_connection()?;
    let statement =
        conn.prepare_statement(r#"select strftime("%Y-%m-%d %H:%M:%f", "now") as now"#)?;
    let now_timestamp = Utc::now().timestamp_millis();
    let result = statement.execute_query()?;

    let local = result.get_local_time(1)?.unwrap();
    assert!((now_timestamp - local.timestamp_millis()).abs() < 1000);
    let local = result.get_local_time_by_label("now")?.unwrap();
    assert!((now_timestamp - local.timestamp_millis()).abs() < 1000);

    let local = result.get_utc_time(1)?.unwrap();
    assert!((now_timestamp - local.timestamp_millis()).abs() < 1000);
    let local = result.get_utc_time_by_label("now")?.unwrap();
    assert!((now_timestamp - local.timestamp_millis()).abs() < 1000);

    let statement = conn.prepare_statement("select NULL")?;
    let result = statement.execute_query()?;
    assert_eq!(result.get_local_time(1)?, None);
    assert_eq!(result.get_utc_time(1)?, None);
    Ok(())
}
