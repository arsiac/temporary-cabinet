use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Keypair::Table)
                    .if_not_exists()
                    .col(pk_uuid(Keypair::Id))
                    .col(string_len(Keypair::SecretKey, 1000))
                    .col(string_len(Keypair::PublicKey, 1000))
                    .col(date_time(Keypair::ExpireAt))
                    .col(date_time(Keypair::CreateAt))
                    .col(date_time(Keypair::UpdateAt))
                    .col(integer(Keypair::Version))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Keypair::Table)
                    .name("idx-keypair-pk")
                    .col(Keypair::PublicKey)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Keypair::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Keypair {
    Table,
    Id,
    SecretKey,
    PublicKey,
    ExpireAt,
    CreateAt,
    UpdateAt,
    Version,
}
