#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use serde_json::to_string;
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::notes::{ActiveModel, Entity, Model, Column as NotesColumn};
use crate::models::_entities::note_permissions;
use crate::models::_entities::users::{Entity as Users, Column as UsersColumn};



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub title: Option<String>,
    pub content: Option<String>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.title = Set(self.title.clone());
        item.content = Set(self.content.clone());
    }
}

async fn load_item(ctx: &AppContext, auth: &auth::JWT, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;

    validate_note_ownership(&item.as_ref(), &auth)?;

    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>, 
    auth: auth::JWT
) -> Result<Response> {
    // Fetch notes that are either:
    // - Public (no owner_user_id)
    // - Owned by the authenticated user
    // - Shared with the authenticated user via note_permissions

    let public_notes = Entity::find()
        .filter(NotesColumn::OwnerUserId.is_null())
        .all(&ctx.db)
        .await?;

    let pid_str = &auth.claims.pid;
    println!("pid_str: {}", pid_str); 
    let user_id = get_user_id_from_pid(&ctx, &auth.claims.pid).await?;

    let owned_notes = Entity::find()
        .filter(NotesColumn::OwnerUserId.eq(user_id))
        .all(&ctx.db)
        .await?;
    
    let shared_notes = Entity::find()
        .inner_join(note_permissions::Entity)
        .filter(note_permissions::Column::UserId.eq(pid_str.parse::<i32>().map_err(|_| Error::string("BadRequest2"))?))
        .all(&ctx.db)
        .await?;

    // Combine all the notes into a single list
    let mut combined_notes = Vec::new();
    combined_notes.extend(public_notes);
    combined_notes.extend(owned_notes);
    combined_notes.extend(shared_notes);

    format::json(combined_notes)
}


async fn get_user_id_from_pid(ctx: &AppContext, pid: &str) -> Result<i32, Error> {
    let uuid_pid = Uuid::parse_str(pid).map_err(|_| Error::string("Invalid UUID format"))?;

    let user = Users::find()
        .filter(UsersColumn::Pid.eq(uuid_pid))
        .one(&ctx.db)
        .await?;

    if let Some(user) = user {
        Ok(user.id)
    } else {
        Err(Error::NotFound)
    }
}
fn claims_to_string(auth: &auth::JWT) -> String {
    to_string(&auth.claims).unwrap_or_else(|_| "Failed to serialize claims".to_string())
}
#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>, 
    auth: auth::JWT, // Get the authenticated user
    Json(params): Json<Params>
) -> Result<Response> {
    // Create a new note with the authenticated user's ID as the owner
    let pid_str = &auth.claims.pid;

    println!("{:?}", auth.claims);

    println!("pid_str: {}", pid_str);

    let claims_str = claims_to_string(&auth);
    Error::string(&claims_str);
    let user_id = get_user_id_from_pid(&ctx, &auth.claims.pid).await?;

    let mut item = ActiveModel {
        owner_user_id: Set(pid_str.parse::<i32>().map_err(|_| Error::string(&user_id.to_string()))?), // Set the owner_user_id
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}


#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    auth: auth::JWT,
    Json(params): Json<Params>,
    
) -> Result<Response> {
    let item = load_item(&ctx, &auth, id).await?;

    validate_note_ownership(&Some(&item), &auth)?;

    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn remove(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    auth: auth::JWT
    ) -> Result<Response> {
    let item = load_item(&ctx, &auth, id).await?;

    validate_note_ownership(&Some(&item), &auth)?;

    item.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, &auth, id).await?)
}

pub fn validate_note_ownership(item: &Option<&Model>, auth: &auth::JWT) -> Result<(), Error> {
    if let Some(note) = item {
        let owner_user_id = auth
            .claims
            .pid
            .parse::<i32>()
            .map_err(|_| Error::string("BadRequest4"))?;

        if Some(note.owner_user_id) != Some(owner_user_id) {
            return Err(Error::string("Forbidden"));
        }

        Ok(())
    } else {
        Err(Error::string("Not found"))
    }
}


pub fn routes() -> Routes {
    Routes::new()
        .prefix("notes")
        .add("/", get(list))
        .add("/", post(add))
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", post(update))
}
