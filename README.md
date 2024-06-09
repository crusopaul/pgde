A simple library for consuming `tokio_postgres::row::Row` data into structs that derive the `RowConsumer` trait.

This crate provides a variety of derivable implementations that can be used to consume PostgreSQL data depending on preference.
- `from_row`
- `from_rows`
- `consume`
- `consume_json` if feature `consume_json` is enabled

The latter implementations are built from `from_row`.

Most of the complex PostgreSQL types are not supported, namely arrays. Consequently `Vec` types on structs are not currently supported.

## Features
A variety of features provide support for additional implementation and types.

| Feature | Description | Extra dependencies | Default |
| ------- | ----------- | ------------------ | ------- |
| `consume_json` | Implements `consume_json` on classes that derive the `RowConsumer` trait | serde, serde_json | No |
| `geo` | Implements crate on `geo_types::Point<f64>`, `geo_types::Rect<f64>`, and `geo_types::LineString<f64>` | geo-types | No |
| `mac` | Implements crate on `eui48::MacAddress` | eui48 | No |
| `json` | Implements crate on `serde_json::Value` | serde_json | No |
| `uuid` | Implements crate on `uuid::Uuid` | uuid | No |

## Examples
You may use `consume` to consume PostgreSQL row data into a struct like so.

```
# tokio_test::block_on(async {
use pgde::ConsumeError;
use pgde::RowConsumer;
use pgde_derive::RowConsumer;
use tokio_postgres::{NoTls, Row};

#[derive(RowConsumer)]
struct Foo {
    Id: i32,
    Data: String,
}

match tokio_postgres::connect("host=localhost user=postgres password=password dbname=postgres", NoTls).await {
    Ok(v) => {
        let client = v.0;
        let conn = v.1;

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("connection error: {}", e);
            }
        });

        let query = "select * from public.\"Foo\";";

        match Foo::consume(&client, query, &[]).await {
            Ok(v) => { // v is of type Vec<Foo>
                match v.first() {
                    Some(v) => println!("Id {} has Data {}", v.Id, v.Data),
                    None => eprintln!("No data in table"),
                }
            },
            Err(v) => match v {
                ConsumeError::ConversionError => eprintln!("Could not convert data"),
                ConsumeError::DatabaseConnectionError => eprintln!("Database errored on processing the query"),
            },
        };
    },
    Err(_) => eprintln!("Could not connect to database"),
};
# })
```

See the `RowConsumer` trait for examples of `from_row` and `from_rows`.

This crate also provides implementations on a variety of data types, some provided by enabling features.

| Type | Feature |
| ---- | ------- |
| `bool` | `default` |
| `i8` | `default` |
| `i16` | `default` |
| `i32` | `default` |
| `u32` | `default` |
| `i64` | `default` |
| `f32` | `default` |
| `f64` | `default` |
| `Vec<u8>` | `default` |
| `String` | `default` |
| `SystemTime` | `default` |
| `IpAddr` | `default` |
| `geo_types::Point<f64>` | `geo` |
| `geo_types::Rect<f64>` | `geo` |
| `geo_types::LineString<f64>` | `geo` |
| `eui48::MacAddress` | `mac` |
| `serde_json::Value` | `json` |
| `uuid::Uuid` | `uuid` |

## Testing
Testing requires access to a PostgreSQL database with no tables. Setting the following environment variables will allow you to test.

| Environment Variable | Description |
| -------------------- | ----------- |
| `PGDE_DB_HOST` | The host that the database can be accessed at. |
| `POSTGRES_USER` | The user credential to provide. |
| `POSTGRES_PASSWORD` | The password to provide. |
| `POSTGRES_DB` | The name of the database to use for testing. |