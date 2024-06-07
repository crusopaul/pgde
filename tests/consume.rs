//! Attempts to test a variety of `consume` scenarios for data types mentioned in the provided `FromSql` type implementations from postgres_types.
use pgde::RowConsumer;
#[cfg(feature = "consume_json")]
use pgde_derive::RowConsumer;
#[cfg(feature = "consume_json")]
use serde::Serialize;
#[cfg(feature = "json")]
use serde_json::json;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::time::SystemTime;
#[cfg(feature = "consume_json")]
use tokio_postgres::Row;
use tokio_postgres::{Client, NoTls};
#[cfg(feature = "uuid")]
use uuid::Uuid;

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
    assert_ne!(DATABASE_HOST, "bad", "No database host provided");
    assert_ne!(DATABASE_USER, "bad", "No database user provided");
    assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
    assert_ne!(DATABASE_NAME, "bad", "No database name provided");

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
    assert_ne!(DATABASE_HOST, "bad", "No database host provided");
    assert_ne!(DATABASE_USER, "bad", "No database user provided");
    assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
    assert_ne!(DATABASE_NAME, "bad", "No database name provided");

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
    assert_ne!(DATABASE_HOST, "bad", "No database host provided");
    assert_ne!(DATABASE_USER, "bad", "No database user provided");
    assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
    assert_ne!(DATABASE_NAME, "bad", "No database name provided");

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
    assert_ne!(DATABASE_HOST, "bad", "No database host provided");
    assert_ne!(DATABASE_USER, "bad", "No database user provided");
    assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
    assert_ne!(DATABASE_NAME, "bad", "No database name provided");

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
    assert_ne!(DATABASE_HOST, "bad", "No database host provided");
    assert_ne!(DATABASE_USER, "bad", "No database user provided");
    assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
    assert_ne!(DATABASE_NAME, "bad", "No database name provided");

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
    assert_ne!(DATABASE_HOST, "bad", "No database host provided");
    assert_ne!(DATABASE_USER, "bad", "No database user provided");
    assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
    assert_ne!(DATABASE_NAME, "bad", "No database name provided");

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
    assert_ne!(DATABASE_HOST, "bad", "No database host provided");
    assert_ne!(DATABASE_USER, "bad", "No database user provided");
    assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
    assert_ne!(DATABASE_NAME, "bad", "No database name provided");

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
    assert_ne!(DATABASE_HOST, "bad", "No database host provided");
    assert_ne!(DATABASE_USER, "bad", "No database user provided");
    assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
    assert_ne!(DATABASE_NAME, "bad", "No database name provided");

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
    assert_ne!(DATABASE_HOST, "bad", "No database host provided");
    assert_ne!(DATABASE_USER, "bad", "No database user provided");
    assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
    assert_ne!(DATABASE_NAME, "bad", "No database name provided");

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
    assert_ne!(DATABASE_HOST, "bad", "No database host provided");
    assert_ne!(DATABASE_USER, "bad", "No database user provided");
    assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
    assert_ne!(DATABASE_NAME, "bad", "No database name provided");

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
    assert_ne!(DATABASE_HOST, "bad", "No database host provided");
    assert_ne!(DATABASE_USER, "bad", "No database user provided");
    assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
    assert_ne!(DATABASE_NAME, "bad", "No database name provided");

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
async fn consume_system_time() -> Result<(), String> {
    assert_ne!(DATABASE_HOST, "bad", "No database host provided");
    assert_ne!(DATABASE_USER, "bad", "No database user provided");
    assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
    assert_ne!(DATABASE_NAME, "bad", "No database name provided");

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
    assert_ne!(DATABASE_HOST, "bad", "No database host provided");
    assert_ne!(DATABASE_USER, "bad", "No database user provided");
    assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
    assert_ne!(DATABASE_NAME, "bad", "No database name provided");

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
#[cfg(feature = "uuid")]
async fn consume_uuid() -> Result<(), String> {
    assert_ne!(DATABASE_HOST, "bad", "No database host provided");
    assert_ne!(DATABASE_USER, "bad", "No database user provided");
    assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
    assert_ne!(DATABASE_NAME, "bad", "No database name provided");

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
#[cfg(feature = "json")]
async fn consume_json() -> Result<(), String> {
    assert_ne!(DATABASE_HOST, "bad", "No database host provided");
    assert_ne!(DATABASE_USER, "bad", "No database user provided");
    assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
    assert_ne!(DATABASE_NAME, "bad", "No database name provided");

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
                        match serde_json::Value::consume(
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

                                    match serde_json::Value::consume(
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
#[cfg(feature = "consume_json")]
async fn consume_json_impl() -> Result<(), String> {
    assert_ne!(DATABASE_HOST, "bad", "No database host provided");
    assert_ne!(DATABASE_USER, "bad", "No database user provided");
    assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
    assert_ne!(DATABASE_NAME, "bad", "No database name provided");

    #[derive(Serialize, RowConsumer)]
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
