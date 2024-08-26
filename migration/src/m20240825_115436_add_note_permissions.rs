use loco_rs::schema::table_auto_tz;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the Enum type
        manager.create_enum(
            Enum::create()
                .name("permission_type")
                .values(vec!["Read", "Write"]) // Add your permission types here
                .to_owned(),
        ).await?;

        // Create NotePermissions Table with Enum column
        manager.create_table(
            Table::create()
                .table(NotePermissions::Table)
                .if_not_exists()
                .col(ColumnDef::new(NotePermissions::NoteId).integer().not_null())
                .col(ColumnDef::new(NotePermissions::UserId).integer().not_null())
                .col(
                    ColumnDef::new(NotePermissions::PermissionType)
                        .enumeration("permission_type", ["Read", "Write"])
                        .not_null(),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-note")
                        .from(NotePermissions::Table, NotePermissions::NoteId)
                        .to(Notes::Table, Notes::Id),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-user")
                        .from(NotePermissions::Table, NotePermissions::UserId)
                        .to(Users::Table, Users::Id),
                )
                .to_owned(),
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop NotePermissions Table
        manager.drop_table(Table::drop().table(NotePermissions::Table).to_owned()).await?;

        // Drop the Enum type
        manager.drop_enum(Enum::drop().name("permission_type").to_owned()).await?;

        Ok(())
    }
}

#[derive(Iden)]
enum NotePermissions {
    Table,
    NoteId,
    UserId,
    PermissionType,
}

#[derive(Iden)]
enum Notes {
    Table,
    Id,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}
