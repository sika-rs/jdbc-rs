use jdbc::{errors::Error, Datasource};

mod util;

#[test]
fn test() -> Result<(), jdbc::errors::Error> {
    let ds = util::sqlite();
    let conn = ds.get_connection()?;

    let statement = conn.prepare_statement("select ?")?.set_string(1, "value")?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert_eq!(result.get_string(1)?, "value");

    test_null(&ds)?;

    Ok(())
}

fn test_null(ds: &Datasource) -> Result<(), jdbc::errors::Error> {
    let conn = ds.get_connection()?;

    let statement = conn.prepare_statement("select NULL")?;
    let result = statement.execute_query()?;
    assert_eq!(result.next()?, true);
    assert!(matches!(
        result.get_string(1),
        Err(Error::JniError(jni::errors::Error::NullPtr(_)))
    ));

    Ok(())
}
