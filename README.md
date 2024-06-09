A simple library for consuming `tokio_postgres::row::Row` data into structs that derive the `RowConsumer` trait.

Most of the complex PostgreSQL types are not supported, namely arrays. Consequently `Vec` types on structs are not currently supported.

## Features

| Feature | Description | Extra dependencies | Default |
| ------- | ----------- | ------------------ | ------- |
| `consume_json` | Implements `consume_json` on classes that derive the `RowConsumer` trait | serde, serde_json | No |
| `json` | Implements `consume` on `serde_json::Value` | serde_json | No |
| `uuid` | Implements `consume` on `uuid::Uuid` | uuid | No |

## Examples
### `consume`
You may use `consume` to consume PostgreSQL row data into a struct like so.
```
#[derive(RowConsumer)]
struct Foo {
    Id: i32,
    Data: String,
}

...

let query = "select * from public.\"Foo\";";

match Foo::consume(&conn, query, &[]).await {
    Ok(v) => ..., // v is of type Vec<Foo>
    Err(v) => ...,
};
```

This crate implements `from_row` on the following types so that `consume` can be used in a similar fashion.

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
| `eui48::MacAddress` | `mac` |
| `serde_json::Value` | `json` |
| `uuid::Uuid` | `uuid` |

```
let query = "select Id from public.\"Foo\";";

match i32::consume(&conn, query, &[]).await {
    Ok(v) => ..., // v is of type Vec<i32>
    Err(v) => ...,
};
```

### Features
The following features provide `consume` on the corresponding types.

| Feature | Rust Type |
| ------- | --------- |
| `json` | `serde_json::Value` |
| `mac` | `eui48::MacAddress` |
| `uuid` | `uuid::Uuid` |

With the `consume_json` feature you get access to `consume_json`, which returns json data in a `String`.
```
#[derive(Serialize, RowConsumer)]
struct Foo { ... }

...

match Foo::consume_json(&conn, query, &[]).await {
    Ok(v) => ..., // v is of type String
    Err(v) => ...,
};
```

## Testing
Testing requires access to a PostgreSQL database with no tables. Setting the following environment variables will allow you to test.

| Environment Variable | Description |
| -------------------- | ----------- |
| `PGDE_DB_HOST` | The host that the database can be accessed at. |
| `POSTGRES_USER` | The user credential to provide. |
| `POSTGRES_PASSWORD` | The password to provide. |
| `POSTGRES_DB` | The name of the database to use for testing. |

To test, you would then run.
```
cargo test --tests --all-features
```