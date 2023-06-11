pub use sea_orm_migration::prelude::*;

mod m20230422_085708_create_fridge;
mod m20230422_085918_create_product;
mod m20230422_085923_create_fridge_product_join;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230422_085708_create_fridge::Migration),
            Box::new(m20230422_085918_create_product::Migration),
            Box::new(m20230422_085923_create_fridge_product_join::Migration),
        ]
    }
}
