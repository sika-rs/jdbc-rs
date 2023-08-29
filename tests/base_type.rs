#[macro_use]
extern crate lazy_static;
mod util;

#[cfg(not(feature = "async"))]
#[test]
fn test() -> Result<(), jdbc::errors::Error> {
    let ds = util::sqlite();
    let conn = ds.get_connection()?;

    let statement = conn
        .prepare_statement("select ? as value")?
        .set_short(1, i16::MAX)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_short(1)?, Some(i16::MAX));
    assert_eq!(result.get_short_by_label("value")?, Some(i16::MAX));

    let statement = conn
        .prepare_statement("select ? as value")?
        .set_int(1, i32::MAX)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_int(1)?, Some(i32::MAX));
    assert_eq!(result.get_int_by_label("value")?, Some(i32::MAX));

    let statement = conn
        .prepare_statement("select ? as value")?
        .set_long(1, i64::MAX)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_long(1)?, Some(i64::MAX));
    assert_eq!(result.get_long_by_label("value")?, Some(i64::MAX));

    let statement = conn
        .prepare_statement("select ? as value")?
        .set_float(1, f32::MAX)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_float(1)?, Some(f32::MAX));
    assert_eq!(result.get_float_by_label("value")?, Some(f32::MAX));

    let statement = conn
        .prepare_statement("select ? as value")?
        .set_double(1, f64::MAX)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_double(1)?, Some(f64::MAX));
    assert_eq!(result.get_double_by_label("value")?, Some(f64::MAX));

    let statement = conn
        .prepare_statement("select ? as value")?
        .set_boolean(1, true)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_boolean(1)?, Some(true));
    assert_eq!(result.get_boolean_by_label("value")?, Some(true));

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
    assert_eq!(result.get_short(1)?, None);
    assert_eq!(result.get_int(1)?, None);
    assert_eq!(result.get_long(1)?, None);
    assert_eq!(result.get_float(1)?, None);
    assert_eq!(result.get_double(1)?, None);
    assert_eq!(result.get_boolean(1)?, None);
    assert_eq!(result.was_null()?, true);
    Ok(())
}
