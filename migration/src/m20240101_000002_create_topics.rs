use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Topics::Table)
                    .col(pk_auto(Topics::Id))
                    .col(string(Topics::Category)) // nahwu, shorof
                    .col(integer(Topics::Order))
                    .col(string(Topics::Slug).unique_key())
                    .col(string(Topics::TitleId))
                    .col(string(Topics::TitleAr))
                    .col(string(Topics::TitleEn))
                    .col(text(Topics::DescriptionId))
                    .col(text(Topics::DescriptionAr))
                    .col(text(Topics::DescriptionEn))
                    .col(integer(Topics::RequiredLevel).default(1))
                    .col(boolean(Topics::IsActive).default(true))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_topics_category")
                    .table(Topics::Table)
                    .col(Topics::Category)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_topics_order")
                    .table(Topics::Table)
                    .col(Topics::Order)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Topics::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Topics {
    Table,
    Id,
    Category,
    Order,
    Slug,
    TitleId,
    TitleAr,
    TitleEn,
    DescriptionId,
    DescriptionAr,
    DescriptionEn,
    RequiredLevel,
    IsActive,
}
