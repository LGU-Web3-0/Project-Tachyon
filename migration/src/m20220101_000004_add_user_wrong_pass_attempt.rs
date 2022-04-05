use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000004_add_user_wrong_pass_attempt"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(entity::user::Entity)
                    .add_column(
                        ColumnDef::new(entity::user::Column::WrongPassAttempt).big_integer(),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(entity::user::Entity)
                    .drop_column(entity::user::Column::WrongPassAttempt)
                    .to_owned(),
            )
            .await
    }
}
