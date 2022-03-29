use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_object_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(entity::object::Entity)
                    .col(
                        ColumnDef::new(entity::object::Column::Uuid)
                            .uuid()
                            .not_null()
                            .unique_key()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(entity::object::Column::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(entity::object::Column::Mimetype)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entity::object::Column::UploadTime)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entity::object::Column::Visibility)
                            .boolean()
                            .not_null(),
                    )
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                sea_query::Index::create()
                    .table(entity::object::Entity)
                    .col(entity::object::Column::Uuid)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                sea_query::Index::create()
                    .table(entity::object::Entity)
                    .col(entity::object::Column::Name)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(entity::object::Entity)
                    .to_owned(),
            )
            .await
    }
}
