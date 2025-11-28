//! A simple library for consuming `tokio_postgres::row::Row` data into structs that derive the `RowConsumer` trait.
//!
//! This crate provides a variety of derivable implementations that can be used to consume PostgreSQL data depending on preference.
//! - `from_row`
//! - `from_rows`
//! - `consume`
//! - `consume_json` if feature `consume_json` is enabled
//!
//! The latter implementations are built from `from_row`.
//!
//! ## Features
//! A variety of features provide support for additional implementation and types.
//!
//! | Feature | Description | Extra dependencies | Default |
//! | ------- | ----------- | ------------------ | ------- |
//! | `bit_{0_6, 0_7, 0_8}` | Implements crate on `bit_vec::BitVec` | bit-vec | No |
//! | `chrono_0_4` | Implements crate on types supplied by chrono | chrono | No |
//! | `consume_json_1` | Implements `consume_json` on classes that derive the `RowConsumer` trait | serde, serde_json | No |
//! | `geo_0_7` | Implements crate on `geo_types::Point<f64>`, `geo_types::Rect<f64>`, and `geo_types::LineString<f64>` | geo-types | No |
//! | `mac_1` | Implements crate on `eui48::MacAddress` | eui48 | No |
//! | `json_1` | Implements crate on `serde_json::Value` | serde_json | No |
//! | `time_0_3` | Implements crate on types supplied by time | time | No |
//! | `uuid_1` | Implements crate on `uuid::Uuid` | uuid | No |
//!
//! ## Examples
//! You may use `consume` to consume PostgreSQL row data into a struct like so.
//!
//! ```
//! # tokio_test::block_on(async {
//! use pgde::ConsumeError;
//! use pgde::RowConsumer;
//! use pgde_derive::RowConsumer;
//! use tokio_postgres::{NoTls, Row};
//!
//! #[derive(RowConsumer)]
//! struct Foo {
//!     Id: i32,
//!     Data: String,
//! }
//!
//! match tokio_postgres::connect("host=localhost user=postgres password=password dbname=postgres", NoTls).await {
//!     Ok(v) => {
//!         let client = v.0;
//!         let conn = v.1;
//!
//!         tokio::spawn(async move {
//!             if let Err(e) = conn.await {
//!                 eprintln!("connection error: {}", e);
//!             }
//!         });
//!
//!         let query = "select * from public.\"Foo\";";
//!
//!         match Foo::consume(&client, query, &[]).await {
//!             Ok(v) => { // v is of type Vec<Foo>
//!                 match v.first() {
//!                     Some(v) => println!("Id {} has Data {}", v.Id, v.Data),
//!                     None => eprintln!("No data in table"),
//!                 }
//!             },
//!             Err(v) => match v {
//!                 ConsumeError::ConversionError => eprintln!("Could not convert data"),
//!                 ConsumeError::DatabaseConnectionError => eprintln!("Database errored on processing the query"),
//!             },
//!         };
//!     },
//!     Err(_) => eprintln!("Could not connect to database"),
//! };
//! # })
//! ```
//!
//! Types of `Vec<T>` and `Option<T>`, where `T` implements `FromSql`, are also supported on structs, or as standalone consuming types, that derive `RowConsumer`. When querying nullable fields, it is best to wrap field types in an `Option<>`. See the `RowConsumer` trait for use examples of `from_row` and `from_rows`.
//!
//! This crate also provides implementations on a variety of data types, some provided by enabling features.
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
//! | `Vec<u8>` | `default` |
//! | `String` | `default` |
//! | `SystemTime` | `default` |
//! | `IpAddr` | `default` |
//! | `bit_vec::BitVec` | `bit_{0_6, 0_7, 0_8}` |
//! | `chrono::NaiveDateTime` | `chrono_0_4` |
//! | `chrono::DateTime<Utc>` | `chrono_0_4` |
//! | `chrono::DateTime<Local>` | `chrono_0_4` |
//! | `chrono::DateTime<FixedOffset>` | `chrono_0_4` |
//! | `chrono::NaiveDate` | `chrono_0_4` |
//! | `chrono::NaiveTime` | `chrono_0_4` |
//! | `geo_types::Point<f64>` | `geo_0_7` |
//! | `geo_types::Rect<f64>` | `geo_0_7` |
//! | `geo_types::LineString<f64>` | `geo_0_7` |
//! | `eui48::MacAddress` | `mac_1` |
//! | `serde_json::Value` | `json_1` |
//! | `time::PrimitiveDateTime` | `time_0_3` |
//! | `time::OffsetDateTime` | `time_0_3` |
//! | `time::Date` | `time_0_3` |
//! | `time::Time` | `time_0_3` |
//! | `uuid::Uuid` | `uuid_1` |
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
#[cfg(feature = "bit_0_6")]
use bit_vec::BitVec;
#[cfg(feature = "bit_0_7")]
use bit_vec::BitVec;
#[cfg(feature = "bit_0_8")]
use bit_vec::BitVec;
#[cfg(feature = "chrono_0_4")]
use chrono::prelude::*;
#[cfg(feature = "mac_1")]
use eui48::MacAddress;
#[cfg(feature = "geo_0_7")]
use geo_types::coord;
#[cfg(feature = "geo_0_7")]
use geo_types::line_string;
#[cfg(feature = "geo_0_7")]
use geo_types::LineString;
#[cfg(feature = "geo_0_7")]
use geo_types::Point;
#[cfg(feature = "geo_0_7")]
use geo_types::Rect;
#[cfg(feature = "consume_json_1")]
use serde::Serialize;
use std::future::Future;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::time::SystemTime;
#[cfg(feature = "time_0_3")]
use time::{Date, OffsetDateTime, PrimitiveDateTime, Time};
use tokio_postgres::row::Row;
use tokio_postgres::types::ToSql;
use tokio_postgres::Client;
#[cfg(feature = "uuid_1")]
use uuid::Uuid;

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
    /// Here's an abridged version of the `from_rows` implementation in this crate.
    ///
    /// ```
    /// use pgde::ConsumeError;
    /// use pgde::RowConsumer;
    /// use tokio_postgres::Row;
    ///
    /// fn from_rows(rows: Vec<Row>) -> Result<Vec<String>, Vec<String>>
    /// where
    ///     String: Sized,
    /// {
    ///     let mut has_issue = false;
    ///     let mut data = Vec::with_capacity(rows.len());
    ///
    ///     for row in rows.into_iter() {
    ///         match String::from_row(row) {
    ///             Ok(v) => data.push(v),
    ///             Err((v, _)) => {
    ///                 has_issue = true;
    ///                 data.push(v);
    ///             }
    ///         }
    ///     }
    ///
    ///     match has_issue {
    ///         false => Ok(data),
    ///         true => Err(data),
    ///     }
    /// }
    /// ```
    fn from_row(row: Row) -> Result<Self, (Self, Vec<String>)>
    where
        Self: Sized;

    /// The n-row consumer built off of the unit row consumer. Returns successfully
    /// converted data on error, but provides no breakdown of the errors that occurred.
    ///
    /// ## Example
    /// Here's an abridged version of the `consume` implementation in this crate.
    ///
    /// ```
    /// use pgde::ConsumeError;
    /// use pgde::RowConsumer;
    /// use std::future::Future;
    /// use tokio_postgres::Client;
    /// use tokio_postgres::types::ToSql;
    ///
    /// fn consume<'a>(
    ///     conn: &'a Client,
    ///     query: &'a str,
    ///     params: &'a [&'a (dyn ToSql + Sync)],
    /// ) -> impl Future<Output = Result<Vec<String>, ConsumeError>> + Send + 'a
    /// where
    ///     String: Sized,
    /// {
    ///     async move {
    ///         match conn.query(query, params).await {
    ///             Ok(v) => match String::from_rows(v) {
    ///                 Ok(v) => Ok(v),
    ///                 Err(_) => Err(ConsumeError::ConversionError),
    ///             },
    ///             Err(_) => Err(ConsumeError::DatabaseConnectionError),
    ///         }
    ///     }
    /// }
    /// ```
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
    ///
    /// ## Example
    /// You may use `consume` to consume PostgreSQL row data into a struct like so.
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// use pgde::ConsumeError;
    /// use pgde::RowConsumer;
    /// use pgde_derive::RowConsumer;
    /// use tokio_postgres::{NoTls, Row};
    ///
    /// #[derive(RowConsumer)]
    /// struct Foo {
    ///     Id: i32,
    ///     Data: String,
    /// }
    ///
    /// match tokio_postgres::connect("host=localhost user=postgres password=password dbname=postgres", NoTls).await {
    ///     Ok(v) => {
    ///         let client = v.0;
    ///         let conn = v.1;
    ///
    ///         tokio::spawn(async move {
    ///             if let Err(e) = conn.await {
    ///                 eprintln!("connection error: {}", e);
    ///             }
    ///         });
    ///
    ///         let query = "select * from public.\"Foo\";";
    ///
    ///         match Foo::consume(&client, query, &[]).await {
    ///             Ok(v) => { // v is of type Vec<Foo>
    ///                 match v.first() {
    ///                     Some(v) => println!("Id {} has Data {}", v.Id, v.Data),
    ///                     None => eprintln!("No data in table"),
    ///                 }
    ///             },
    ///             Err(v) => match v {
    ///                 ConsumeError::ConversionError => eprintln!("Could not convert data"),
    ///                 ConsumeError::DatabaseConnectionError => eprintln!("Database errored on processing the query"),
    ///             },
    ///         };
    ///     },
    ///     Err(_) => eprintln!("Could not connect to database"),
    /// };
    /// # })
    /// ```
    ///
    /// This crate also provides implementations on a variety of data types, some provided
    /// by enabling features, that can be used by doing something like the following.
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// use pgde::ConsumeError;
    /// use pgde::RowConsumer;
    /// use tokio_postgres::{NoTls, Row};
    ///
    /// match tokio_postgres::connect("host=localhost user=postgres password=password dbname=postgres", NoTls).await {
    ///     Ok(v) => {
    ///         let client = v.0;
    ///         let conn = v.1;
    ///
    ///         tokio::spawn(async move {
    ///             if let Err(e) = conn.await {
    ///                 eprintln!("connection error: {}", e);
    ///             }
    ///         });
    ///
    ///         let query = "select 1;";
    ///
    ///         match i32::consume(&client, query, &[]).await {
    ///             Ok(v) => { // v is of type Vec<i32>
    ///                 match v.first() {
    ///                     Some(v) => println!("1 is {}", *v),
    ///                     None => eprintln!("No data received"),
    ///                 }
    ///             },
    ///             Err(v) => match v {
    ///                 ConsumeError::ConversionError => eprintln!("Could not convert data"),
    ///                 ConsumeError::DatabaseConnectionError => eprintln!("Database errored on processing the query"),
    ///             },
    ///         };
    ///     },
    ///     Err(_) => eprintln!("Could not connect to database"),
    /// };
    /// # })
    /// ```
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
    ///
    /// ## Example
    /// `consume_json` can be used similarly to `consume`.
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// use pgde::ConsumeError;
    /// use pgde::RowConsumer;
    /// use pgde_derive::RowConsumer;
    /// use serde::Serialize;
    /// use tokio_postgres::{NoTls, Row};
    ///
    /// #[derive(Serialize, RowConsumer)]
    /// struct Foo {
    ///     Id: i32,
    ///     Data: String,
    /// }
    ///
    /// match tokio_postgres::connect("host=localhost user=postgres password=password dbname=postgres", NoTls).await {
    ///     Ok(v) => {
    ///         let client = v.0;
    ///         let conn = v.1;
    ///
    ///         tokio::spawn(async move {
    ///             if let Err(e) = conn.await {
    ///                 eprintln!("connection error: {}", e);
    ///             }
    ///         });
    ///
    ///         let query = "select * from public.\"Foo\";";
    ///
    ///         match Foo::consume_json(&client, query, &[]).await {
    ///             Ok(v) => { // v is of type String
    ///                 println!("Received json data...\n{}", v);
    ///             },
    ///             Err(v) => eprintln!("An error occurred while querying database"),
    ///         };
    ///     },
    ///     Err(_) => eprintln!("Could not connect to database"),
    /// };
    /// # })
    /// ```
    #[cfg(feature = "consume_json_1")]
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

