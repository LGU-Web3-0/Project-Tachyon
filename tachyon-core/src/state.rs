use crate::configs::Configs;
use crate::utils::Result;
use actix_web::cookie::Key;
use anyhow::anyhow;
use entity::sea_orm::{Database, DatabaseConnection};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, Tokio1Executor};

pub struct State {
    pub sql_db: DatabaseConnection,
    pub kv_db: sled::Db,
    pub key: Key,
    pub admin_name: String,
    pub lettre: Option<(String, AsyncSmtpTransport<Tokio1Executor>)>,
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
            lettre: configs.smtp.as_ref().map(|x| {
                (
                    x.userinfo.clone(),
                    AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(x.host.as_str())
                        .unwrap()
                        .port(x.port)
                        .credentials(Credentials::new(x.username.clone(), x.password.clone()))
                        .build(),
                )
            }),
        })
    }

    #[cfg(all(not(miri), test, feature = "integration-test"))]
    pub async fn mocked(uuid: uuid::Uuid) -> Result<Self> {
        use migration::Migrator;
        use migration::MigratorTrait;

        let sql_db = Database::connect(crate::test::DB_ADDRESS).await?;
        Migrator::down(&sql_db, Some(999)).await?;
        Migrator::up(&sql_db, Some(999)).await?;
        let kv_db = sled::Config::new()
            .path(format!("/tmp/tachyon-mock-test-{}", uuid))
            .open()?;
        let key = Key::try_generate().ok_or_else(|| anyhow!("unable to generate cookie key"))?;
        Ok(State {
            sql_db,
            kv_db,
            key,
            admin_name: "Schrodinger ZHU".to_string(),
            lettre: None,
        })
    }
}
