use crate::sea_orm::{DbBackend, Statement, StatementBuilder};
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_user_table"
    }
}

struct EmailConstraint;

impl StatementBuilder for EmailConstraint {
    fn build(&self, db_backend: &DbBackend) -> Statement {
        match db_backend {
            DbBackend::Postgres => {
                const STMT: &str = r#"ALTER TABLE "user" ADD CONSTRAINT proper_email CHECK (email ~* '^[A-Za-z0-9._+%-]+@[A-Za-z0-9.-]+[.][A-Za-z]+$');"#;
                Statement::from_string(DbBackend::Postgres, STMT.to_string())
            }
            _ => panic!("db other than PG is not supported"),
        }
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(entity::user::Entity)
                    .col(
                        ColumnDef::new(entity::user::Column::Id)
                            .big_integer()
                            .auto_increment()
                            .not_null()
                            .unique_key()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(entity::user::Column::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entity::user::Column::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(entity::user::Column::Password)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entity::user::Column::PgpKey)
                            .binary()
                            .not_null(),
                    )
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                sea_query::Index::create()
                    .table(entity::user::Entity)
                    .col(entity::user::Column::Id)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                sea_query::Index::create()
                    .table(entity::user::Entity)
                    .col(entity::user::Column::Email)
                    .to_owned(),
            )
            .await?;

        manager.exec_stmt(EmailConstraint).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(entity::user::Entity)
                    .to_owned(),
            )
            .await
    }
}
