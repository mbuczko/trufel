use crate::vault::Vault;

use rust_embed::RustEmbed;
use semver::Version;
use sqlx::Executor;
use sqlx::{postgres::PgRow, Row};
use std::fmt;
use std::str;

#[derive(RustEmbed)]
#[folder = "resources/db/migrations/"]
struct Asset;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Migration {
    file: String,
    version: String,
    description: String,
}

impl fmt::Display for Migration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} => {} ({})",
            self.file, self.version, self.description
        )
    }
}

impl Migration {
    pub fn new(file: String, version: String, description: String) -> Self {
        Migration {
            file,
            version,
            description,
        }
    }
}

impl Vault {
    pub async fn version(&self) -> anyhow::Result<(String, Version)> {
        sqlx::query("SELECT version, app_semver FROM migrations ORDER BY version DESC LIMIT 1")
            .try_map(|row: PgRow| {
                Ok((
                    row.try_get::<String, _>(0).unwrap(),
                    Version::parse(row.try_get(1).unwrap()).unwrap(),
                ))
            })
            .fetch_one(&self.pool)
            .await
            .map_err(Into::into)
    }
}

fn build_migration(base_version: String, app_semver: Version) -> Option<String> {
    let mut migrations: Vec<Migration> = Asset::iter()
        .map(|file| {
            let v: Vec<&str> = file.split("__").collect();
            Migration::new(
                file.to_string(),
                v[0].to_string(),
                v[1].trim_end_matches(".sql").replace("_", " "),
            )
        })
        .collect();

    // sort migrations by versions first
    migrations.sort_by(|m1, m2| m1.version.cmp(&m2.version));

    // keep only those which haven't been applied yet
    migrations.retain(|m| base_version.is_empty() || m.version.gt(&base_version));

    // ...and compose final transaction
    let final_txn = migrations.iter().fold(String::default(), |mut txn, m| {
        let buf = Asset::get(m.file.as_ref()).unwrap();
        match str::from_utf8(buf.data.as_ref()) {
            Ok(s) => {
                txn.push_str(s);
                txn.push_str(
                    format!(
                        "\n\n\
                         INSERT INTO migrations(version, description, script, app_semver) \
                         VALUES ('{version}', '{description}', '{script}', '{semver}');\n\n",
                        version = m.version,
                        description = m.description,
                        script = m.file,
                        semver = app_semver
                    )
                    .as_str(),
                );
                txn
            }
            _ => panic!("Non UTF8 format of migration file!"),
        }
    });
    if final_txn.is_empty() {
        None
    } else {
        Some(format!("BEGIN TRANSACTION;\n\n{}\n\nCOMMIT;", final_txn))
    }
}

pub async fn upgrade(
    vault: &Vault,
    base_script_version: String,
    app_semver: Version,
) -> anyhow::Result<()> {
    if let Some(m) = build_migration(base_script_version, app_semver) {
        let mut conn = vault.pool.acquire().await?;
        match conn.execute(m.as_str()).await {
            Ok(_) => tracing::debug!("Upgraded to {}", vault.version().await?.0),
            Err(e) => panic!("Couldn't update the database. Bailing out: {:?}", e),
        }
    }
    Ok(())
}
