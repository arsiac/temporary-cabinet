use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cabinet::Table)
                    .if_not_exists()
                    .col(big_integer(Cabinet::Code).primary_key())
                    .col(string_len_null(Cabinet::Name, 40))
                    .col(string_len_null(Cabinet::Description, 400))
                    .col(string_len_null(Cabinet::Password, 100))
                    .col(integer(Cabinet::Status))
                    .col(string_len_null(Cabinet::HoldToken, 40))
                    .col(date_time_null(Cabinet::ExpireAt))
                    .col(date_time(Cabinet::CreateAt))
                    .col(date_time(Cabinet::UpdateAt))
                    .col(integer(Cabinet::Version))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Cabinet::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Cabinet {
    Table,
    Code,
    Name,
    Description,
    Password,
    Status,
    HoldToken,
    ExpireAt,
    CreateAt,
    UpdateAt,
    Version,
}
