A simple library for consuming `tokio_postgres::row::Row` data into structs that derive the `RowConsumer` trait.

Most of the complex PostgreSQL types are not supported, namely arrays. Consequently `Vec` types on structs are not currently supported.

## Example
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

With the `consume_json` feature you get access to `consume_json`, which returns json data of type `serde_json::Value`.
```
let jsonOutput = Foo::consume_json(conn, query, &[]).await;
```

## Features

| Feature | Description | Extra dependencies | Default |
| ------- | ----------- | ------------------ | ------- |
| `consume_json` | Implements `consume_json` on classes that derive the `RowConsumer` trait | serde, serde_json | No |
| `json` | Implements `from_row` on `serde_json::Value` | serde_json | No |