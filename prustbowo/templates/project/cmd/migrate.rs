use anyhow::Result;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("db/migrations");

fn establish_connection() -> Result<PgConnection> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Ok(PgConnection::establish(&database_url)?)
}

fn main() -> Result<()> {
    let mut conn = establish_connection()?;
    conn.run_pending_migrations(MIGRATIONS)?;
    println!("migrations applied");
    Ok(())
}