/// A macro for implementing `from_row` on primitive types or types outside of this crate
/// that implement `FromSql`. Used internally to implement `from_row` on `bool`, `i32`,
/// `String`, etc.
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

/// A macro for implementing `from_row` on primitive types or types outside of this crate
/// that implement `FromSql`. Used internally to implement `from_row` on `SystemTime` and
/// `IpAddr`.
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

pg_type_implementation![
    bool,
    i8,
    i16,
    i32,
    u32,
    i64,
    f32,
    f64,
    String,
    Vec<u8>,
    Vec<bool>,
    Vec<i8>,
    Vec<i16>,
    Vec<i32>,
    Vec<u32>,
    Vec<i64>,
    Vec<f32>,
    Vec<f64>,
    Vec<String>,
    Vec<Vec<u8>>,
    Option<bool>,
    Option<i8>,
    Option<i16>,
    Option<i32>,
    Option<u32>,
    Option<i64>,
    Option<f32>,
    Option<f64>,
    Option<String>,
    Option<Vec<u8>>
];

pg_type_expr_implementation![
    SystemTime,
    SystemTime::now(),
    Vec<SystemTime>,
    vec![SystemTime::now()],
    Option<SystemTime>,
    Some(SystemTime::now()),
    IpAddr,
    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
    Vec<IpAddr>,
    vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))],
    Option<IpAddr>,
    Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
];

