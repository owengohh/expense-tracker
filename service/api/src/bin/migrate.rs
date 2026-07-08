use std::{
    env, fs,
    path::{Path, PathBuf},
};

use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use tokio_postgres::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();

    let database_url = env::var("MIGRATION_DATABASE_URL")
        .or_else(|_| env::var("DATABASE_URL"))
        .map_err(|_| "MIGRATION_DATABASE_URL or DATABASE_URL must be set")?;
    let migrations_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("migrations");
    let migration_paths = ordered_migration_paths(&migrations_dir)?;

    let connector = TlsConnector::builder().build()?;
    let tls = MakeTlsConnector::new(connector);
    let (mut client, connection) = tokio_postgres::connect(&database_url, tls).await?;

    tokio::spawn(async move {
        if let Err(error) = connection.await {
            eprintln!("database connection error: {error}");
        }
    });
    ensure_migration_table(&client).await?;

    let mut applied_count = 0usize;
    for migration_path in migration_paths {
        let file_name = migration_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or("invalid migration file name")?;

        if migration_already_applied(&client, file_name).await? {
            println!("Skipping {file_name} (already applied).");
            continue;
        }

        let sql = fs::read_to_string(&migration_path)?;
        let transaction = client.transaction().await?;
        transaction.batch_execute(&sql).await?;
        transaction
            .execute(
                "insert into schema_migrations (filename) values ($1)",
                &[&file_name],
            )
            .await?;
        transaction.commit().await?;

        applied_count += 1;
        println!("Applied {} successfully.", migration_path.display());
    }

    if applied_count == 0 {
        println!("No pending migrations.");
    }

    Ok(())
}

fn ordered_migration_paths(
    migrations_dir: &Path,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error + Send + Sync>> {
    let mut paths = fs::read_dir(migrations_dir)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()?;

    paths.retain(|path| path.extension().and_then(|ext| ext.to_str()) == Some("sql"));
    paths.sort();

    Ok(paths)
}

async fn ensure_migration_table(
    client: &Client,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    client
        .batch_execute(
            "
            create table if not exists schema_migrations (
              filename text primary key,
              applied_at timestamptz not null default now()
            );
            ",
        )
        .await?;

    Ok(())
}

async fn migration_already_applied(
    client: &Client,
    file_name: &str,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let row = client
        .query_opt(
            "select 1 from schema_migrations where filename = $1",
            &[&file_name],
        )
        .await?;

    Ok(row.is_some())
}
