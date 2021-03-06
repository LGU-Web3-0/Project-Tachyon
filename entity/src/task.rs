use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "task"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Deserialize, Serialize)]
pub struct Model {
    pub id: i64,
    pub name: String,
    pub create_date: DateTimeUtc,
    pub due_date: DateTimeUtc,
    pub finish_date: Option<DateTimeUtc>,
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Name,
    CreateDate,
    DueDate,
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
            Self::CreateDate => ColumnType::TimestampWithTimeZone.def(),
            Self::DueDate => ColumnType::TimestampWithTimeZone.def(),
            Self::FinishDate => ColumnType::TimestampWithTimeZone.def(),
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

impl Model {
    pub fn prepare<S0, S1>(
        name: S0,
        create_date: &DateTimeUtc,
        due_date: &DateTimeUtc,
        description: S1,
    ) -> anyhow::Result<ActiveModel>
    where
        S0: AsRef<str>,
        S1: AsRef<str>,
    {
        Ok(ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(name.as_ref().to_owned()),
            create_date: ActiveValue::Set(*create_date),
            due_date: ActiveValue::Set(*due_date),
            finish_date: ActiveValue::NotSet,
            description: ActiveValue::Set(description.as_ref().to_owned()),
        })
    }
}
