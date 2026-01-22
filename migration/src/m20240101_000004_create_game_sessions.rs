use sea_orm_migration::{prelude::*, schema::*};

use super::m20240101_000001_create_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(GameSessions::Table)
                    .col(pk_auto(GameSessions::Id))
                    .col(integer(GameSessions::UserId))
                    .col(string(GameSessions::GameType)) // word-builder, sentence-doctor, etc.
                    .col(string_null(GameSessions::TopicId))
                    .col(integer(GameSessions::Score))
                    .col(integer(GameSessions::MaxScore))
                    .col(integer(GameSessions::Duration)) // in seconds
                    .col(json(GameSessions::Mistakes)) // JSON array of mistake objects
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_sessions_user")
                            .from(GameSessions::Table, GameSessions::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_game_sessions_user")
                    .table(GameSessions::Table)
                    .col(GameSessions::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_game_sessions_game_type")
                    .table(GameSessions::Table)
                    .col(GameSessions::GameType)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GameSessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum GameSessions {
    Table,
    Id,
    UserId,
    GameType,
    TopicId,
    Score,
    MaxScore,
    Duration,
    Mistakes,
}
