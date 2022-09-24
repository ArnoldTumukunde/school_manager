use crate::{models::student::Student, repository::mongodb_repo::MongoRepo};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};

#[post("/student")]
pub async fn create_student(db: Data<MongoRepo>, new_student: Json<Student>) -> HttpResponse {
    let data = Student {
        id: None,
        email: new_student.email.to_string(),
        password: new_student.password.to_string(),
        fname: new_student.fname.to_string(),
        lname: new_student.lname.to_string(),
        dob: new_student.dob.to_owned(),
        phone: new_student.phone.to_string(),
        mobile: new_student.mobile.to_string(),
        status: new_student.status.to_owned(),
        parent: new_student.parent.to_owned(),
        date_of_join: new_student.date_of_join.to_owned(),
        last_login_date: new_student.last_login_date.to_owned(),
        last_login_ip: new_student.last_login_ip.to_string(),
    };
    let new_student = db.create_student(data).await;
    match new_student {
        Ok(student) => HttpResponse::Ok().json(student),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/student/{id}")]
pub async fn get_student(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let new_student = db.get_student(&id).await;
    match new_student {
        Ok(student) => HttpResponse::Ok().json(student),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/student/{id}")]
pub async fn update_student(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_student: Json<Student>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let data = Student {
        id: None,
        email: new_student.email.to_string(),
        password: new_student.password.to_string(),
        fname: new_student.fname.to_string(),
        lname: new_student.lname.to_string(),
        dob: new_student.dob.to_owned(),
        phone: new_student.phone.to_string(),
        mobile: new_student.mobile.to_string(),
        parent: new_student.parent.to_owned(),
        date_of_join: new_student.date_of_join.to_owned(),
        status: new_student.status.to_owned(),
        last_login_date: new_student.last_login_date.to_owned(),
        last_login_ip: new_student.last_login_ip.to_string(),
    };
    let update_result = db.update_student(&id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_student_info = db.get_student(&id).await;
                return match updated_student_info {
                    Ok(student) => HttpResponse::Ok().json(student),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No student found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/student/{id}")]
pub async fn delete_student(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_student(&id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Student successfully deleted");
            } else {
                return HttpResponse::NotFound().json("Student with specified ID not found");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/students")]
pub async fn get_all_students(db: Data<MongoRepo>) -> HttpResponse {
    let students = db.get_all_students().await;
    match students {
        Ok(student) => HttpResponse::Ok().json(student),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
