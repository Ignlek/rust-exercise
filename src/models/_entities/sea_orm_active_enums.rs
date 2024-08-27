//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "permission_type")]
pub enum PermissionType {
    #[sea_orm(string_value = "Read")]
    Read,
    #[sea_orm(string_value = "Write")]
    Write,
}
