//! Attempts to test a variety of `consume` scenarios for data types mentioned in the provided `FromSql` type implementations from postgres_types.
#[cfg(feature = "bit_0_6")]
use bit_vec_0_6::BitVec;
#[cfg(feature = "bit_0_7")]
use bit_vec_0_7::BitVec;
#[cfg(feature = "bit_0_8")]
use bit_vec_0_8::BitVec;
#[cfg(feature = "chrono_0_4")]
use chrono_0_4::prelude::*;
#[cfg(feature = "mac_1")]
use eui48_1::MacAddress;
#[cfg(feature = "geo_0_7")]
use geo_types_0_7::coord;
#[cfg(feature = "geo_0_7")]
use geo_types_0_7::line_string;
#[cfg(feature = "geo_0_7")]
use geo_types_0_7::point;
#[cfg(feature = "geo_0_7")]
use geo_types_0_7::Rect;
use pgde::RowConsumer;
use pgde_derive::RowConsumer;
#[cfg(feature = "consume_json_1")]
use serde_1::{self, Serialize};
#[cfg(feature = "json_1")]
use serde_json_1::json;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::time::SystemTime;
#[cfg(feature = "time_0_3")]
use time_0_3::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time};
use tokio_postgres::Row;
use tokio_postgres::{Client, NoTls};
#[cfg(feature = "uuid_1")]
use uuid_1::Uuid;

#[macro_export]
macro_rules! db_env_assertion {
    () => {{
        assert_ne!(DATABASE_HOST, "bad", "No database host provided");
        assert_ne!(DATABASE_USER, "bad", "No database user provided");
        assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
        assert_ne!(DATABASE_NAME, "bad", "No database name provided");
    }};
}

async fn connect_to_database() -> Result<Client, ()> {
    let conn_string = format!(
        "host={} user={} password={} dbname={}",
        DATABASE_HOST, DATABASE_USER, DATABASE_PASSWORD, DATABASE_NAME
    );

    match tokio_postgres::connect(&conn_string, NoTls).await {
        Ok(v) => {
            let client = v.0;
            let conn = v.1;

            tokio::spawn(async move {
                if let Err(e) = conn.await {
                    eprintln!("connection error: {}", e);
                }
            });

            Ok(client)
        }
        Err(_) => Err(()),
    }
}

const DATABASE_HOST: &str = match option_env!("PGDE_DB_HOST") {
    Some(v) => v,
    None => "bad",
};

const DATABASE_USER: &str = match option_env!("POSTGRES_USER") {
    Some(v) => v,
    None => "bad",
};

const DATABASE_PASSWORD: &str = match option_env!("POSTGRES_PASSWORD") {
    Some(v) => v,
    None => "bad",
};

const DATABASE_NAME: &str = match option_env!("POSTGRES_DB") {
    Some(v) => v,
    None => "bad",
};

