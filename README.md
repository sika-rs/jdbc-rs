# jdbc-rs

[中文](./README_cn.md) | English

Rust bindings to the Java Jdbc.

Many databases today do not have official support for Rust, so many databases cannot be connected using Rust. However, most databases officially provide Java version of JDBC drivers, and these drivers are very stable.

Using this library, you can use Java's JDBC to connect to any type of database.

Rust does not have an official database-driven abstraction, and the most widely used unofficial crate is Sqlx. The ultimate purpose of this library is to provide the implementation of sqlx to temporarily support databases not supported by Sqlx.

`English documentation uses Google Translate.`

# Getting Started

### Introduce Rust dependencies

Cargo.toml
```
[dependencies]
jdbc = "0.1.0"
```

### Download java dependencies

1、Download the jar file from the web.

You can download the jar file from [MVN Repository](https://mvnrepository.com/) or other ways, and put it in the `libs` directory of the project.

2、use maven

Create a new `pom.xml` in the project root directory, you can copy this file from the project root directory.

Execute the mvn command to copy dependencies to the target directory. Or copy `build.rs` from this project, please modify the target directory by yourself.

```
mvn dependency:copy-dependencies -DoutputDirectory=./libs
```

### Example

```
    let datasource = jdbc::Builder::new()
        .jdbc_url("jdbc:sqlite::memory:")
        .build()
        .expect("Failed to initialize data source.");

    let conn = datasource.get_connection()?;

    let statement = conn
        .prepare_statement("select username,age from user where id=?")?
        .set_int(1, 1000)?;

    let result = statement.execute_query()?;

    while result.next()? {
        let username = result.get_string(1)?;
        let age = result.get_int(1)?;
        println!("user:{:?} age:{:?}", username, age);
    }
```

# Support type

- [x] byte
- [x] short
- [x] int
- [x] long
- [x] float
- [x] double
- [x] boolean
- [x] String
- [x] Date
- [ ] Object
- [ ] Blob
- [ ] Clob
- [x] byte[]
- [x] BigDecimal
- [ ] AsciiStream
- [ ] BinaryStream
- [ ] CharacterStream