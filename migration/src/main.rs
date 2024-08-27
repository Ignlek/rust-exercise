use sea_orm_migration::prelude::*;
use dotenv::dotenv;
use std::env; 

#[async_std::main]
async fn main() {
    dotenv().ok(); // Load environment variables from `.env`


    // Manually set DATABASE_URL if not set
    if env::var("DATABASE_URL").is_err() {
        env::set_var("DATABASE_URL", "postgres://loco:loco@localhost:5432/loco_starter_development");
    }
 // Print current working directory to confirm it's correct
 if let Ok(path) = env::current_dir() {
    println!("Current working directory: {}", path.display());
} else {
    println!("Unable to determine current working directory");
}

// Debug print to check if DATABASE_URL is set
if let Ok(database_url) = std::env::var("DATABASE_URL") {
    println!("DATABASE_URL is set to: {}", database_url);
} else {
    println!("DATABASE_URL is not set");
}

    cli::run_cli(migration::Migrator).await;