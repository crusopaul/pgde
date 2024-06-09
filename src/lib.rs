//! A simple library for consuming `tokio_postgres::row::Row` data into structs that derive the `RowConsumer` trait.
//!
//! Most of the complex PostgreSQL types are not supported, namely arrays. Consequently `Vec` types on structs are not currently supported.
//!
//! ## Features
//!
//! | Feature | Description | Extra dependencies | Default |
//! | ------- | ----------- | ------------------ | ------- |
//! | `consume_json` | Implements `consume_json` on classes that derive the `RowConsumer` trait | serde, serde_json | No |
//! | `json` | Implements `consume` on `serde_json::Value` | serde_json | No |
//! | `uuid` | Implements `consume` on `uuid::Uuid` | uuid | No |
//!
//! ## Examples
//! ### `consume`
//! You may use `consume` to consume PostgreSQL row data into a struct like so.
//! ```
//! #[derive(RowConsumer)]
//! struct Foo {
//!     Id: i32,
//!     Data: String,
//! }
//!
//! ...
//!
//! let query = "select * from public.\"Foo\";";
//!
//! match Foo::consume(&conn, query, &[]).await {
//!     Ok(v) => ..., // v is of type Vec<Foo>
//!     Err(v) => ...,
//! };
//! ```
//!
//! This crate implements `from_row` on the following types so that `consume` can be used in a similar fashion.
//!
//! | Type | Feature |
//! | ---- | ------- |
//! | `bool` | `default` |
//! | `i8` | `default` |
//! | `i16` | `default` |
//! | `i32` | `default` |
//! | `u32` | `default` |
//! | `i64` | `default` |
//! | `f32` | `default` |
//! | `f64` | `default` |
//! | `String` | `default` |
//! | `SystemTime` | `default` |
//! | `IpAddr` | `default` |
//! | `serde_json::Value` | `json` |
//! | `uuid::Uuid` | `uuid` |
//!
//! ```
//! let query = "select Id from public.\"Foo\";";
//!
//! match i32::consume(&conn, query, &[]).await {
//!     Ok(v) => ..., // v is of type Vec<i32>
//!     Err(v) => ...,
//! };
//! ```
//!
//! ### Features
//! The `json` and `uuid` features provide `consume` on `serde_json::Value` and `uuid::Uuid` for json and uuid data types in PostgreSQL.
//!
//! With the `consume_json` feature you get access to `consume_json`, which returns json data in a `String`.
//! ```
//! #[derive(Serialize, RowConsumer)]
//! struct Foo { ... }
//!
//! ...
//!
//! match Foo::consume_json(&conn, query, &[]).await {
//!     Ok(v) => ..., // v is of type String
//!     Err(v) => ...,
//! };
//! ```
//!
//! ## Testing
//! Testing requires access to a PostgreSQL database with no tables. Setting the following environment variables will allow you to test.
//!
//! | Environment Variable | Description |
//! | -------------------- | ----------- |
//! | `PGDE_DB_HOST` | The host that the database can be accessed at. |
//! | `POSTGRES_USER` | The user credential to provide. |
//! | `POSTGRES_PASSWORD` | The password to provide. |
//! | `POSTGRES_DB` | The name of the database to use for testing. |
//!
//! To test, you would then run.
//! ```
//! cargo test --tests --all-features
//! ```
#[cfg(feature = "consume_json")]
use serde::Serialize;
use std::future::Future;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::time::SystemTime;
use tokio_postgres::row::Row;
use tokio_postgres::types::ToSql;
use tokio_postgres::Client;

/// Errors that may occur during row consumption.
pub enum ConsumeError {
    ConversionError,
    DatabaseConnectionError,
}

/// The derivable trait for implementing PostgreSQL row consumption.
pub trait RowConsumer {
    /// The unit row consumer implemented by the pgde_derive crate that consumes row
    /// data into another struct. Upon error, provides field and class information for the
    /// first encountered error in the form of a String as well as partially converted
    /// data.
    ///
    /// ## Example
    /// Provided the following struct `Foo`:
    /// ```
    /// #[derive(RowConsumer)]
    /// struct Foo {
    ///     Id: i32,
    ///     Data: String,
    /// }
    /// ```
    /// The pgde_derive crate will implement the following:
    /// ```
    /// impl pgde::RowConsumer for Foo {
    ///     fn from_row(row: Row) -> Result<Foo, (Foo, Vec<String>)>
    ///     where
    ///         Foo: Sized,
    ///     {
    ///         let mut errors : Vec<String> = Vec::new();
    ///
    ///         let result = Foo {
    ///             Id: match row.try_get::<usize, i32>(0) {
    ///                 Ok(v) => v,
    ///                 Err(_) => {
    ///                     errors.push(format!("Conversion error occurred for field \"{}\" on class \"{}\"", "Id", "Foo"));
    ///                     i32::default()
    ///                 },
    ///             },
    ///             Data: match row.try_get::<usize, String>(1) {
    ///                 Ok(v) => v,
    ///                 Err(_) => {
    ///                     errors.push(format!("Conversion error occurred for field \"{}\" on class \"{}\"", "Data", "Foo"));
    ///                     String::default()
    ///                 },
    ///             },
    ///         };
    ///
    ///         match errors.first() {
    ///             None => Ok(result),
    ///             Some(v) => Err((result, v.to_string())),
    ///         }
    ///     }
    /// }
    /// ```
    fn from_row(row: Row) -> Result<Self, (Self, Vec<String>)>
    where
        Self: Sized;

