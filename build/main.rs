mod creature_core_db_init;

use dotenvy::dotenv;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::env;
use std::fmt::{Display, Formatter};
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
    init_creature_core(conn.clone(), &GameSystem::Pathfinder).await;
    init_creature_core(conn, &GameSystem::Starfinder).await;
}

async fn init_creature_core(conn: Pool<Sqlite>, gs: &GameSystem) {
    creature_core_db_init::create_creature_core_table(&conn, gs)
        .await
        .expect("Could not initialize tables inside the db, something went wrong..");
    creature_core_db_init::initialize_data(&conn, gs)
        .await
        .expect("Could not populate the db, something went wrong..");
    creature_core_db_init::cleanup_db(&conn, gs)
        .await
        .expect("Could not clean up the db. Dirty state detected, closing..");
}

#[derive(PartialEq, Eq)]
pub enum GameSystem {
    Pathfinder,
    Starfinder,
}

impl Display for GameSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pathfinder => "pf",
                Self::Starfinder => "sf",
            }
        )
    }
}
