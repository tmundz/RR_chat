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
