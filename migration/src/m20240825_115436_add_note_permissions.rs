use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the enum type directly using raw SQL command
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TYPE permission_type AS ENUM ('Read', 'Write');"
            )
            .await?;

        // Create the note_permissions table
        manager
            .create_table(
                Table::create()
                    .table(NotePermissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(NotePermissions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(NotePermissions::NoteId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(NotePermissions::UserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(NotePermissions::PermissionType)
                            .enumeration(
                                NotePermissions::PermissionType,
                                [
                                    NotePermissionsPermissionType::Read,
                                    NotePermissionsPermissionType::Write,
                                ],
                            )
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
            )
            .await?;

        // Add the owner_user_id column to the notes table
        manager
            .alter_table(
                Table::alter()
                    .table(Notes::Table)
                    .add_column(
                        ColumnDef::new(Notes::OwnerUserId)
                            .integer()
                            .null(), // Nullable as per the earlier discussion
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the note_permissions table
        manager
            .drop_table(Table::drop().table(NotePermissions::Table).to_owned())
            .await?;

        // Drop the owner_user_id column from the notes table
        manager
            .alter_table(
                Table::alter()
                    .table(Notes::Table)
                    .drop_column(Notes::OwnerUserId)
                    .to_owned(),
            )
            .await?;

        // Drop the enum type using raw SQL command
        manager
            .get_connection()
            .execute_unprepared(
                "DROP TYPE permission_type;"
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum NotePermissions {
    Table,
    Id,
    NoteId,
    UserId,
    PermissionType,
}

#[derive(Iden)]
enum Notes {
    Table,
    Id,
    OwnerUserId, // This represents the new column in the notes table
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}

#[derive(Iden)]
enum NotePermissionsPermissionType {
    Read,
    Write,
}
