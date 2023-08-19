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

    {
        conn.prepare_statement("create table test(id primary key,name VARCHAR(255));")?
            .execute_update()?;
    }
    {
        let row = conn
            .prepare_statement("insert into test(id,name) values(1,'Tom');")?
            .execute_update()?;
        assert_eq!(row, 1);
        let row = conn
            .prepare_statement("insert into test(id,name) values(2,'Jerry');")?
            .execute_update()?;
        assert_eq!(row, 1);
    }
    {
        let mut query = conn.prepare_statement("select id,name from test")?;
        let mut result = query.execute_query()?;
        let mut meta_data = result.get_meta_data()?;

        let columns = meta_data.get_columns_name()?;
        assert_eq!(columns, vec!["id", "name"]);

        // Row 1
        assert_eq!(result.next()?, true);
        assert_eq!(result.get_row()?, 1);
        assert_eq!(result.get_int(1)?, 1);
        assert_eq!(result.get_long(1)?, 1_i64);
        assert_eq!(result.get_float(1)?, 1_f32);
        assert_eq!(result.get_int_by_label("id")?, 1);
        assert_eq!(result.get_string(2)?, "Tom");
        assert_eq!(result.get_string_by_label("name")?, "Tom");

        // Row 2
        assert_eq!(result.next()?, true);
        assert_eq!(result.get_row()?, 2);
        assert_eq!(result.get_int(1)?, 2);
        assert_eq!(result.get_long(1)?, 2_i64);
        assert_eq!(result.get_float(1)?, 2_f32);
        assert_eq!(result.get_int_by_label("id")?, 2);
        assert_eq!(result.get_string(2)?, "Jerry");
        assert_eq!(result.get_string_by_label("name")?, "Jerry");
    }

    Ok(())
}
