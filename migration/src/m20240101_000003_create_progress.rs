use sea_orm_migration::{prelude::*, schema::*};

use super::m20240101_000001_create_users::Users;
use super::m20240101_000002_create_topics::Topics;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Progress::Table)
                    .col(pk_auto(Progress::Id))
                    .col(integer(Progress::UserId))
                    .col(integer(Progress::TopicId))
                    .col(integer(Progress::StarsEarned).default(0)) // 0, 1, 2, or 3
                    .col(integer(Progress::BestScore).default(0))
                    .col(integer(Progress::Attempts).default(0))
                    .col(timestamp_null(Progress::CompletedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_progress_user")
                            .from(Progress::Table, Progress::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_progress_topic")
                            .from(Progress::Table, Progress::TopicId)
                            .to(Topics::Table, Topics::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_progress_user_topic")
                    .table(Progress::Table)
                    .col(Progress::UserId)
                    .col(Progress::TopicId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Progress::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Progress {
    Table,
    Id,
    UserId,
    TopicId,
    StarsEarned,
    BestScore,
    Attempts,
    CompletedAt,
}
