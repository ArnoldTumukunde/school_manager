mod api;
mod models;
mod repository;

use actix_web::{web::Data, App, HttpServer};
use api::{parents_api::*, students_api::*, teachers_api::*};
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
            .service(create_student)
            .service(get_student)
            .service(update_student)
            .service(delete_student)
            .service(create_teacher)
            .service(get_teacher)
            .service(update_teacher)
            .service(delete_teacher)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
