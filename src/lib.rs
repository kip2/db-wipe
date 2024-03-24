use clap::Parser;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{database, MySql, Pool};
use std::io::{self, Write};
use std::process::Command;
use std::{env, error::Error};

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

    // Create dump file
    if check_dump(config) {
        if create_dump(&user_name, &password, &db_name).is_ok() {
            clear_database(&pool, db_name).await;
        } else {
            println!("Failed to create dump, not deleting database");
        };
    } else {
        clear_database(&pool, db_name).await;
    }

    Ok(())
}

fn prompt_yes_no(question: &str) -> bool {
    print!("{}", question);
    io::stdout().flush().expect("Failed to flush stdout");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().to_lowercase().chars().next() {
            Some('y') => return true,
            Some('n') => return false,
            _ => println!("Please enter 'y' or 'n'."),
        }
    }
}

fn check_dump(config: Config) -> bool {
    if !config.dump {
        let question = "Should we also perform a dump? [y/n]: ";
        prompt_yes_no(question)
    } else {
        config.dump
    }
}

fn create_dump(user_name: &str, password: &str, db_name: &str) -> MyResult<()> {
    let command = "mysqldump";
    let output_file = format!("{}.bk.sql", db_name);
    let password_arg = format!("-p{}", password);

    let output = Command::new(command)
        .arg("-u")
        .arg(user_name)
        .arg(&password_arg)
        .arg(db_name)
        .output()
        .expect("Failed to start mysqldump");
    std::fs::write(output_file, output.stdout).expect("Failed to write to file");

    Ok(())
}

async fn clear_database(db: &Pool<MySql>, db_name: String) {
    let mut tx = db.begin().await.expect("transaction error.");

    let mut queries: Vec<String> = Vec::new();
    queries.push(format!("DROP DATABASE {};", db_name));
    queries.push(format!("CREATE DATABASE {};", db_name));

    // Execute SQL query
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
    } // transaction commit

    let _ = tx.commit().await.unwrap_or_else(|e| {
        println!("{:?}", e);
    });
}
