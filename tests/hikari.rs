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
        conn.prepare_statement("create table test(id int primary key);")?
            .execute_update()?;
    }
    {
        let row = conn
            .prepare_statement("insert into test(id) values(1);")?
            .execute_update()?;

        assert_eq!(row, 1);
    }

    {
        let mut query = conn.prepare_statement("select id from test")?;
        let mut result = query.execute_query()?;
        let mut meta_data = result.get_meta_data()?;
        let columns = meta_data.get_columns_name()?;
        println!("columns:{:?}", columns);
        assert_eq!(columns, vec!["id"]);
        let row = result.get_row()?;
        // assert_eq!(row, 1);
        println!("row:{}", row);
    }

    Ok(())
}
