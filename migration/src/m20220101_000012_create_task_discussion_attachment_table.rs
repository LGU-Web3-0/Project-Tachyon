use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000012_create_task_discussion_attachment_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(entity::task_discussion_attachment::Entity)
                    .col(
                        ColumnDef::new(entity::task_discussion_attachment::Column::Id)
                            .big_integer()
                            .auto_increment()
                            .not_null()
                            .unique_key()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(entity::task_discussion_attachment::Column::DiscussionId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entity::task_discussion_attachment::Column::ObjectUuid)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-discussion_id-discussionId")
                            .from(
                                entity::task_discussion_attachment::Entity,
                                entity::task_discussion_attachment::Column::DiscussionId,
                            )
                            .to(
                                entity::task_discussion::Entity,
                                entity::task_discussion::Column::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-object_uuid-objectUuid")
                            .from(
                                entity::task_discussion_attachment::Entity,
                                entity::task_discussion_attachment::Column::ObjectUuid,
                            )
                            .to(entity::object::Entity, entity::object::Column::Uuid)
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
                    .table(entity::task_discussion_attachment::Entity)
                    .to_owned(),
            )
            .await
    }
}
