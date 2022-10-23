use crate::db::init_pool;
use crate::db::migrations::upgrade;

use semver::Version;
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct Vault {
    pub pool: PgPool,
}

impl Vault {
    pub async fn new(db: &str) -> Self {
        match init_pool(db).await {
            Ok(pool) => Vault { pool },
            _ => panic!("Cannot open connection to database"),
        }
    }
}

pub async fn init_vault(db: &str, app_semver: Version) -> anyhow::Result<Vault> {
    tracing::debug!("Opening database ({})", db);

    let vault = Vault::new(db).await;
    let (last_script_version, last_app_version) = match vault.version().await {
        Ok((lsv, lav)) => (lsv, lav),
        Err(_) => (String::default(), Version::parse("0.0.0").unwrap()),
    };
    match last_app_version {
        v if last_app_version < app_semver => {
            tracing::debug!(
                "Upgrading DB schema from {} to latest version: {}",
                v,
                app_semver
            );
            upgrade(&vault, last_script_version, app_semver).await?
        }
        v if last_app_version > app_semver => panic!(
            "Your app version {} is too old, minimal required version is: {}",
            app_semver, v
        ),
        v => {
            tracing::debug!("DB schema already at newest version: {}", v);
        }
    }
    Ok(vault)
}