#[cfg(feature = "bit_0_6")]
pg_type_implementation![BitVec, Vec<BitVec>, Option<BitVec>];
#[cfg(feature = "bit_0_7")]
pg_type_implementation![BitVec, Vec<BitVec>, Option<BitVec>];
#[cfg(feature = "bit_0_8")]
pg_type_implementation![BitVec, Vec<BitVec>, Option<BitVec>];

#[cfg(feature = "chrono_0_4")]
pg_type_implementation![
    NaiveDateTime,
    DateTime<Utc>,
    DateTime<Local>,
    DateTime<FixedOffset>,
    NaiveDate,
    NaiveTime,
    Vec<NaiveDateTime>,
    Vec<DateTime<Utc>>,
    Vec<DateTime<Local>>,
    Vec<DateTime<FixedOffset>>,
    Vec<NaiveDate>,
    Vec<NaiveTime>,
    Option<NaiveDateTime>,
    Option<DateTime<Utc>>,
    Option<DateTime<Local>>,
    Option<DateTime<FixedOffset>>,
    Option<NaiveDate>,
    Option<NaiveTime>
];

#[cfg(feature = "geo_0_7")]
pg_type_implementation![Point<f64>, Vec<Point<f64>>, Option<Point<f64>>];

#[cfg(feature = "geo_0_7")]
pg_type_expr_implementation![
    Rect<f64>,
    Rect::new(coord! { x: 0., y: 0.}, coord! { x: 1., y: 1.},),
    Vec<Rect<f64>>,
    vec![Rect::new(coord! { x: 0., y: 0.}, coord! { x: 1., y: 1.},)],
    Option<Rect<f64>>,
    Some(Rect::new(coord! { x: 0., y: 0.}, coord! { x: 1., y: 1.},)),
    LineString<f64>,
    line_string![
        (x: 0., y: 0.),
        (x: 1., y: 1.),
    ],
    Vec<LineString<f64>>,
    vec![line_string![
        (x: 0., y: 0.),
        (x: 1., y: 1.),
    ]],
    Option<LineString<f64>>,
    Some(line_string![
        (x: 0., y: 0.),
        (x: 1., y: 1.),
    ])
];

