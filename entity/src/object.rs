use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "object"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub uuid: Uuid,
    pub name: String,
    pub mimetype: String,
    pub upload_time: DateTimeUtc,
    pub visibility: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Uuid,
    Name,
    Mimetype,
    UploadTime,
    Visibility,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Uuid,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = Uuid;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Uuid => ColumnType::Uuid.def().unique().indexed(),
            Self::Name => ColumnType::String(None).def().unique().indexed(),
            Self::Mimetype => ColumnType::String(None).def(),
            Self::UploadTime => ColumnType::Timestamp.def(),
            Self::Visibility => ColumnType::Boolean.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
