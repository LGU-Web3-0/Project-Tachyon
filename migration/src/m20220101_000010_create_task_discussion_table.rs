use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000010_create_task_discussion_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(entity::task_discussion::Entity)
                    .col(
                        ColumnDef::new(entity::task_discussion::Column::Id)
                            .big_integer()
                            .auto_increment()
                            .not_null()
                            .unique_key()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(entity::task_discussion::Column::TaskId)
                            .big_integer()
                            .not_null()
                            .indexed(),
                    )
                    .col(
                        ColumnDef::new(entity::task_discussion::Column::UpdateTime)
                            .date_time_utc()
                            .not_null()
                            .indexed(),
                    )
                    .col(
                        ColumnDef::new(entity::task_discussion::Column::UserId)
                            .big_integer()
                            .not_null()
                            .indexed(),
                    )
                    .col(
                        ColumnDef::new(entity::task_discussion::Column::Content)
                            .text()
                            .not_null()
                            .indexed(),
                    ),
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-task_id-taskId")
                            .from(
                                entity::task_discussion::Entity,
                                entity::task_discussion::Column::TaskId,
                            )
                            .to(entity::task::Entity, entity::task::Column::Id)
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
                    .table(entity::task_discussion::Entity)
                    .to_owned(),
            )
            .await
    }
}
