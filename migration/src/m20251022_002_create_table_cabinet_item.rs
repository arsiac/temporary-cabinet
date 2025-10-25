use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CabinetItem::Table)
                    .if_not_exists()
                    .col(big_integer(CabinetItem::Id).primary_key())
                    .col(big_integer(CabinetItem::CabinetCode))
                    .col(string_len(CabinetItem::Category, 20))
                    .col(string_len(CabinetItem::Name, 200))
                    .col(string_len(CabinetItem::Path, 1000))
                    .col(big_integer(CabinetItem::Size))
                    .col(integer(CabinetItem::SortOrder))
                    .col(date_time(CabinetItem::CreateAt))
                    .col(date_time(CabinetItem::UpdateAt))
                    .col(integer(CabinetItem::Version))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(CabinetItem::Table)
                    .name("idx-cabinet_item-cabinet_code-sort_order")
                    .col(CabinetItem::CabinetCode)
                    .col(CabinetItem::SortOrder)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CabinetItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CabinetItem {
    Table,
    Id,
    CabinetCode,
    Category,
    Name,
    Path,
    Size,
    SortOrder,
    CreateAt,
    UpdateAt,
    Version,
}
