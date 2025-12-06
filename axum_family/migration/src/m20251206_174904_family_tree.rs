use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(FamilyTree::Table)
            .if_not_exists()
            .col(
                big_integer(FamilyTree::Id)
                    .auto_increment()
                    .primary_key()
                    .take(),
            )
            .col(json_binary(FamilyTree::TreeData).null())
            .col(
                date_time(FamilyTree::UpdatedAt)
                    .default(Expr::current_timestamp())
                    .null(),
            )
            .to_owned();
        m.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(FamilyTree::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum FamilyTree {
    Table,
    Id,
    TreeData,
    UpdatedAt,
}
