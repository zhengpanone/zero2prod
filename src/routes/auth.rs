use actix_web::{post, web::Data, Responder};
use sqlx::{Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}
#[post("/user")]
async fn create_user(state: Data<AppState>) -> impl Responder {
    ""
}
#[post("/auth")]
async fn basic_auth(state: Data<AppState>) -> impl Responder {
    ""
}
#[post("/article")]
async fn create_article(state: Data<AppState>) -> impl Responder {
    ""
}
