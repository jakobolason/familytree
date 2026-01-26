use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(MedlemEditors::Table)
            .if_not_exists()
            .col(pk_auto(MedlemEditors::Id))
            .col(uuid(MedlemEditors::MedlemPid))
            .col(uuid(MedlemEditors::UserPid))
            .col(string(MedlemEditors::Role).default("viewer"))
            .index(
                Index::create()
                    .name("idx_medlem_user_unique") // name of constraint
                    .table(MedlemEditors::Table) // table to apply to
                    .col(MedlemEditors::MedlemPid) // col1
                    .col(MedlemEditors::UserPid) // col2
                    .unique(),
            )
            .to_owned();
        manager.create_table(table).await?;
        Ok(())
    }
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "medlem_editors").await
    }
}

#[derive(Iden)]
pub enum MedlemEditors {
    Table,
    Id,
    MedlemPid,
    UserPid,
    Role,
}
