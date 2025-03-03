mod creature_core_db_init;

use dotenvy::dotenv;
use sqlx::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;
use std::env;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = &env::var("DATABASE_URL")
        .expect("DB URL IS NOT SET.. Aborting. Hint: set DATABASE_URL environmental variable");

    let conn = SqlitePool::connect_with(
        SqliteConnectOptions::from_str(db_url)
            .expect("Could not find a valid db in the given path")
            .create_if_missing(true),
    )
    .await
    .expect("Could not connect to the given db url, something went wrong..");
    creature_core_db_init::create_creature_core_table(&conn)
        .await
        .expect("Could not initialize tables inside the db, something went wrong..");
    creature_core_db_init::initialize_data(&conn)
        .await
        .expect("Could not populate the db, something went wrong..");
    creature_core_db_init::cleanup_db(&conn)
        .await
        .expect("Could not clean up the db. Dirty state detected, closing..");
}
