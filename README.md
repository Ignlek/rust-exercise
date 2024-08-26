Here is a cleaned-up and formatted version of your README file for GitLab:

---

# Project Setup and Overview

## Database Setup

To set up your PostgreSQL database, run the following Docker command:

```bash
docker run -d -p 5432:5432 -e POSTGRES_USER=x -e POSTGRES_DB=y -e POSTGRES_PASSWORD="z" postgres:15.3-alpine
```

Set the `DATABASE_URL` environment variable:

```bash
set DATABASE_URL=postgres://x:z@localhost:5432/y
```

## Quick Start

### Prerequisites

- A local PostgreSQL instance
- A local Redis instance

Check out your development [configuration](config/development.yaml).

### Database Configuration

To configure the database, please run a local PostgreSQL database with the user `loco` and a database named `[app name]_development`:

```bash
docker run -d -p 5432:5432 -e POSTGRES_USER=loco -e POSTGRES_DB=[app name]_development -e POSTGRES_PASSWORD="loco" postgres:15.3-alpine
```

## Design Decisions

### Ownership of Notes

I added an `OwnerUserId` column to the `Notes` table to track the owner of each note. However, since this system might already be in production, existing notes won't have an owner. I considered two solutions:

1. **Nullable `OwnerUserId`**: Make the `OwnerUserId` column nullable. This way, all previously created notes will not have an owner, and any user can see these notes.
2. **Assign to Admin**: Assign all existing notes to an Admin user.

I chose the first solution because it makes more sense for this project. However, in a real-world scenario, the decision would require further discussion.

### Permissions System

For the `NotePermissions` table, I added a `PermissionType` column as an enum, anticipating future extensions. There are two main approaches to handling enums in this project:

1. **Database-Level Enum**: Define an enum directly in the database.
2. **Rust Enum with Mapping**: Use a Rust enum and map it to a string in the database.

I chose the first option for this task, but in a real-world scenario, the second option might be preferable due to concerns about memory usage and flexibility.

To add an additional value to the enum, you can execute the following migration command:

```rust
manager.execute(
    Statement::from_string(
        manager.get_database_backend(),
        "ALTER TYPE permission_type ADD VALUE 'Edit';".to_owned(),
    ),
).await?;
```

## Permission Models

Different solutions for handling note permissions:

### Option 1: Basic Note Permissions (Individual Note-Level)

This is the simplest option, where permissions are granted on a per-note basis.

**Schema:**

- **Notes Table**: Contains the notes, each with an `owner_user_id`.
- **NotePermissions Table**:
  - `note_id`: References a specific note.
  - `user_id`: References a user who has been granted access.
  - `permission_type`: Enum for the type of permission (e.g., `Read`, `Write`).

**Use Case:** 
- **Scenario**: A user can grant access to specific notes individually to other users.
- **Pro**: Simple to implement and straightforward.
- **Con**: Not scalable if users need to manage permissions for many notes.

### Option 2: Note Lists with Grouped Permissions

This option introduces a `NoteLists` table, allowing users to group notes and then assign permissions to an entire list.

**Schema:**

- **Notes Table**: Contains the notes, each with an `owner_user_id`.
- **NoteLists Table**:
  - `id`: Unique identifier for the list.
  - `owner_user_id`: References the user who created the list.
  - `name`: Name of the list.
- **NoteListItems Table**:
  - `note_list_id`: References a list of notes.
  - `note_id`: References a note in the `Notes` table.
- **NoteListPermissions Table**:
  - `note_list_id`: References a list of notes.
  - `user_id`: References a user who has been granted access.
  - `permission_type`: Enum for the type of permission.

**Use Case:** 
- **Scenario**: A user can create lists of notes (e.g., "Project A Notes") and assign access to these lists, making it easier to manage permissions for multiple notes.
- **Pro**: More scalable and organized, especially for users with many notes.
- **Con**: More complex to implement and manage.

### Option 3: Role-Based Access Control (RBAC)

This approach uses roles to manage permissions more flexibly.

**Schema:**

- **Roles Table**:
  - `id`: Unique identifier for the role.
  - `name`: Name of the role (e.g., "Editor", "Viewer").
- **UserRoles Table**:
  - `user_id`: References a user.
  - `role_id`: References a role.
- **NotePermissions Table**:
  - `note_id`: References a note.
  - `role_id`: References a role.
  - `permission_type`: Enum for the type of permission.

**Use Case:** 
- **Scenario**: Instead of assigning permissions directly to users, you assign roles. Users can then be assigned roles, making it easier to manage large groups of users.
- **Pro**: Highly flexible and scalable, especially for larger systems with many users.
- **Con**: More complex to set up and manage, potentially overkill for smaller systems.

### Option 4: Hierarchical Permissions

This approach allows for a hierarchy of permissions where users can inherit permissions from other users or groups.

**Schema:**

- **Notes Table**: Contains the notes, each with an `owner_user_id`.
- **Permission Hierarchy Table**:
  - `id`: Unique identifier.
  - `parent_id`: Points to another entry in this table, representing a hierarchical relationship.
  - `user_id`: References a user.
  - `note_id`: References a note.
  - `permission_type`: Enum for the type of permission.

**Use Case:** 
- **Scenario**: Useful in organizations where permissions can be inherited down a hierarchy (e.g., from a team leader to team members).
- **Pro**: Highly flexible, supporting complex organizational structures.
- **Con**: Complex to implement and may introduce performance considerations due to hierarchy traversal.

### Conclusion

- **Option 1 (Basic Note Permissions)**: Simple and direct but may become cumbersome as the number of notes grows.
- **Option 2 (Note Lists)**: More organized and scalable, ideal for users with many notes.
- **Option 3 (RBAC)**: Flexible and scalable, suitable for larger systems with many users.
- **Option 4 (Hierarchical Permissions)**: Complex but powerful, ideal for large organizations with complex permission needs.

For this task, I chose **Option 1** for simplicity. In a real-world scenario, the choice would depend on the specific use case.

## Starting the Application

To start your application, use the following command:

```bash
cargo loco start
```
