use crate::configs::Configs;
use crate::utils::Result;
use actix_web::cookie::Key;
use anyhow::anyhow;
use entity::sea_orm::{Database, DatabaseConnection};
use hashbrown::HashMap;

pub struct State {
    pub sql_db: DatabaseConnection,
    pub kv_db: sled::Db,
    pub key: Key,
    pub frontend: HashMap<&'static str, &'static str>,
}

impl State {
    pub async fn from_configs(configs: &Configs) -> Result<Self> {
        let sql_db = Database::connect(&configs.db_uri).await?;
        let kv_db = sled::Config::new().path(&configs.sled_dir).open()?;
        let key = Key::try_generate().ok_or_else(|| anyhow!("unable to generate cookie key"))?;
        let mut frontend = hashbrown::HashMap::new();
        for i in tachyon_frontend::TARGETS {
            frontend.insert(i.1, i.0);
        }
        Ok(State {
            sql_db,
            kv_db,
            key,
            frontend,
        })
    }

    #[cfg(all(not(miri), test, feature = "integration-test"))]
    pub fn mocked(uuid: uuid::Uuid) -> Result<Self> {
        use entity::sea_orm::{DatabaseBackend, MockDatabase};
        let sql_db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
        let kv_db = sled::Config::new()
            .path(format!("/tmp/tachyon-mock-test-{}", uuid))
            .open()?;
        let key = Key::try_generate().ok_or_else(|| anyhow!("unable to generate cookie key"))?;
        let mut frontend = hashbrown::HashMap::new();
        for i in tachyon_frontend::TARGETS {
            frontend.insert(i.1, i.0);
        }
        Ok(State {
            sql_db,
            kv_db,
            key,
            frontend,
        })
    }
}
