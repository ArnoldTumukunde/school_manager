use crate::{models::parent::Parent, repository::mongodb_repo::MongoRepo};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};

#[post("/parent")]
pub async fn create_parent(db: Data<MongoRepo>, new_parent: Json<Parent>) -> HttpResponse {
    let data = Parent {
        id: None,
        email: new_parent.email.to_string(),
        password: new_parent.password.to_string(),
        fname: new_parent.fname.to_string(),
        lname: new_parent.lname.to_string(),
        dob: new_parent.dob.to_owned(),
        phone: new_parent.phone.to_string(),
        mobile: new_parent.mobile.to_string(),
        status: new_parent.status.to_owned(),
        last_login_date: new_parent.last_login_date.to_owned(),
        last_login_ip: new_parent.last_login_ip.to_string(),
    };
    let parent_detail = db.create_parent(data).await;
    match parent_detail {
        Ok(parent) => HttpResponse::Ok().json(parent),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/parent/{id}")]
pub async fn get_parent(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let parent_detail = db.get_parent(&id).await;
    match parent_detail {
        Ok(parent) => HttpResponse::Ok().json(parent),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/parent/{id}")]
pub async fn update_parent(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_parent: Json<Parent>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let data = Parent {
        id: None,
        email: new_parent.email.to_string(),
        password: new_parent.password.to_string(),
        fname: new_parent.fname.to_string(),
        lname: new_parent.lname.to_string(),
        dob: new_parent.dob.to_owned(),
        phone: new_parent.phone.to_string(),
        mobile: new_parent.mobile.to_string(),
        status: new_parent.status.to_owned(),
        last_login_date: new_parent.last_login_date.to_owned(),
        last_login_ip: new_parent.last_login_ip.to_string(),
    };
    let update_result = db.update_parent(&id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_parent_info = db.get_parent(&id).await;
                return match updated_parent_info {
                    Ok(parent) => HttpResponse::Ok().json(parent),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No parent found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/parent/{id}")]
pub async fn delete_parent(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_parent(&id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Parent successfully deleted");
            } else {
                return HttpResponse::NotFound().json("Parent with specified ID not found");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/parents")]
pub async fn get_all_parents(db: Data<MongoRepo>) -> HttpResponse {
    let parents = db.get_all_parents().await;
    match parents {
        Ok(parent) => HttpResponse::Ok().json(parent),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
