use anyhow::anyhow;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "task_user_assignment"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub task_id: i64,
    pub user_id: i64,
    pub assign_date: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    TaskId,
    UserId,
    AssignDate,
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
            Self::TaskId => ColumnType::BigInteger.def(),
            Self::UserId => ColumnType::BigInteger.def(),
            Self::AssignDate => ColumnType::TimestampWithTimeZone.def(),
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
    pub fn prepare(
        task_id: i64,
        user_id: i64,
        assign_date: DateTimeUtc,
    ) -> anyhow::Result<ActiveModel> {
        Ok(ActiveModel {
            id: ActiveValue::NotSet,
            task_id: ActiveValue::Set(task_id),
            user_id: ActiveValue::Set(user_id),
            assign_date: ActiveValue::Set(assign_date),
        })
    }
}
