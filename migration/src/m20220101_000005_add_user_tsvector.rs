use crate::sea_orm::{DbBackend, Statement, StatementBuilder};
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000005_add_user_tsvector"
    }
}

mod up {
    use super::*;

    pub struct AddTsVector;

    impl StatementBuilder for AddTsVector {
        fn build(&self, db_backend: &DbBackend) -> Statement {
            match db_backend {
                DbBackend::Postgres => {
                    const STMT: &str = r#"
                ALTER TABLE "user"
                ADD COLUMN user_search_vector tsvector
                GENERATED ALWAYS AS (to_tsvector('english', coalesce(name, '') || ' ' || coalesce(email, ''))) STORED;
                "#;
                    Statement::from_string(DbBackend::Postgres, STMT.to_string())
                }
                _ => panic!("db other than PG is not supported"),
            }
        }
    }

    pub struct AddTsIndex;

    impl StatementBuilder for AddTsIndex {
        fn build(&self, db_backend: &DbBackend) -> Statement {
            match db_backend {
                DbBackend::Postgres => {
                    const STMT: &str = r#"
                CREATE INDEX user_search_index ON "user" USING GIN (user_search_vector);
                "#;
                    Statement::from_string(DbBackend::Postgres, STMT.to_string())
                }
                _ => panic!("db other than PG is not supported"),
            }
        }
    }

    pub struct AddTsTrigger;

    impl StatementBuilder for AddTsTrigger {
        fn build(&self, db_backend: &DbBackend) -> Statement {
            match db_backend {
                DbBackend::Postgres => {
                    const STMT: &str = r#"
                CREATE TRIGGER update_user_search_vector
                    BEFORE INSERT OR UPDATE
                    ON "user"
                    FOR EACH ROW
                    EXECUTE PROCEDURE
                        tsvector_update_trigger(user_search_vector, 'pg_catelog.english', name, email);
                "#;
                    Statement::from_string(DbBackend::Postgres, STMT.to_string())
                }
                _ => panic!("db other than PG is not supported"),
            }
        }
    }
}

mod down {
    use super::*;

    pub struct DropTsVector;

    impl StatementBuilder for DropTsVector {
        fn build(&self, db_backend: &DbBackend) -> Statement {
            match db_backend {
                DbBackend::Postgres => {
                    const STMT: &str = r#"
                ALTER TABLE "user"
                DROP COLUMN IF EXISTS user_search_vector;
                "#;
                    Statement::from_string(DbBackend::Postgres, STMT.to_string())
                }
                _ => panic!("db other than PG is not supported"),
            }
        }
    }

    pub struct DropTsIndex;

    impl StatementBuilder for DropTsIndex {
        fn build(&self, db_backend: &DbBackend) -> Statement {
            match db_backend {
                DbBackend::Postgres => {
                    const STMT: &str = r#"
                DROP INDEX IF EXISTS user_search_index;
                "#;
                    Statement::from_string(DbBackend::Postgres, STMT.to_string())
                }
                _ => panic!("db other than PG is not supported"),
            }
        }
    }

    pub struct DropTsTrigger;

    impl StatementBuilder for DropTsTrigger {
        fn build(&self, db_backend: &DbBackend) -> Statement {
            match db_backend {
                DbBackend::Postgres => {
                    const STMT: &str = r#"
                DROP TRIGGER IF EXISTS update_user_search_vector ON "user";
                "#;
                    Statement::from_string(DbBackend::Postgres, STMT.to_string())
                }
                _ => panic!("db other than PG is not supported"),
            }
        }
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.exec_stmt(up::AddTsVector).await?;
        manager.exec_stmt(up::AddTsIndex).await?;
        manager.exec_stmt(up::AddTsTrigger).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.exec_stmt(down::DropTsTrigger).await?;
        manager.exec_stmt(down::DropTsIndex).await?;
        manager.exec_stmt(down::DropTsVector).await?;
        Ok(())
    }
}