#[cfg(feature = "mac_1")]
pg_type_implementation![MacAddress, Vec<MacAddress>, Option<MacAddress>];

#[cfg(feature = "time_0_3")]
pg_type_expr_implementation![
    PrimitiveDateTime,
    PrimitiveDateTime::MIN,
    OffsetDateTime,
    OffsetDateTime::UNIX_EPOCH,
    Date,
    Date::MIN,
    Time,
    Time::MIDNIGHT,
    Vec<PrimitiveDateTime>,
    vec![PrimitiveDateTime::MIN],
    Vec<OffsetDateTime>,
    vec![OffsetDateTime::UNIX_EPOCH],
    Vec<Date>,
    vec![Date::MIN],
    Vec<Time>,
    vec![Time::MIDNIGHT],
    Option<PrimitiveDateTime>,
    Some(PrimitiveDateTime::MIN),
    Option<OffsetDateTime>,
    Some(OffsetDateTime::UNIX_EPOCH),
    Option<Date>,
    Some(Date::MIN),
    Option<Time>,
    Some(Time::MIDNIGHT)
];

#[cfg(feature = "uuid_1")]
pg_type_implementation![Uuid, Vec<Uuid>, Option<Uuid>];

#[cfg(feature = "json_1")]
pg_type_implementation![
    serde_json::Value,
    Vec<serde_json::Value>,
    Option<serde_json::Value>
];
