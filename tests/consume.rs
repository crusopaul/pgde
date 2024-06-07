//! Attempts to test a variety of `consume` scenarios for data types mentioned in the provided `FromSql` type implementations from postgres_types.
use pgde::RowConsumer;
use std::time::SystemTime;
use tokio_postgres::{Client, NoTls};

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

const DATABASE_USER: &str = match option_env!("PGDE_DB_USER") {
    Some(v) => v,
    None => "bad",
};

const DATABASE_PASSWORD: &str = match option_env!("PGDE_DB_PASSWORD") {
    Some(v) => v,
    None => "bad",
};

const DATABASE_NAME: &str = match option_env!("PGDE_DB_NAME") {
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
