use jdbc::wrapper::{
    hikari::{HikariConfig, HikariDataSource},
    properties::Properties,
    sql::DataSource,
};

mod util;

#[test]
fn test() -> Result<(), jdbc::errors::Error> {
    let vm = util::test_vm();
    let mut env = vm.attach_current_thread()?;

    let mut props = Properties::new(&mut env)?;
    props.set_property("jdbcUrl", "jdbc:sqlite::memory:")?;
    props.set_property("maximumPoolSize", "1000")?;
    props.set_property("driverClassName", "org.sqlite.JDBC")?;
    let config = HikariConfig::new(&mut env, props)?;

    let datasource = HikariDataSource::new(&mut env, config)?;

    let mut datasource = DataSource::from_ref(&mut env, datasource.into())?;
    let mut conn = datasource.get_connection()?;

    let data = ["Tom", "Jerry", "Spike"];

    // Create Table
    conn.prepare_statement("create table test(id primary key,name VARCHAR(255));")?
        .execute_update()?;

    // Insert Data
    for i in 0..data.len() {
        let mut statement = conn.prepare_statement("insert into test(id,name) values(?,?);")?;
        statement.set_int(1, i as i32 + 1)?;
        statement.set_string(2, data[i])?;
        let row = statement.execute_update()?;
        assert_eq!(row, 1);
    }

    // Read Data
    let mut statement = conn.prepare_statement("select id,name from test")?;
    let mut result = statement.execute_query()?;
    let mut meta_data = result.get_meta_data()?;

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
        assert_eq!(result.get_int(1)?, id);
        assert_eq!(result.get_long(1)?, id as i64);
        assert_eq!(result.get_float(1)?, id as f32);
        // get column 1 by column label
        assert_eq!(result.get_int_by_label("id")?, id);
        // get column 2
        assert_eq!(result.get_string(2)?, name);
        assert_eq!(result.get_string_by_label("name")?, name);
    }

    Ok(())
}
