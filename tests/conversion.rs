use pgde::RowConsumer;
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
async fn can_query_database() -> Result<(), String> {
    assert_ne!(DATABASE_HOST, "bad", "No database host provided");
    assert_ne!(DATABASE_USER, "bad", "No database user provided");
    assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
    assert_ne!(DATABASE_NAME, "bad", "No database name provided");

    match connect_to_database().await {
        Ok(v) => match i32::consume(&v, "select 1", &[]).await {
            Ok(v) => match v.first() {
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
async fn can_manage_tables() -> Result<(), String> {
    assert_ne!(DATABASE_HOST, "bad", "No database host provided");
    assert_ne!(DATABASE_USER, "bad", "No database user provided");
    assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
    assert_ne!(DATABASE_NAME, "bad", "No database name provided");

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists can_manage_tables (
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
async fn can_consume_bool() -> Result<(), String> {
    assert_ne!(DATABASE_HOST, "bad", "No database host provided");
    assert_ne!(DATABASE_USER, "bad", "No database user provided");
    assert_ne!(DATABASE_PASSWORD, "bad", "No database password provided");
    assert_ne!(DATABASE_NAME, "bad", "No database name provided");

    match connect_to_database().await {
        Ok(v) => match v
            .query(
                "create table if not exists can_consume_bool (
                    field1 boolean
                );",
                &[],
            )
            .await
        {
            Ok(_) => match v
                .query(
                    "insert into public.\"can_consume_bool\" values ( true ), (false);",
                    &[],
                )
                .await
            {
                Ok(_) => {
                    match bool::consume(
                        &v,
                        "select field1 from public.\"can_consume_bool\" order by 1 desc;",
                        &[],
                    )
                    .await
                    {
                        Ok(result) => match result.first() {
                            Some(result_value) => {
                                assert!(*result_value, "Could not consume bool");

                                match result.last() {
                                    Some(result_value) => {
                                        assert_eq!(*result_value, false, "Could not consume bool");
                                        Ok(())
                                    }
                                    None => Err(String::from("Could not consume bool")),
                                }
                            }
                            None => Err(String::from("Could not consume bool")),
                        },
                        Err(_) => Err(String::from("Could not consume bool")),
                    }
                }
                Err(v) => Err(v.to_string()),
            },
            Err(v) => Err(v.to_string()),
        },
        Err(_) => Err(String::from("Could not connect to database")),
    }
}

// field2 int,
// field3 int,
// field4 int,
// field5 int,
// field6 int,
// field7 float4,
// field8 float8,
// field9 varchar(64),
// field10 uuid,
// field11 json,
// field12 jsonb,
// field13 timestamp,
// field14 inet
