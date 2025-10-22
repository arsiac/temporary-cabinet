pub use sea_orm_migration::prelude::*;

mod m20251022_001_create_table_cabinet;
mod m20251022_002_create_table_cabinet_item;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251022_001_create_table_cabinet::Migration),
            Box::new(m20251022_002_create_table_cabinet_item::Migration),
        ]
    }
}
