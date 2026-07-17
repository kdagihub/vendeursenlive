use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};
use tracing::info;

const MIGRATIONS: &[(&str, &str)] = &[
    (
        "20260709170000_create_core_live_commerce_tables",
        include_str!("../../../migrations/20260709170000_create_core_live_commerce_tables/up.sql"),
    ),
    (
        "20260709234000_add_password_reset_tokens",
        include_str!("../../../migrations/20260709234000_add_password_reset_tokens/up.sql"),
    ),
    (
        "20260710193000_add_email_verification_tokens",
        include_str!("../../../migrations/20260710193000_add_email_verification_tokens/up.sql"),
    ),
];

pub async fn run_pending_migrations(db: &DatabaseConnection) -> anyhow::Result<()> {
    ensure_migration_table(db).await?;

    let backend = db.get_database_backend();

    for (version, sql) in MIGRATIONS {
        if is_migration_applied(db, version).await? {
            continue;
        }

        for statement in split_sql_statements(sql) {
            db.execute(Statement::from_string(backend, statement))
                .await?;
        }

        let insert_sql = format!(
            "INSERT INTO schema_migrations (version) VALUES ('{}')",
            escape_sql_literal(version)
        );
        db.execute(Statement::from_string(backend, insert_sql))
            .await?;

        info!(version, "database migration applied");
    }

    Ok(())
}

async fn ensure_migration_table(db: &DatabaseConnection) -> anyhow::Result<()> {
    let backend = db.get_database_backend();
    db.execute(Statement::from_string(
        backend,
        r#"
        CREATE TABLE IF NOT EXISTS schema_migrations (
            version VARCHAR(255) PRIMARY KEY,
            applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    ))
    .await?;

    Ok(())
}

async fn is_migration_applied(db: &DatabaseConnection, version: &str) -> anyhow::Result<bool> {
    let backend = db.get_database_backend();
    let sql = format!(
        "SELECT version FROM schema_migrations WHERE version = '{}'",
        escape_sql_literal(version)
    );

    Ok(db
        .query_one(Statement::from_string(backend, sql))
        .await?
        .is_some())
}

fn split_sql_statements(sql: &str) -> Vec<String> {
    sql.split(';')
        .filter_map(|statement| {
            let statement = statement.trim();
            (!statement.is_empty()).then(|| format!("{statement};"))
        })
        .collect()
}

fn escape_sql_literal(value: &str) -> String {
    value.replace('\'', "''")
}
