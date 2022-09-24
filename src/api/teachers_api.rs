use crate::{models::teacher::Teacher, repository::mongodb_repo::MongoRepo};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};

#[post("/teacher")]
pub async fn create_teacher(db: Data<MongoRepo>, new_teacher: Json<Teacher>) -> HttpResponse {
    let data = Teacher {
        id: None,
        email: new_teacher.email.to_string(),
        password: new_teacher.password.to_string(),
        fname: new_teacher.fname.to_string(),
        lname: new_teacher.lname.to_string(),
        dob: new_teacher.dob.to_owned(),
        phone: new_teacher.phone.to_string(),
        mobile: new_teacher.mobile.to_string(),
        status: new_teacher.status.to_owned(),
        last_login_date: new_teacher.last_login_date.to_owned(),
        last_login_ip: new_teacher.last_login_ip.to_string(),
    };
    let teacher_detail = db.create_teacher(data).await;
    match teacher_detail {
        Ok(teacher) => HttpResponse::Ok().json(teacher),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/teacher/{id}")]
pub async fn get_parent(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let teacher_detail = db.get_teacher(&id).await;
    match teacher_detail {
        Ok(teacher) => HttpResponse::Ok().json(teacher),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/teacher/{id}")]
pub async fn update_teacher(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_teacher: Json<Teacher>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let data = Teacher {
        id: None,
        email: new_teacher.email.to_string(),
        password: new_teacher.password.to_string(),
        fname: new_teacher.fname.to_string(),
        lname: new_teacher.lname.to_string(),
        dob: new_teacher.dob.to_owned(),
        phone: new_teacher.phone.to_string(),
        mobile: new_teacher.mobile.to_string(),
        status: new_teacher.status.to_owned(),
        last_login_date: new_teacher.last_login_date.to_owned(),
        last_login_ip: new_teacher.last_login_ip.to_string(),
    };
    let update_result = db.update_teacher(&id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_teacher_info = db.get_teacher(&id).await;
                return match updated_teacher_info {
                    Ok(teacher) => HttpResponse::Ok().json(teacher),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No teacher found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/teacher/{id}")]
pub async fn delete_teacher(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_teacher(&id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Teacher successfully deleted");
            } else {
                return HttpResponse::NotFound().json("Teacher with specified ID not found");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/teachers")]
pub async fn get_all_teachers(db: Data<MongoRepo>) -> HttpResponse {
    let teachers = db.get_all_teachers().await;
    match teachers {
        Ok(teacher) => HttpResponse::Ok().json(teacher),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
