#![allow(
    elided_lifetimes_in_paths,
    clippy::wildcard_imports,
    clippy::enum_variant_names
)]
pub use sea_orm_migration::prelude::*;

mod m20250101_000001_user;
mod m20250101_000002_seed_users;
mod m20251204_184527_medlem;
mod m20251206_174904_family_tree;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250101_000001_user::Migration),
            Box::new(m20250101_000002_seed_users::Migration),
            Box::new(m20251204_184527_medlem::Migration),
            Box::new(m20251206_174904_family_tree::Migration),
        ]
    }
}

