use jdbc::Datasource;

mod util;

#[test]
fn test() -> Result<(), jdbc::errors::Error> {
    let ds = util::sqlite();
    let conn = ds.get_connection()?;

    let statement = conn.prepare_statement("select ?")?.set_short(1, 1)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_short(1)?, 1);

    let statement = conn.prepare_statement("select ?")?.set_int(1, i32::MAX)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_int(1)?, i32::MAX);

    let statement = conn.prepare_statement("select ?")?.set_long(1, i64::MAX)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_long(1)?, i64::MAX);

    let statement = conn.prepare_statement("select ?")?.set_float(1, f32::MAX)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_float(1)?, f32::MAX);

    let statement = conn
        .prepare_statement("select ?")?
        .set_double(1, f64::MAX)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_double(1)?, f64::MAX);

    let statement = conn.prepare_statement("select ?")?.set_boolean(1, true)?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_boolean(1)?, true);

    test_null(&ds)?;
    Ok(())
}

fn test_null(ds: &Datasource) -> Result<(), jdbc::errors::Error> {
    let conn = ds.get_connection()?;

    let statement = conn.prepare_statement("select NULL")?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_short(1)?, 0);

    let statement = conn.prepare_statement("select NULL")?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_int(1)?, 0);

    let statement = conn.prepare_statement("select NULL")?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_long(1)?, 0);

    let statement = conn.prepare_statement("select NULL")?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_float(1)?, 0.0);

    let statement = conn.prepare_statement("select NULL")?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_double(1)?, 0.0);

    let statement = conn.prepare_statement("select NULL")?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_boolean(1)?, false);

    Ok(())
}