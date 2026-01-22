#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]

pub use sea_orm_migration::prelude::*;

mod m20240101_000001_create_users;
mod m20240101_000002_create_topics;
mod m20240101_000003_create_progress;
mod m20240101_000004_create_game_sessions;
mod m20240101_000005_create_achievements;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_create_users::Migration),
            Box::new(m20240101_000002_create_topics::Migration),
            Box::new(m20240101_000003_create_progress::Migration),
            Box::new(m20240101_000004_create_game_sessions::Migration),
            Box::new(m20240101_000005_create_achievements::Migration),
        ]
    }
}
