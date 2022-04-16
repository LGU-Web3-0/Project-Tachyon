use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000007_create_project_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(entity::project::Entity)
                    .col(
                        ColumnDef::new(entity::project::Column::Id)
                            .big_integer()
                            .auto_increment()
                            .not_null()
                            .unique_key()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(entity::project::Column::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(entity::project::Column::Priority)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entity::project::Column::CategoryId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entity::project::Column::DueDate)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entity::project::Column::BeginDate)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entity::project::Column::FinishDate)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entity::project::Column::Description)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-category_id-categoryId")
                            .from(entity::project::Entity, entity::project::Column::CategoryId)
                            .to(entity::category::Entity, entity::category::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(entity::project::Entity)
                    .to_owned(),
            )
            .await
    }
}