    /// The n-row consumer built off of the unit row consumer. Returns successfully
    /// converted data on error, but provides no breakdown of the errors that occurred.
    fn from_rows(rows: Vec<Row>) -> Result<Vec<Self>, Vec<Self>>
    where
        Self: Sized,
    {
        let mut has_issue = false;
        let mut data = Vec::with_capacity(rows.len());

        for row in rows.into_iter() {
            match Self::from_row(row) {
                Ok(v) => data.push(v),
                Err((v, _)) => {
                    has_issue = true;
                    data.push(v);
                }
            }
        }

        match has_issue {
            false => Ok(data),
            true => Err(data),
        }
    }

    /// Consumes row data from provided connection, query, and parameters. Provides no
    /// data on error, instead provides a [ConsumeError] enum.
    fn consume(
        conn: &Client,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl Future<Output = Result<Vec<Self>, ConsumeError>> + Send
    where
        Self: Sized,
    {
        async move {
            match conn.query(query, params).await {
                Ok(v) => match Self::from_rows(v) {
                    Ok(v) => Ok(v),
                    Err(_) => Err(ConsumeError::ConversionError),
                },
                Err(_) => Err(ConsumeError::DatabaseConnectionError),
            }
        }
    }

    /// Attempts to convert the results of `consume` into a `serde_json::Value`. On
    /// error returns `serde_json::Value::Null`.
    #[cfg(feature = "consume_json")]
    fn consume_json(
        conn: &Client,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl Future<Output = Result<String, String>> + Send
    where
        Self: Serialize + Sized,
    {
        async move {
            match &Self::consume(conn, query, params).await {
                Ok(v) => match serde_json::to_string(v) {
                    Ok(v) => Ok(v),
                    Err(_) => Err(serde_json::Value::default().to_string()),
                },
                Err(_) => Err(serde_json::Value::default().to_string()),
            }
        }
    }
}

/// A macro for implementing `from_row` on primitive types. Used internally to implement
/// `from_row` on `bool`, `i32`, `String`, etc.
///
/// ## Example
/// ```
/// pg_type_implementation![bool]
/// ```
#[macro_export]
macro_rules! pg_type_implementation {
    ( $( $x:ty ),* ) => {
        $(
            impl RowConsumer for $x {
                fn from_row(row: Row) -> Result<Self, (Self, Vec<String>)>
                where
                    Self: Sized,
                {
                    let mut errors : Vec<String> = Vec::new();

                    let class_instance = match row.try_get::<usize, $x>(0) {
                        Ok(v) => v,
                        Err(_) => {
                            errors.push(format!("Conversion error occurred for class \"{}\"", stringify!($x)));
                            <$x>::default()
                        },
                    };

                    match errors.len() {
                        0 => Ok(class_instance),
                        _ => Err((class_instance, errors)),
                    }
                }
            }
        )*
    };
}

/// A macro for implementing `from_row` on primitive types that defaults to a provided
/// expression. Used internally to implement `from_row` on `SystemTime` and `IpAddr`.
///
/// ## Example
/// ```
/// pg_type_expr_implementation![
///     SystemTime,
///     SystemTime::now()
/// ]
/// ```
#[macro_export]
macro_rules! pg_type_expr_implementation {
    ( $( $x:ty, $y:expr ),* ) => {
        $(
            impl RowConsumer for $x {
                fn from_row(row: Row) -> Result<Self, (Self, Vec<String>)>
                where
                    Self: Sized,
                {
                    let mut errors : Vec<String> = Vec::new();

                    match row.try_get::<usize, $x>(0) {
                        Ok(v) => Ok(v),
                        Err(_) => {
                            errors.push(format!("Conversion error occurred for class \"{}\"", stringify!($x)));
                            Err(($y, errors))
                        },
                    }
                }
            }
        )*
    };
}

pg_type_implementation![bool, i8, i16, i32, u32, i64, f32, f64, String, Vec<u8>];

#[cfg(feature = "mac")]
pg_type_implementation![eui48::MacAddress];

#[cfg(feature = "uuid")]
pg_type_implementation![uuid::Uuid];

#[cfg(feature = "json")]
pg_type_implementation![serde_json::Value];

pg_type_expr_implementation![
    SystemTime,
    SystemTime::now(),
    IpAddr,
    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
];
