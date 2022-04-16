extern crate core;

pub use sea_schema::migration::prelude::*;

mod m20220101_000001_create_object_table;
mod m20220101_000002_create_user_table;
mod m20220101_000003_create_task_table;
mod m20220101_000004_add_user_wrong_pass_attempt;
mod m20220101_000005_add_user_tsvector;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_object_table::Migration),
            Box::new(m20220101_000002_create_user_table::Migration),
            Box::new(m20220101_000003_create_task_table::Migration),
            Box::new(m20220101_000004_add_user_wrong_pass_attempt::Migration),
            Box::new(m20220101_000005_add_user_tsvector::Migration),
        ]
    }
}
