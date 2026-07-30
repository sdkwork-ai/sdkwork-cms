#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = sdkwork_database_config::DatabaseConfig::from_env("CMS")?;
    let schema = std::env::var("SDKWORK_DATABASE_SCHEMA")?;

    match sdkwork_database_sqlx::create_pool_from_config(config).await {
        Ok(pool) => {
            println!("Connected to the workspace database successfully.");

            let tables: Vec<(String,)> = sqlx::query_as(
                "SELECT table_name FROM information_schema.tables \
                 WHERE table_schema = $1 ORDER BY table_name",
            )
            .bind(&schema)
            .fetch_all(pool.as_postgres().expect("PostgreSQL pool"))
            .await?;

            println!("\nExisting tables in schema {schema}:");
            for (table_name,) in &tables {
                println!("  - {table_name}");
            }

            if tables.is_empty() {
                println!("  (no tables found)");
            }
        }
        Err(error) => {
            println!("Failed to connect: {error}");
            println!("Configure the canonical SDKWORK_DATABASE_* PostgreSQL profile.");
        }
    }

    Ok(())
}
