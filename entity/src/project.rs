use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "project"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub name: String,
    pub priority: String,
    pub category_id: i64,
    pub due_date: DateTimeUtc,
    pub begin_date: DateTimeUtc,
    pub finish_date: DateTimeUtc,
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Name,
    Priority,
    CategoryId,
    DueDate,
    BeginDate,
    FinishDate,
    Description,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i64;
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
            Self::Priority => ColumnType::String(None).def(),
            Self::CategoryId => ColumnType::BigInteger.def(),
            Self::DueDate => ColumnType::Timestamp.def(),
            Self::BeginDate => ColumnType::Timestamp.def(),
            Self::FinishDate => ColumnType::Timestamp.def(),
            Self::Description => ColumnType::String(None).def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
