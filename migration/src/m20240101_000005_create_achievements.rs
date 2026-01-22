use sea_orm_migration::{prelude::*, schema::*};

use super::m20240101_000001_create_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Achievement definitions table
        manager
            .create_table(
                table_auto(Achievements::Table)
                    .col(pk_auto(Achievements::Id))
                    .col(string(Achievements::Type).unique_key())
                    .col(string(Achievements::NameId))
                    .col(string(Achievements::NameAr))
                    .col(string(Achievements::NameEn))
                    .col(text(Achievements::DescriptionId))
                    .col(text(Achievements::DescriptionAr))
                    .col(text(Achievements::DescriptionEn))
                    .col(string(Achievements::Icon))
                    .col(integer(Achievements::XpReward).default(0))
                    .to_owned(),
            )
            .await?;

        // User achievements (many-to-many)
        manager
            .create_table(
                table_auto(UserAchievements::Table)
                    .col(pk_auto(UserAchievements::Id))
                    .col(integer(UserAchievements::UserId))
                    .col(integer(UserAchievements::AchievementId))
                    .col(timestamp(UserAchievements::UnlockedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_achievements_user")
                            .from(UserAchievements::Table, UserAchievements::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_achievements_achievement")
                            .from(UserAchievements::Table, UserAchievements::AchievementId)
                            .to(Achievements::Table, Achievements::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_achievements_unique")
                    .table(UserAchievements::Table)
                    .col(UserAchievements::UserId)
                    .col(UserAchievements::AchievementId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserAchievements::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Achievements::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Achievements {
    Table,
    Id,
    Type,
    NameId,
    NameAr,
    NameEn,
    DescriptionId,
    DescriptionAr,
    DescriptionEn,
    Icon,
    XpReward,
}

#[derive(DeriveIden)]
pub enum UserAchievements {
    Table,
    Id,
    UserId,
    AchievementId,
    UnlockedAt,
}
