mod api;
mod models;
mod repository;

use actix_web::{web::Data, App, HttpServer};
use api::parents_api::{create_parent, delete_parent, get_parent, update_parent};
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_parent)
            .service(get_parent)
            .service(update_parent)
            .service(delete_parent)
    })
    .bind(("localhost", 8000))?
    .run()
    .await
}
