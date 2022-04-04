use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000003_create_task_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(entity::task::Entity)
                    .col(
                        ColumnDef::new(entity::task::Column::Id)
                            .big_integer()
                            .auto_increment()
                            .not_null()
                            .unique_key()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(entity::task::Column::CreateDate)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entity::task::Column::DueDate)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entity::task::Column::FinishDate)
                            .timestamp()
                            .not_null(),
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
                    .table(entity::task::Entity)
                    .to_owned(),
            )
            .await
    }
}
