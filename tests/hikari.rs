#[macro_use]
extern crate lazy_static;
mod util;

#[test]
fn test() -> Result<(), jdbc::errors::Error> {
    let datasource = util::sqlite();
    let conn = datasource.get_connection()?;

    let data = ["Tom", "Jerry", "Spike"];

    // Create Table
    conn.prepare_statement("create table test(id primary key,name VARCHAR(255));")?
        .execute_update()?;

    // Insert Data
    for i in 0..data.len() {
        let mut statement = conn
            .prepare_statement("insert into test(id,name) values(?,?);")?
            .set_int(1, i as i32 + 1)?
            .set_string(2, data[i])?;
        let row = statement.execute_update()?;
        assert_eq!(row, 1);
    }

    // Read Data
    let statement = conn.prepare_statement("select id,name from test")?;
    let result = statement.execute_query()?;
    let meta_data = result.get_meta_data()?;

    let columns = meta_data.get_columns_name()?;
    assert_eq!(columns, vec!["id", "name"]);

    for i in 0..data.len() {
        let row = i as i32 + 1;
        let id = row;
        let name = data[i];
        assert_eq!(result.next()?, true);
        // Get Row Number
        assert_eq!(result.get_row()?, row);
        // get column 1 by column index
        assert_eq!(result.get_int(1)?, Some(id));
        assert_eq!(result.get_long(1)?, Some(id as i64));
        assert_eq!(result.get_float(1)?, Some(id as f32));
        assert_eq!(result.was_null()?, false);
        // get column 1 by column label
        assert_eq!(result.get_int_by_label("id")?, Some(id));
        // get column 2
        assert_eq!(result.get_string(2)?, Some(name.into()));
        assert_eq!(result.get_string_by_label("name")?, Some(name.into()));
        assert_eq!(result.was_null()?, false);
    }

    Ok(())
}
