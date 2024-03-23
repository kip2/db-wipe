use clap::Parser;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::{env, error::Error, fs};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Config {
    #[arg(help = "Input file path")]
    file: String,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    Ok(config)
}

pub async fn run(config: Config) -> MyResult<()> {
    // Read environment variables and generate URL
    dotenv::dotenv().expect("Fialed to read .env file");
    let database_url = env::var("DATABASE_URL").expect("DABASE_URL must be set");

    // Generate DB connection
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .unwrap_or_else(|_| panic!("Cannot connect to the database"));

    Ok(())
}
