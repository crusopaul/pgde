A simple library for consuming `tokio_postgres::row::Row` data into structs that derive the `RowConsumer` trait.

Most of the complex PostgreSQL types are not supported, namely arrays. Consequently `Vec` types on structs are not currently supported.

## Features

| Feature | Description | Extra dependencies | Default |
| ------- | ----------- | ------------------ | ------- |
| `consume_json` | Implements `consume_json` on classes that derive the `RowConsumer` trait | serde, serde_json | No |
| `json` | Implements `from_row` on `serde_json::Value` | serde_json | No |

## Examples
### `consume`
You may use `consume` to consume PostgreSQL row data into a struct like so:
```
#[derive(RowConsumer)]
struct Foo {
    Id: i32,
    Data: String,
}

...

let query = "select * from public.\"Foo\";";

match Foo::consume(conn, query, &[]).await {
    Ok(v) => ..., // v is of type Vec<Foo>
    Err(v) => ...,
};
```

This crate implements `from_row` on the following types so that `consume` can be used in a similar fashion
- `bool`
- `i8`
- `i16`
- `i32`
- `u32`
- `i64`
- `f32`
- `f64`
- `String`

```
let query = "select Id from public.\"Foo\";";

match i32::consume(conn, query, &[]).await {
    Ok(v) => ..., // v is of type Vec<i32>
    Err(v) => ...,
};
```

### Features
The `json` feature provides `consume` on `serde_json::Value` for json data types in PostgreSQL.

With the `consume_json` feature you get access to `consume_json`, which returns json data in a `String`.
```
#[derive(Serialize, RowConsumer)]
struct Foo { ... }

...

match Foo::consume_json(conn, query, &[]).await {
    Ok(v) => ..., // v is of type String
    Err(v) => ...,
};
```