#[tokio::test]
async fn query_database() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match i32::consume(&v, "select 1", &[]).await {
            Ok(v) => match v.last() {
                Some(v) => {
                    assert_eq!(*v, 1);
                    Ok(())
                }
                None => Err(String::from("Cannot query database")),
            },
            Err(_) => Err(String::from("Could not convert 1 to i32")),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
async fn manage_tables() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists manage_tables (
                    foo int
                );",
                &[],
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
async fn consume_boolean() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_boolean (
                    field1 boolean
                );",
                &[],
            )
            .await
        {
            Ok(_) => match v
                .query(
                    "insert into public.\"consume_boolean\" values ( true ), (false);",
                    &[],
                )
                .await
            {
                Ok(_) => {
                    match bool::consume(
                        &v,
                        "select field1 from public.\"consume_boolean\" order by 1 desc;",
                        &[],
                    )
                    .await
                    {
                        Ok(result) => match result.first() {
                            Some(result_value) => {
                                assert!(*result_value, "Could not consume boolean into bool");

                                match result.last() {
                                    Some(result_value) => {
                                        assert_eq!(
                                            *result_value, false,
                                            "Could not consume boolean into bool"
                                        );

                                        Ok(())
                                    }
                                    None => {
                                        Err(String::from("Could not consume boolean into bool"))
                                    }
                                }
                            }
                            None => Err(String::from("Could not consume boolean into bool")),
                        },
                        Err(_) => Err(String::from("Could not consume boolean into bool")),
                    }
                }
                Err(v) => Err(v.to_string()),
            },
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
async fn consume_char() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_char (
                    field1 \"char\"
                );",
                &[],
            )
            .await
        {
            Ok(_) => match v
                .query(
                    "insert into public.\"consume_char\" values ( 97::\"char\" );",
                    &[],
                )
                .await
            {
                Ok(_) => {
                    match i8::consume(&v, "select field1 from public.\"consume_char\";", &[]).await
                    {
                        Ok(result) => match result.last() {
                            Some(result_value) => {
                                assert_eq!(*result_value, 97, "Could not consume \"char\" into i8");
                                Ok(())
                            }
                            None => Err(String::from("Could not consume \"char\" into i8")),
                        },
                        Err(_) => Err(String::from("Could not consume \"char\" into i8")),
                    }
                }
                Err(v) => Err(v.to_string()),
            },
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
async fn consume_i16() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_i16 (
                    field1 smallint,
                    field2 smallserial
                );",
                &[],
            )
            .await
        {
            Ok(_) => match v
                .query("insert into public.\"consume_i16\" values ( -9, 9 );", &[])
                .await
            {
                Ok(_) => {
                    match i16::consume(&v, "select field1 from public.\"consume_i16\";", &[]).await
                    {
                        Ok(result) => match result.last() {
                            Some(result_value) => {
                                assert_eq!(
                                    *result_value, -9,
                                    "Could not consume smallint into i16"
                                );

                                match i16::consume(
                                    &v,
                                    "select field2 from public.\"consume_i16\";",
                                    &[],
                                )
                                .await
                                {
                                    Ok(result) => match result.last() {
                                        Some(result_value) => {
                                            assert_eq!(
                                                *result_value, 9,
                                                "Could not consume smallserial into i16"
                                            );

                                            Ok(())
                                        }
                                        None => Err(String::from(
                                            "Could not consume smallserial into i16",
                                        )),
                                    },
                                    Err(_) => {
                                        Err(String::from("Could not consume smallserial into i16"))
                                    }
                                }
                            }
                            None => Err(String::from("Could not consume smallint into i16")),
                        },
                        Err(_) => Err(String::from("Could not consume smallint into i16")),
                    }
                }
                Err(v) => Err(v.to_string()),
            },
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
async fn consume_i32() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_i32 (
                    field1 int,
                    field2 serial
                );",
                &[],
            )
            .await
        {
            Ok(_) => match v
                .query(
                    "insert into public.\"consume_i32\" values ( -32769, 32768 );",
                    &[],
                )
                .await
            {
                Ok(_) => {
                    match i32::consume(&v, "select field1 from public.\"consume_i32\";", &[]).await
                    {
                        Ok(result) => match result.last() {
                            Some(result_value) => {
                                assert_eq!(*result_value, -32769, "Could not consume int into i32");

                                match i32::consume(
                                    &v,
                                    "select field2 from public.\"consume_i32\";",
                                    &[],
                                )
                                .await
                                {
                                    Ok(result) => match result.last() {
                                        Some(result_value) => {
                                            assert_eq!(
                                                *result_value, 32768,
                                                "Could not consume serial into i32"
                                            );

                                            Ok(())
                                        }
                                        None => {
                                            Err(String::from("Could not consume serial into i32"))
                                        }
                                    },
                                    Err(_) => {
                                        Err(String::from("Could not consume serial into i32"))
                                    }
                                }
                            }
                            None => Err(String::from("Could not consume int into i32")),
                        },
                        Err(_) => Err(String::from("Could not consume int into i32")),
                    }
                }
                Err(v) => Err(v.to_string()),
            },
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
async fn consume_u32() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_u32 (
                    field1 oid
                );",
                &[],
            )
            .await
        {
            Ok(_) => match v
                .query("insert into public.\"consume_u32\" values ( 564182 );", &[])
                .await
            {
                Ok(_) => {
                    match u32::consume(&v, "select field1 from public.\"consume_u32\";", &[]).await
                    {
                        Ok(result) => match result.last() {
                            Some(result_value) => {
                                assert_eq!(*result_value, 564182, "Could not consume oid into u32");
                                Ok(())
                            }
                            None => Err(String::from("Could not consume oid into u32")),
                        },
                        Err(_) => Err(String::from("Could not consume oid into u32")),
                    }
                }
                Err(v) => Err(v.to_string()),
            },
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
async fn consume_i64() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_i64 (
                    field1 bigint,
                    field2 bigserial
                );",
                &[],
            )
            .await
        {
            Ok(_) => match v
                .query(
                    "insert into public.\"consume_i64\" values ( -2147483649, 2147483648 );",
                    &[],
                )
                .await
            {
                Ok(_) => {
                    match i64::consume(&v, "select field1 from public.\"consume_i64\";", &[]).await
                    {
                        Ok(result) => match result.last() {
                            Some(result_value) => {
                                assert_eq!(
                                    *result_value, -2147483649,
                                    "Could not consume bigint into i64"
                                );

                                match i64::consume(
                                    &v,
                                    "select field2 from public.\"consume_i64\";",
                                    &[],
                                )
                                .await
                                {
                                    Ok(result) => match result.last() {
                                        Some(result_value) => {
                                            assert_eq!(
                                                *result_value, 2147483648,
                                                "Could not consume bigserial into i64"
                                            );

                                            Ok(())
                                        }
                                        None => Err(String::from(
                                            "Could not consume bigserial into i64",
                                        )),
                                    },
                                    Err(_) => {
                                        Err(String::from("Could not consume bigserial into i64"))
                                    }
                                }
                            }
                            None => Err(String::from("Could not consume bigint into i64")),
                        },
                        Err(_) => Err(String::from("Could not consume bigint into i64")),
                    }
                }
                Err(v) => Err(v.to_string()),
            },
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
async fn consume_f32() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_f32 (
                    field1 float4
                );",
                &[],
            )
            .await
        {
            Ok(_) => match v
                .query("insert into public.\"consume_f32\" values ( 1.5 );", &[])
                .await
            {
                Ok(_) => {
                    match f32::consume(&v, "select field1 from public.\"consume_f32\";", &[]).await
                    {
                        Ok(result) => match result.last() {
                            Some(result_value) => {
                                assert_eq!(*result_value, 1.5, "Could not consume float4 into f32");
                                Ok(())
                            }
                            None => Err(String::from("Could not consume float4 into f32")),
                        },
                        Err(_) => Err(String::from("Could not consume float4 into f32")),
                    }
                }
                Err(v) => Err(v.to_string()),
            },
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
async fn consume_f64() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_f64 (
                    field1 float8
                );",
                &[],
            )
            .await
        {
            Ok(_) => match v
                .query("insert into public.\"consume_f64\" values ( 1.5 );", &[])
                .await
            {
                Ok(_) => {
                    match f64::consume(&v, "select field1 from public.\"consume_f64\";", &[]).await
                    {
                        Ok(result) => match result.last() {
                            Some(result_value) => {
                                assert_eq!(*result_value, 1.5, "Could not consume float8 into f64");
                                Ok(())
                            }
                            None => Err(String::from("Could not consume float8 into f64")),
                        },
                        Err(_) => Err(String::from("Could not consume float8 into f64")),
                    }
                }
                Err(v) => Err(v.to_string()),
            },
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
async fn consume_string() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_string (
                    field1 char(11)
                );",
                &[],
            )
            .await
        {
            Ok(_) => match v
                .query(
                    "insert into public.\"consume_string\" values ( 'hello world' );",
                    &[],
                )
                .await
            {
                Ok(_) => {
                    match String::consume(&v, "select field1 from public.\"consume_string\";", &[])
                        .await
                    {
                        Ok(result) => match result.last() {
                            Some(result_value) => {
                                assert_eq!(
                                    *result_value, "hello world",
                                    "Could not consume char into String"
                                );

                                Ok(())
                            }
                            None => Err(String::from("Could not consume char into String")),
                        },
                        Err(_) => Err(String::from("Could not consume char into String")),
                    }
                }
                Err(v) => Err(v.to_string()),
            },
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
async fn consume_vec_u8() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_vec_u8 (
                    field1 bytea
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                match v
                    .query(
                        "insert into public.\"consume_vec_u8\" values ( '\\x1234'::bytea );",
                        &[],
                    )
                    .await
                {
                    Ok(_) => {
                        match Vec::<u8>::consume(
                            &v,
                            "select field1 from public.\"consume_vec_u8\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value,
                                        vec![0x12_u8, 0x34_u8,],
                                        "Could not consume bytea into Vec<u8>"
                                    );

                                    Ok(())
                                }
                                None => Err(String::from("Could not consume bytea into Vec<u8>")),
                            },
                            Err(_) => Err(String::from("Could not consume bytea into Vec<u8>")),
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
async fn consume_system_time() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_system_time (
                    field1 timestamp
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                match v
                    .query(
                        "insert into public.\"consume_system_time\" values ( $1 );",
                        &[&SystemTime::UNIX_EPOCH],
                    )
                    .await
                {
                    Ok(_) => {
                        match SystemTime::consume(
                            &v,
                            "select field1 from public.\"consume_system_time\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value,
                                        SystemTime::UNIX_EPOCH,
                                        "Could not consume timestamp into SystemTime"
                                    );
                                    Ok(())
                                }
                                None => {
                                    Err(String::from("Could not consume timestamp into SystemTime"))
                                }
                            },
                            Err(_) => {
                                Err(String::from("Could not consume timestamp into SystemTime"))
                            }
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
async fn consume_ip() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_ip (
                    field1 inet
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                match v
                    .query(
                        "insert into public.\"consume_ip\" values ( $1 );",
                        &[&IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))],
                    )
                    .await
                {
                    Ok(_) => {
                        match IpAddr::consume(&v, "select field1 from public.\"consume_ip\";", &[])
                            .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value,
                                        IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
                                        "Could not consume inet into IpAddr"
                                    );

                                    Ok(())
                                }
                                None => Err(String::from("Could not consume inet into IpAddr")),
                            },
                            Err(_) => Err(String::from("Could not consume inet into IpAddr")),
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
#[cfg(feature = "mac_1")]
async fn consume_macaddress() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_macaddress (
                    field1 macaddr
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                let test_macaddr = MacAddress::new([12_u8, 34_u8, 56_u8, 78_u8, 90_u8, 12_u8]);

                match v
                    .query(
                        "insert into public.\"consume_macaddress\" values ( $1 );",
                        &[&test_macaddr],
                    )
                    .await
                {
                    Ok(_) => {
                        match MacAddress::consume(
                            &v,
                            "select field1 from public.\"consume_macaddress\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value, test_macaddr,
                                        "Could not consume macaddr into MacAddress"
                                    );

                                    Ok(())
                                }
                                None => {
                                    Err(String::from("Could not consume macaddr into MacAddress"))
                                }
                            },
                            Err(_) => {
                                Err(String::from("Could not consume macaddr into MacAddress"))
                            }
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
#[cfg(feature = "geo_0_7")]
async fn consume_point() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_point (
                    field1 point
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                let test_point = point! { x: 1.5, y: -1.5};

                match v
                    .query(
                        "insert into public.\"consume_point\" values ( $1 );",
                        &[&test_point],
                    )
                    .await
                {
                    Ok(_) => {
                        match geo_types_0_7::Point::<f64>::consume(
                            &v,
                            "select field1 from public.\"consume_point\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value, test_point,
                                        "Could not consume point into Point<f64>"
                                    );

                                    Ok(())
                                }
                                None => {
                                    Err(String::from("Could not consume point into Point<f64>"))
                                }
                            },
                            Err(_) => Err(String::from("Could not consume point into Point<f64>")),
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
#[cfg(feature = "geo_0_7")]
async fn consume_rect() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_rect (
                    field1 box
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                let test_box = Rect::new(coord! { x: 0., y: 4.}, coord! { x: 3., y: 10.});

                match v
                    .query(
                        "insert into public.\"consume_rect\" values ( $1 );",
                        &[&test_box],
                    )
                    .await
                {
                    Ok(_) => {
                        match geo_types_0_7::Rect::<f64>::consume(
                            &v,
                            "select field1 from public.\"consume_rect\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value, test_box,
                                        "Could not consume box into Rect<f64>"
                                    );

                                    Ok(())
                                }
                                None => Err(String::from("Could not consume box into Rect<f64>")),
                            },
                            Err(_) => Err(String::from("Could not consume box into Rect<f64>")),
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
#[cfg(feature = "geo_0_7")]
async fn consume_linestring() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_linestring (
                    field1 path
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                let test_linestring = line_string![
                    (x: 0., y: 0.),
                    (x: 1., y: 1.),
                ];

                match v
                    .query(
                        "insert into public.\"consume_linestring\" values ( $1 );",
                        &[&test_linestring],
                    )
                    .await
                {
                    Ok(_) => {
                        match geo_types_0_7::LineString::<f64>::consume(
                            &v,
                            "select field1 from public.\"consume_linestring\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value, test_linestring,
                                        "Could not consume path into LineString<f64>"
                                    );

                                    Ok(())
                                }
                                None => {
                                    Err(String::from("Could not consume path into LineString<f64>"))
                                }
                            },
                            Err(_) => {
                                Err(String::from("Could not consume path into LineString<f64>"))
                            }
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
#[cfg(feature = "json_1")]
async fn consume_json() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_json (
                    field1 json,
                    field2 jsonb
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                let john = json!({
                    "name": "John Doe",
                    "age": 43,
                    "phones": [
                        "+44 1234567",
                        "+44 2345678"
                    ]
                });

                match v
                    .query(
                        "insert into public.\"consume_json\" values ( $1, $2 );",
                        &[&john, &john],
                    )
                    .await
                {
                    Ok(_) => {
                        match serde_json_1::Value::consume(
                            &v,
                            "select field1 from public.\"consume_json\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value, john,
                                        "Could not consume json into Value"
                                    );

                                    match serde_json_1::Value::consume(
                                        &v,
                                        "select field2 from public.\"consume_json\";",
                                        &[],
                                    )
                                    .await
                                    {
                                        Ok(result) => match result.last() {
                                            Some(result_value) => {
                                                assert_eq!(
                                                    *result_value, john,
                                                    "Could not consume jsonb into Value"
                                                );

                                                Ok(())
                                            }
                                            None => Err(String::from(
                                                "Could not consume jsonb into Value",
                                            )),
                                        },
                                        Err(_) => {
                                            Err(String::from("Could not consume jsonb into Value"))
                                        }
                                    }
                                }
                                None => Err(String::from("Could not consume json into Value")),
                            },
                            Err(_) => Err(String::from("Could not consume json into Value")),
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
#[cfg(feature = "uuid_1")]
async fn consume_uuid() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_uuid (
                    field1 uuid
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                let test_uuid = Uuid::new_v4();

                match v
                    .query(
                        "insert into public.\"consume_uuid\" values ( $1 );",
                        &[&test_uuid],
                    )
                    .await
                {
                    Ok(_) => {
                        match Uuid::consume(&v, "select field1 from public.\"consume_uuid\";", &[])
                            .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value, test_uuid,
                                        "Could not consume uuid into Uuid"
                                    );

                                    Ok(())
                                }
                                None => Err(String::from("Could not consume uuid into Uuid")),
                            },
                            Err(_) => Err(String::from("Could not consume uuid into Uuid")),
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
#[cfg(feature = "bit_0_6")]
async fn consume_bits() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_bits (
                    field1 bit(16),
                    field2 varbit(16)
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                let test_bits = BitVec::from_bytes(&[0b10100000, 0b00010010]);

                match v
                    .query(
                        "insert into public.\"consume_bits\" values ( $1, $2 );",
                        &[&test_bits, &test_bits],
                    )
                    .await
                {
                    Ok(_) => {
                        match BitVec::consume(
                            &v,
                            "select field1 from public.\"consume_bits\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value, test_bits,
                                        "Could not consume bit into BitVec"
                                    );

                                    match BitVec::consume(
                                        &v,
                                        "select field2 from public.\"consume_bits\";",
                                        &[],
                                    )
                                    .await
                                    {
                                        Ok(result) => match result.last() {
                                            Some(result_value) => {
                                                assert_eq!(
                                                    *result_value, test_bits,
                                                    "Could not consume varbit into BitVec"
                                                );

                                                Ok(())
                                            }
                                            None => Err(String::from(
                                                "Could not consume varbit into BitVec",
                                            )),
                                        },
                                        Err(_) => Err(String::from(
                                            "Could not consume varbit into BitVec",
                                        )),
                                    }
                                }
                                None => Err(String::from("Could not consume bit into BitVec")),
                            },
                            Err(_) => Err(String::from("Could not consume bit into BitVec")),
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
#[cfg(feature = "bit_0_7")]
async fn consume_bits() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_bits (
                    field1 bit(16),
                    field2 varbit(16)
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                let test_bits = BitVec::from_bytes(&[0b10100000, 0b00010010]);

                match v
                    .query(
                        "insert into public.\"consume_bits\" values ( $1, $2 );",
                        &[&test_bits, &test_bits],
                    )
                    .await
                {
                    Ok(_) => {
                        match BitVec::consume(
                            &v,
                            "select field1 from public.\"consume_bits\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value, test_bits,
                                        "Could not consume bit into BitVec"
                                    );

                                    match BitVec::consume(
                                        &v,
                                        "select field2 from public.\"consume_bits\";",
                                        &[],
                                    )
                                    .await
                                    {
                                        Ok(result) => match result.last() {
                                            Some(result_value) => {
                                                assert_eq!(
                                                    *result_value, test_bits,
                                                    "Could not consume varbit into BitVec"
                                                );

                                                Ok(())
                                            }
                                            None => Err(String::from(
                                                "Could not consume varbit into BitVec",
                                            )),
                                        },
                                        Err(_) => Err(String::from(
                                            "Could not consume varbit into BitVec",
                                        )),
                                    }
                                }
                                None => Err(String::from("Could not consume bit into BitVec")),
                            },
                            Err(_) => Err(String::from("Could not consume bit into BitVec")),
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
#[cfg(feature = "bit_0_8")]
async fn consume_bits() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_bits (
                    field1 bit(16),
                    field2 varbit(16)
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                let test_bits = BitVec::from_bytes(&[0b10100000, 0b00010010]);

                match v
                    .query(
                        "insert into public.\"consume_bits\" values ( $1, $2 );",
                        &[&test_bits, &test_bits],
                    )
                    .await
                {
                    Ok(_) => {
                        match BitVec::consume(
                            &v,
                            "select field1 from public.\"consume_bits\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value, test_bits,
                                        "Could not consume bit into BitVec"
                                    );

                                    match BitVec::consume(
                                        &v,
                                        "select field2 from public.\"consume_bits\";",
                                        &[],
                                    )
                                    .await
                                    {
                                        Ok(result) => match result.last() {
                                            Some(result_value) => {
                                                assert_eq!(
                                                    *result_value, test_bits,
                                                    "Could not consume varbit into BitVec"
                                                );

                                                Ok(())
                                            }
                                            None => Err(String::from(
                                                "Could not consume varbit into BitVec",
                                            )),
                                        },
                                        Err(_) => Err(String::from(
                                            "Could not consume varbit into BitVec",
                                        )),
                                    }
                                }
                                None => Err(String::from("Could not consume bit into BitVec")),
                            },
                            Err(_) => Err(String::from("Could not consume bit into BitVec")),
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
#[cfg(feature = "consume_json_1")]
async fn consume_json_impl() -> Result<(), String> {
    db_env_assertion!();

    #[derive(Serialize, RowConsumer)]
    #[serde(crate = "self::serde_1")]
    struct Foo {
        foo: i32,
        bar: i32,
    }

    match connect_to_database().await {
        Ok(v) => match Foo::consume_json(&v, "select 1, 2;", &[]).await {
            Ok(result) => {
                assert_eq!(
                    *result,
                    String::from("[{\"foo\":1,\"bar\":2}]"),
                    "Could not consume_json into struct"
                );

                Ok(())
            }
            Err(_) => Err(String::from("Could not consume_json into struct")),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
async fn consume_option() -> Result<(), String> {
    db_env_assertion!();

    #[derive(RowConsumer)]
    struct OptionConsumer {
        nullable: Option<i32>,
        not_nullable: Option<i32>,
    }

    match connect_to_database().await {
        Ok(v) => match OptionConsumer::consume(&v, "select null::int, 1;", &[]).await {
            Ok(result) => match result.last() {
                Some(result_value) => {
                    assert_eq!(
                        result_value.nullable, None,
                        "Could not consume null into Option<i32>"
                    );
                    assert_eq!(
                        result_value.not_nullable,
                        Some(1),
                        "Could not consume not-null into Option<i32>"
                    );
                    Ok(())
                }
                None => Err(String::from("Could not consume null into Option<i32>")),
            },
            Err(_) => Err(String::from("Could not consume null into Option<i32>")),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[tokio::test]
async fn consume_option_unit() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match Option::<i32>::consume(&v, "select null::int;", &[]).await {
            Ok(result) => match result.last() {
                Some(result_value) => {
                    assert_eq!(
                        *result_value, None,
                        "Could not consume null into Option<i32>"
                    );
                    Ok(())
                }
                None => Err(String::from("Could not consume null into Option<i32>")),
            },
            Err(_) => Err(String::from("Could not consume null into Option<i32>")),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[cfg(feature = "chrono_0_4")]
#[tokio::test]
async fn consume_chrono_naivedatetime() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_chrono_naivedatetime (
                    field1 timestamp
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                match v
                    .query(
                        "insert into public.\"consume_chrono_naivedatetime\" values ( $1 );",
                        &[&NaiveDateTime::default()],
                    )
                    .await
                {
                    Ok(_) => {
                        match NaiveDateTime::consume(
                            &v,
                            "select field1 from public.\"consume_chrono_naivedatetime\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value,
                                        NaiveDateTime::default(),
                                        "Could not consume timestamp into NaiveDateTime"
                                    );
                                    Ok(())
                                }
                                None => Err(String::from(
                                    "Could not consume timestamp into NaiveDateTime",
                                )),
                            },
                            Err(_) => Err(String::from(
                                "Could not consume timestamp into NaiveDateTime",
                            )),
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[cfg(feature = "chrono_0_4")]
#[tokio::test]
async fn consume_chrono_datetime_utc() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_chrono_datetime_utc (
                    field1 timestamptz
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                match v
                    .query(
                        "insert into public.\"consume_chrono_datetime_utc\" values ( $1 );",
                        &[&DateTime::<Utc>::default()],
                    )
                    .await
                {
                    Ok(_) => {
                        match DateTime::<Utc>::consume(
                            &v,
                            "select field1 from public.\"consume_chrono_datetime_utc\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value,
                                        DateTime::<Utc>::default(),
                                        "Could not consume timestamptz into DateTime<Utc>"
                                    );
                                    Ok(())
                                }
                                None => Err(String::from(
                                    "Could not consume timestamptz into DateTime<Utc>",
                                )),
                            },
                            Err(_) => Err(String::from(
                                "Could not consume timestamptz into DateTime<Utc>",
                            )),
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[cfg(feature = "chrono_0_4")]
#[tokio::test]
async fn consume_chrono_datetime_local() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_chrono_datetime_local (
                    field1 timestamptz
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                match v
                    .query(
                        "insert into public.\"consume_chrono_datetime_local\" values ( $1 );",
                        &[&DateTime::<Local>::default()],
                    )
                    .await
                {
                    Ok(_) => {
                        match DateTime::<Local>::consume(
                            &v,
                            "select field1 from public.\"consume_chrono_datetime_local\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value,
                                        DateTime::<Local>::default(),
                                        "Could not consume timestamptz into DateTime<Local>"
                                    );
                                    Ok(())
                                }
                                None => Err(String::from(
                                    "Could not consume timestamptz into DateTime<Local>",
                                )),
                            },
                            Err(_) => Err(String::from(
                                "Could not consume timestamptz into DateTime<Local>",
                            )),
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[cfg(feature = "chrono_0_4")]
#[tokio::test]
async fn consume_chrono_datetime_fixedoffset() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_chrono_datetime_fixedoffset (
                    field1 timestamptz
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                let test_datetime = FixedOffset::east_opt(5)
                    .unwrap()
                    .with_ymd_and_hms(2016, 11, 08, 0, 0, 0)
                    .unwrap();

                match v
                    .query(
                        "insert into public.\"consume_chrono_datetime_fixedoffset\" values ( $1 );",
                        &[&test_datetime],
                    )
                    .await
                {
                    Ok(_) => {
                        match DateTime::<FixedOffset>::consume(
                            &v,
                            "select field1 from public.\"consume_chrono_datetime_fixedoffset\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value, test_datetime,
                                        "Could not consume timestamptz into DateTime<FixedOffset>"
                                    );
                                    Ok(())
                                }
                                None => Err(String::from(
                                    "Could not consume timestamptz into DateTime<FixedOffset>",
                                )),
                            },
                            Err(_) => Err(String::from(
                                "Could not consume timestamptz into DateTime<FixedOffset>",
                            )),
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[cfg(feature = "chrono_0_4")]
#[tokio::test]
async fn consume_chrono_naivedate() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_chrono_naivedate (
                    field1 date
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                match v
                    .query(
                        "insert into public.\"consume_chrono_naivedate\" values ( $1 );",
                        &[&NaiveDate::default()],
                    )
                    .await
                {
                    Ok(_) => {
                        match NaiveDate::consume(
                            &v,
                            "select field1 from public.\"consume_chrono_naivedate\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value,
                                        NaiveDate::default(),
                                        "Could not consume date into NaiveDate"
                                    );
                                    Ok(())
                                }
                                None => Err(String::from("Could not consume date into NaiveDate")),
                            },
                            Err(_) => Err(String::from("Could not consume date into NaiveDate")),
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[cfg(feature = "chrono_0_4")]
#[tokio::test]
async fn consume_chrono_naivetime() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_chrono_naivetime (
                    field1 time
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                match v
                    .query(
                        "insert into public.\"consume_chrono_naivetime\" values ( $1 );",
                        &[&NaiveTime::default()],
                    )
                    .await
                {
                    Ok(_) => {
                        match NaiveTime::consume(
                            &v,
                            "select field1 from public.\"consume_chrono_naivetime\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value,
                                        NaiveTime::default(),
                                        "Could not consume time into NaiveTime"
                                    );
                                    Ok(())
                                }
                                None => Err(String::from("Could not consume time into NaiveTime")),
                            },
                            Err(_) => Err(String::from("Could not consume time into NaiveTime")),
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[cfg(feature = "time_0_3")]
#[tokio::test]
async fn consume_time_primitivedatetime() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_time_primitivedatetime (
                    field1 timestamp
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                let test_datetime = Date::from_calendar_date(2020, Month::January, 1).unwrap().midnight();

                match v
                    .query(
                        "insert into public.\"consume_time_primitivedatetime\" values ( $1 );",
                        &[&test_datetime],
                    )
                    .await
                {
                    Ok(_) => {
                        match PrimitiveDateTime::consume(
                            &v,
                            "select field1 from public.\"consume_time_primitivedatetime\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value, test_datetime,
                                        "Could not consume timestamp into PrimitiveDateTime"
                                    );
                                    Ok(())
                                }
                                None => Err(String::from(
                                    "Could not consume timestamp into PrimitiveDateTime",
                                )),
                            },
                            Err(_) => Err(String::from(
                                "Could not consume timestamp into PrimitiveDateTime",
                            )),
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[cfg(feature = "time_0_3")]
#[tokio::test]
async fn consume_time_offsetdatetime() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_time_offsetdatetime (
                    field1 timestamptz
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                let test_datetime = Date::from_calendar_date(2020, Month::January, 1).unwrap().midnight().assume_utc();

                match v
                    .query(
                        "insert into public.\"consume_time_offsetdatetime\" values ( $1 );",
                        &[&test_datetime],
                    )
                    .await
                {
                    Ok(_) => {
                        match OffsetDateTime::consume(
                            &v,
                            "select field1 from public.\"consume_time_offsetdatetime\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value, test_datetime,
                                        "Could not consume timestamptz into OffsetDateTime"
                                    );
                                    Ok(())
                                }
                                None => Err(String::from(
                                    "Could not consume timestamptz into OffsetDateTime",
                                )),
                            },
                            Err(_) => Err(String::from(
                                "Could not consume timestamptz into OffsetDateTime",
                            )),
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[cfg(feature = "time_0_3")]
#[tokio::test]
async fn consume_time_date() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_time_date (
                    field1 date
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                let test_date = Date::from_calendar_date(2020, Month::January, 1).unwrap();

                match v
                    .query(
                        "insert into public.\"consume_time_date\" values ( $1 );",
                        &[&test_date],
                    )
                    .await
                {
                    Ok(_) => {
                        match Date::consume(
                            &v,
                            "select field1 from public.\"consume_time_date\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value, test_date,
                                        "Could not consume date into Date"
                                    );
                                    Ok(())
                                }
                                None => Err(String::from("Could not consume date into Date")),
                            },
                            Err(_) => Err(String::from("Could not consume date into Date")),
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

#[cfg(feature = "time_0_3")]
#[tokio::test]
async fn consume_time_time() -> Result<(), String> {
    db_env_assertion!();

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists consume_time_time (
                    field1 time
                );",
                &[],
            )
            .await
        {
            Ok(_) => {
                let test_time = Time::from_hms(5, 0, 0).unwrap();

                match v
                    .query(
                        "insert into public.\"consume_time_time\" values ( $1 );",
                        &[&test_time],
                    )
                    .await
                {
                    Ok(_) => {
                        match Time::consume(
                            &v,
                            "select field1 from public.\"consume_time_time\";",
                            &[],
                        )
                        .await
                        {
                            Ok(result) => match result.last() {
                                Some(result_value) => {
                                    assert_eq!(
                                        *result_value, test_time,
                                        "Could not consume time into Time"
                                    );
                                    Ok(())
                                }
                                None => Err(String::from("Could not consume time into Time")),
                            },
                            Err(_) => Err(String::from("Could not consume time into Time")),
                        }
                    }
                    Err(v) => Err(v.to_string()),
                }
            }
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}
