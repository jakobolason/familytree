use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Medlem::Table)
            .if_not_exists()
            .col(
                big_integer(Medlem::Id)
                    .auto_increment()
                    .primary_key()
                    .take(),
            )
            .col(uuid(Medlem::Pid).unique_key())
            // Core info
            .col(string(Medlem::Name))
            .col(string_uniq(Medlem::Email).unique_key())
            .col(string(Medlem::PhoneNr).null())
            .col(string(Medlem::Address).null())
            .col(string(Medlem::City).null())
            .col(date(Medlem::Birthdate).null())
            .col(string(Medlem::Status).default("Alive"))
            .col(uuid(Medlem::UserPid).null())
            // store [uuid, uuid]
            .col(json_binary(Medlem::ParentsPid).null())
            .col(json_binary(Medlem::PreviousPartners).null())
            .col(json_binary(Medlem::ChildrenPid).null())
            .col(uuid(Medlem::PartnerPid).null())
            .col(date_time(Medlem::CreatedAt).default(Expr::current_timestamp()))
            .col(date_time(Medlem::UpdatedAt).default(Expr::current_timestamp()))
            .to_owned();
        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Medlem::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Medlem {
    Table,
    CreatedAt,
    UpdatedAt,
    Id,
    Pid,
    Email,
    Name,
    PhoneNr,
    Address,
    City,
    // Stateful, can change
    Birthdate,
    Status, // Alive, Dead, Cutoff ...
    // PID mapping
    UserPid,          // The user associated with this medlem
    ParentsPid,       // (parent1, parent2)
    PartnerPid,       // Should be manageable, unlike the above
    PreviousPartners, //
    ChildrenPid,      //i
}
