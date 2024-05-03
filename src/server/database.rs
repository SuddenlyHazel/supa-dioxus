use std::env;

use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;

static ONCE: OnceCell<DatabaseConnection> = OnceCell::const_new();

fn db_uri() -> String {
  env::var("DB_URI").unwrap()
}

pub async fn connection<'a>() -> &'a DatabaseConnection {
  ONCE.get_or_init(|| async {
    Database::connect(db_uri()).await.unwrap()
  }).await
}
