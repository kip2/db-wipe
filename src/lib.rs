use clap::Parser;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{database, MySql, Pool};
use std::{env, error::Error, fs};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Config {
    #[arg(help = "Target database name")]
    db_name: String,
    #[arg(short = 'b', long = "backup", help = "Backup execution")]
    backup: bool,
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
        user_name, password, host, port, db_name
    );

    // Generate DB connection
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .unwrap_or_else(|_| panic!("Cannot connect to the database"));

    Ok(())
}

async fn execute_query(db: &Pool<MySql>, queries: Vec<String>) {
    // Gererate transaction
    let mut tx = db.begin().await.expect("transaction error.");

    for query in queries {
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
    }

    // transaction commit
    let _ = tx.commit().await.unwrap_or_else(|e| {
        println!("{:?}", e);
    });
}
