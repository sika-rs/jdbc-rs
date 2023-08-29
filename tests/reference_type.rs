#[cfg(not(feature = "async"))]
use chrono::Utc;

mod util;

init!();

#[cfg(not(feature = "async"))]
#[test]
fn test() -> Result<(), jdbc::errors::Error> {
    let conn = DS.get_connection()?;

    let statement = conn.prepare_statement("select ?")?.set_string(1, "value")?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_string(1)?, Some("value".into()));

    // bytes
    let statement = conn
        .prepare_statement("select ? as value")?
        .set_bytes(1, "hello world".as_bytes())?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_bytes(1)?, Some("hello world".into()));

    Ok(())
}

#[cfg(not(feature = "async"))]
#[test]
fn test_time() -> Result<(), jdbc::errors::Error> {
    let conn = DS.get_connection()?;
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
    let conn = DS.get_connection()?;

    let statement = conn.prepare_statement("select NULL")?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);

    assert_eq!(result.get_string(1)?, None);
    assert_eq!(result.get_timestamp_millis(1)?, None);
    assert_eq!(result.get_bytes(1)?, None);

    assert_eq!(result.was_null()?, true);
    Ok(())
}

#[cfg(not(feature = "async"))]
#[cfg(feature = "chrono")]
#[test]
fn test_chrono() -> Result<(), jdbc::errors::Error> {
    let conn = DS.get_connection()?;
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

#[cfg(not(feature = "async"))]
#[test]
pub fn test_big_decimal() -> Result<(), jdbc::errors::Error> {
    use bigdecimal::BigDecimal;
    use jni::objects::JValueGen;
    let mut env = util::VM.attach_current_thread()?;
    let num = "-1290000000000000000000000.4167500";
    let rust_num = num.parse().unwrap_or(BigDecimal::from(0));
    let java_string = env.new_string(num)?;
    let java_num = env.new_object(
        "java/math/BigDecimal",
        "(Ljava/lang/String;)V",
        &[JValueGen::Object(&java_string)],
    )?;

    let java_num = jdbc::util::to_string(&mut env, &java_num)?;
    assert_eq!(rust_num.to_string(), java_num);

    let conn = DS.get_connection()?;
    let statement = conn
        .prepare_statement("select ? as value")?
        .set_big_decimal(1, &rust_num)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_string(1)?, Some(num.into()));
    assert_eq!(result.get_big_decimal(1)?.as_ref(), Some(&rust_num));
    assert_eq!(
        result.get_big_decimal_by_label("value")?.as_ref(),
        Some(&rust_num)
    );
    Ok(())
}
