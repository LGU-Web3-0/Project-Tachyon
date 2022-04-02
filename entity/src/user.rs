use nanorand::Rng;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "user"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub pgp_key: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Name,
    Email,
    Password,
    PgpKey,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = Uuid;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def().unique().indexed(),
            Self::Name => ColumnType::String(None).def(),
            Self::Email => ColumnType::String(None).def().unique().indexed(),
            Self::Password => ColumnType::String(None).def(),
            Self::PgpKey => ColumnType::Binary.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn prepare<S0, S1, S2, S3>(
        name: S0,
        email: S1,
        password: S2,
        key: S3,
    ) -> anyhow::Result<ActiveModel>
    where
        S0: AsRef<str>,
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
    {
        let config: argon2::Config = argon2::Config::default();
        let mut salt = [0u8; 64];
        nanorand::tls_rng().fill_bytes(&mut salt);
        let password = argon2::hash_encoded(password.as_ref().as_bytes(), &salt, &config)?;
        let mut reader = sequoia_openpgp::armor::Reader::new(key.as_ref().as_bytes(), None);
        let mut pgp_key = Vec::with_capacity(1024);
        reader
            .read_to_end(&mut pgp_key)
            .map_err(Into::into)
            .map(|x| {
                log::debug!("read {} bytes from user armored value", x);
                ActiveModel {
                    id: ActiveValue::NotSet,
                    name: ActiveValue::Set(name.as_ref().to_owned()),
                    email: ActiveValue::Set(email.as_ref().to_owned()),
                    password: ActiveValue::Set(password),
                    pgp_key: ActiveValue::Set(pgp_key),
                }
            })
    }
}

#[cfg(any(test, feature = "test"))]
mod test {
    pub const KEY_BLOCK: &str = r#"-----BEGIN PGP PUBLIC KEY BLOCK-----
Comment: User-ID:	SchrodingerZhu <i@zhuyi.fan>
Comment: Created:	4/12/20 6:30 PM
Comment: Type:	4,096-bit RSA (secret key available)
Comment: Usage:	Signing, Encryption, Certifying User-IDs, SSH Authentication
Comment: Fingerprint:	313241EF9EAE2A855A08477F6552350D8D7EE5FB

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
=npsE
-----END PGP PUBLIC KEY BLOCK-----"#;

    #[cfg(not(miri))]
    #[test]
    fn it_creates_user() {
        use super::*;
        Entity::prepare("schrodinger", "i@zhuyi.fan", "123456", KEY_BLOCK).unwrap();
    }
}

#[cfg(feature = "test")]
pub use test::KEY_BLOCK;
