use std::time::Instant;
use actix::*;
use actix_files::NamedFile;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use serde_json::json;
use uuid::Uuid;
use crate::db;
use crate::models;
use crate::server;
use crate::session;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

//entry point
pub async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap();
}

pub async fn chat_server(
    req: HttpRequest,
    Stream: web::Payload,
    pool: web::Data<DbPool>,
    srv: web::Data<Addr<server::chat_server>>,
) -> Result<Httpsponse, Error> {
    ws::start(
        Session::WsChatSession {
            id: 0,
            hb: instant::now(),
            addr: srv.get_ref().clone(),
            db_pool: pool,
        },
        &req,
        Stream
    )
}

#[post("/users/create")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::insert_new_user(&mut conn, &form.username, &form.phone)
    }).await?.map_err(actix_web::error::ErrorUnprocessableEntity)?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("/users/{user_id}")]
pub async fn get_user_by_id(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_id = id.to_owned();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::find_user_by_uid(&mut conn, user_id)
    }).await?.map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("No user found with phone {id}")
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("/converstions/{uid}")]
pub async fn get_conversation_by_id(
    pool: web::Data<DbPool>,
    uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let room_id = uid.to_owned();
    let conversations = web::block(move || {
        let mut conn = pool.get()?;
        db::get_conversation_by_room_uid(&mut conn, room_id)
    }).await?.map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(data) = conversations {
        Ok(HttpResponse::Ok().json(data))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("No conversation with room {room_id}")
            })
            .to_string(),
        );
    }
}







