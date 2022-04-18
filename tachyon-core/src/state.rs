use crate::configs::Configs;
use crate::utils::Result;
use actix_web::cookie::Key;
use anyhow::anyhow;
use entity::sea_orm::{Database, DatabaseConnection};

pub struct State {
    pub sql_db: DatabaseConnection,
    pub kv_db: sled::Db,
    pub key: Key,
    pub admin_name: String,
    pub sendgrid_key: Option<String>,
}

impl State {
    pub async fn from_configs(configs: &Configs) -> Result<Self> {
        let sql_db = Database::connect(&configs.db_uri).await?;
        let kv_db = sled::Config::new().path(&configs.sled_dir).open()?;

        let key = if let Some(key) = configs.fixed_key.as_ref() {
            Key::derive_from(key.as_bytes())
        } else {
            Key::try_generate().ok_or_else(|| anyhow!("unable to generate cookie key"))?
        };
        Ok(State {
            sql_db,
            kv_db,
            key,
            admin_name: configs.admin_name.clone(),
            sendgrid_key: configs.sendgrid_key.clone(),
        })
    }

    #[cfg(all(not(miri), test, feature = "integration-test"))]
    pub async fn mocked(uuid: uuid::Uuid) -> Result<Self> {
        use migration::Migrator;
        use migration::MigratorTrait;

        let sql_db = Database::connect(crate::test::DB_ADDRESS).await?;
        Migrator::down(&sql_db, None).await?;
        Migrator::up(&sql_db, None).await?;
        let kv_db = sled::Config::new()
            .path(format!("/tmp/tachyon-mock-test-{}", uuid))
            .open()?;
        let key = Key::try_generate().ok_or_else(|| anyhow!("unable to generate cookie key"))?;
        Ok(State {
            sql_db,
            kv_db,
            key,
            admin_name: "admin".to_string(),
        })
    }
}
