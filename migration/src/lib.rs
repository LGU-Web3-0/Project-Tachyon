#![feature(box_syntax)]
extern crate core;

pub use sea_schema::migration::prelude::*;

mod m20220101_000001_create_object_table;
mod m20220101_000002_create_user_table;
mod m20220101_000003_create_task_table;
mod m20220101_000004_add_user_wrong_pass_attempt;
mod m20220101_000005_add_user_tsvector;
mod m20220101_000006_create_team_table;
mod m20220101_000007_create_team_member_table;
mod m20220101_000008_create_category_table;
mod m20220101_000009_create_project_table;
mod m20220101_000010_create_task_discussion_table;
mod m20220101_000011_create_task_user_assignment_table;
mod m20220101_000012_create_task_discussion_attachment_table;
mod m20220101_000013_add_task_description;
mod m20220101_000014_add_task_tsvector;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            box m20220101_000001_create_object_table::Migration,
            box m20220101_000002_create_user_table::Migration,
            box m20220101_000003_create_task_table::Migration,
            box m20220101_000004_add_user_wrong_pass_attempt::Migration,
            box m20220101_000005_add_user_tsvector::Migration,
            box m20220101_000006_create_team_table::Migration,
            box m20220101_000007_create_team_member_table::Migration,
            box m20220101_000008_create_category_table::Migration,
            box m20220101_000009_create_project_table::Migration,
            box m20220101_000010_create_task_discussion_table::Migration,
            box m20220101_000011_create_task_user_assignment_table::Migration,
            box m20220101_000012_create_task_discussion_attachment_table::Migration,
            box m20220101_000013_add_task_description::Migration,
            box m20220101_000014_add_task_tsvector::Migration,
        ]
    }
}
