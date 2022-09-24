use dotenv::dotenv;
use std::env;

use crate::models::{parent::Parent, student::Student, teacher::Teacher};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

pub struct MongoRepo {
    teacher_col: Collection<Teacher>,
    parent_col: Collection<Parent>,
    student_col: Collection<Student>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("student_manager");
        let teacher_col: Collection<Teacher> = db.collection("Teacher");
        let parent_col: Collection<Parent> = db.collection("Parent");
        let student_col: Collection<Student> = db.collection("Parent");
        MongoRepo {
            teacher_col,
            parent_col,
            student_col,
        }
    }

    pub async fn create_teacher(&self, new_teacher: Teacher) -> Result<InsertOneResult, Error> {
        let new_doc = Teacher {
            id: None,
            email: new_teacher.email,
            password: new_teacher.password,
            fname: new_teacher.fname,
            lname: new_teacher.lname,
            dob: new_teacher.dob,
            phone: new_teacher.phone,
            mobile: new_teacher.mobile,
            status: new_teacher.status,
            last_login_date: new_teacher.last_login_date,
            last_login_ip: new_teacher.last_login_ip,
        };

        let teacher = self
            .teacher_col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating teacher");

        Ok(teacher)
    }

    pub async fn create_student(&self, new_student: Student) -> Result<InsertOneResult, Error> {
        let new_doc = Student {
            id: None,
            email: new_student.email,
            password: new_student.password,
            fname: new_student.fname,
            lname: new_student.lname,
            dob: new_student.dob,
            phone: new_student.phone,
            mobile: new_student.mobile,
            status: new_student.status,
            parent: new_student.parent,
            date_of_join: new_student.date_of_join,
            last_login_date: new_student.last_login_date,
            last_login_ip: new_student.last_login_ip,
        };

        let student = self
            .student_col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating student");

        Ok(student)
    }

    pub async fn create_parent(&self, new_parent: Parent) -> Result<InsertOneResult, Error> {
        let new_doc = Parent {
            id: None,
            email: new_parent.email,
            password: new_parent.password,
            fname: new_parent.fname,
            lname: new_parent.lname,
            dob: new_parent.dob,
            phone: new_parent.phone,
            mobile: new_parent.mobile,
            status: new_parent.status,
            last_login_date: new_parent.last_login_date,
            last_login_ip: new_parent.last_login_ip,
        };

        let parent = self
            .parent_col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating parent");

        Ok(parent)
    }

    pub async fn get_teacher(&self, id: &String) -> Result<Teacher, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let teacher_detail = self
            .teacher_col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting teacher's detail");
        Ok(teacher_detail.unwrap())
    }

    pub async fn get_parent(&self, id: &String) -> Result<Parent, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let parent_detail = self
            .parent_col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting parent's detail");
        Ok(parent_detail.unwrap())
    }

    pub async fn get_student(&self, id: &String) -> Result<Student, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let student_detail = self
            .student_col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting parent's detail");
        Ok(student_detail.unwrap())
    }

    pub async fn update_teacher(
        &self,
        id: &String,
        new_teacher: Teacher,
    ) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id":obj_id};
        let new_doc = doc! {
            "$set":
            {
                "id": new_teacher.id,
                "email": new_teacher.email,
                "password": new_teacher.password,
                "fname": new_teacher.fname,
                "lname": new_teacher.lname,
                "dob": new_teacher.dob,
                "phone": new_teacher.phone,
                "mobile": new_teacher.mobile,
                "status": new_teacher.status,
                "last_login_date": new_teacher.last_login_date,
                "last_login_ip": new_teacher.last_login_ip,
            },
        };

        let updated_doc = self
            .teacher_col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating teacher");
        Ok(updated_doc)
    }

    pub async fn update_parent(
        &self,
        id: &String,
        new_parent: Parent,
    ) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id":obj_id};
        let new_doc = doc! {
            "$set":
            {
                "id": new_parent.id,
                "email": new_parent.email,
                "password": new_parent.password,
                "fname": new_parent.fname,
                "lname": new_parent.lname,
                "dob": new_parent.dob,
                "phone": new_parent.phone,
                "mobile": new_parent.mobile,
                "status": new_parent.status,
                "last_login_date": new_parent.last_login_date,
                "last_login_ip": new_parent.last_login_ip,
            },
        };

        let updated_doc = self
            .parent_col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating parent");
        Ok(updated_doc)
    }

    pub async fn update_student(
        &self,
        id: &String,
        new_student: Student,
    ) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id":obj_id};
        let new_doc = doc! {
            "$set":
            {
                "id": new_student.id,
                "email": new_student.email,
                "password": new_student.password,
                "fname": new_student.fname,
                "lname": new_student.lname,
                "dob": new_student.dob,
                "phone": new_student.phone,
                "mobile": new_student.mobile,
                "parent": new_student.parent.id,
                "date_of_join": new_student.date_of_join,
                "status": new_student.status,
                "last_login_date": new_student.last_login_date,
                "last_login_ip": new_student.last_login_ip,
            },
        };

        let updated_doc = self
            .student_col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating parent");
        Ok(updated_doc)
    }

    pub async fn delete_teacher(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let teacher_detail = self
            .teacher_col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting teacher");

        Ok(teacher_detail)
    }

    pub async fn delete_parent(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let parent_detail = self
            .parent_col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting parent");

        Ok(parent_detail)
    }

    pub async fn delete_student(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let student_detail = self
            .student_col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting student");

        Ok(student_detail)
    }

    pub async fn get_all_teachers(&self) -> Result<Vec<Teacher>, Error> {
        let mut cursors = self
            .teacher_col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of teacher");

        let mut teachers: Vec<Teacher> = Vec::new();
        while let Some(teacher) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            teachers.push(teacher)
        }
        Ok(teachers)
    }

    pub async fn get_all_parents(&self) -> Result<Vec<Parent>, Error> {
        let mut cursors = self
            .parent_col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of parent");

        let mut parents: Vec<Parent> = Vec::new();
        while let Some(parent) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            parents.push(parent)
        }
        Ok(parents)
    }

    pub async fn get_all_students(&self) -> Result<Vec<Student>, Error> {
        let mut cursors = self
            .student_col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of student");

        let mut students: Vec<Student> = Vec::new();
        while let Some(student) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            students.push(student)
        }
        Ok(students)
    }
}
