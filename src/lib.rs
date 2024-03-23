use clap::Parser;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{database, MySql, Pool};
use std::{env, error::Error, fs};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Config {
    #[arg(short = 'd', long = "dump", help = "Create dump file")]
    dump: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    Ok(config)
}

pub async fn run(config: Config) -> MyResult<()> {
    // Read environment variables and generate URL
    dotenv::dotenv().expect("Failed to read .env file");

    let user_name = env::var("USER_NAME").expect("USER_NAME must be set");
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
    let password = env::var("PASSWORD").expect("PASSWORD must be set");
    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set");

    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        &user_name, &password, &host, &port, &db_name
    );

    // Generate DB connection
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .unwrap_or_else(|_| panic!("Cannot connect to the database"));

    // todo: Process to perform a dump in the dump flag is set

    delete_database(&pool, db_name).await;

    Ok(())
}

async fn delete_database(db: &Pool<MySql>, db_name: String) {
    let mut tx = db.begin().await.expect("transaction error.");

    let query = format!("DROP DATABASE {};", db_name);

    // Execute SQL query
    let result = sqlx::query(&query).execute(&mut *tx).await;

    match result {
        Ok(_) => {}
        Err(e) => {
            println!("Database query failed: {}", e);
            println!("Failed query: {:?}", &query);
            // rollback
            tx.rollback().await.expect("Transaction rollback error.");
            return;
        }
    }

    // transaction commit
    let _ = tx.commit().await.unwrap_or_else(|e| {
        println!("{:?}", e);
    });
}
