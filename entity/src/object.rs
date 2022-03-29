use super::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "object")]
pub struct Model {
    #[sea_orm(primary_key, indexed, unique)]
    pub uuid: Uuid,
    #[sea_orm(indexed, unique)]
    pub name: String,
    pub mimetype: String,
    pub upload_time: DateTimeUtc,
    pub visibility: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
