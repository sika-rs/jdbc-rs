# jdbc-rs

中文 | [English](./README.md)

Java Jdbc的Rust绑定库。

如今很多数据库并没有对Rust的官方支持，所以很多数据库不能使用Rust连接。但是绝大多数数据库官方都提供了Java版本的JDBC驱动，这些驱动非常稳定。

使用这个库，你可以使用Java的JDBC，从而连接任意类型数据库。

Rust并没有官方的数据库驱动抽象，目前使用量最大的非官方Crate是Sqlx。本库的最终目的是提供sqlx的实现，用以临时支持Sqlx未支持的数据库。

# Getting Started

### 引入Rust依赖

Cargo.toml
```
[dependencies]
jdbc = "0.1.0"
```

### 下载java依赖

1.从网络上下载jar文件

你可以从[MVN Repository](https://mvnrepository.com/)或其他途径下载jar文件，放到项目的`libs`目录下。

2.使用maven

在项目根目录下新建`pom.xml`，你可以从本项目根目录中复制此文件。

执行mvn命令以复制依赖到目标目录。或者从本项目复制`build.rs`，请自行修改目标目录。

```
mvn dependency:copy-dependencies -DoutputDirectory=./libs
```

### 示例

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

# 支持类型

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
- [ ] BigDecimal
- [ ] AsciiStream
- [ ] BinaryStream
- [ ] CharacterStream