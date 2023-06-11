use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FridgeProductJoin::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FridgeProductJoin::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(FridgeProductJoin::FridgeId)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(ForeignKey::create()
                        .from(FridgeProductJoin::Table, FridgeProductJoin::FridgeId)
                        .to(Fridge::Table, Fridge::Id)
                    )
                    .col(
                        ColumnDef::new(FridgeProductJoin::ProductId)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(ForeignKey::create()
                        .from(FridgeProductJoin::Table, FridgeProductJoin::ProductId)
                        .to(Product::Table, Product::Id)
                    )
                    .col(
                        ColumnDef::new(FridgeProductJoin::Expiry)
                            .date_time()
                    )
                    .col(
                        ColumnDef::new(FridgeProductJoin::PurchaseDate)
                            .date_time()
                    )
                    .to_owned(),
            )
            .await
        // manager.create_foreign_key(sea_query::ForeignKey::create())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FridgeProductJoin::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum FridgeProductJoin {
    Table,
    Id,
    FridgeId,
    ProductId,
    Expiry,
    PurchaseDate,
}

#[derive(Iden)]
enum Fridge {
    Table,
    Id,
}

#[derive(Iden)]
enum Product {
    Table,
    Id,
}