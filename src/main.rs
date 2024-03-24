#[tokio::main]
async fn main() {
    // Retrieve arguments
    let config = match db_wipe::get_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    // Execute
    if let Err(e) = db_wipe::run(config).await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
