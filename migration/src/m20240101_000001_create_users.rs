use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Users::Table)
                    .col(pk_auto(Users::Id))
                    .col(uuid(Users::Pid))
                    .col(string_null(Users::Email))
                    .col(string(Users::Name))
                    .col(string_null(Users::Avatar))
                    .col(integer(Users::CurrentLevel).default(1))
                    .col(integer(Users::TotalXp).default(0))
                    .col(string(Users::PreferredLanguage).default("id"))
                    .col(integer(Users::Streak).default(0))
                    .col(timestamp_null(Users::LastActiveAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_users_email")
                    .table(Users::Table)
                    .col(Users::Email)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_users_pid")
                    .table(Users::Table)
                    .col(Users::Pid)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Pid,
    Email,
    Name,
    Avatar,
    CurrentLevel,
    TotalXp,
    PreferredLanguage,
    Streak,
    LastActiveAt,
}
