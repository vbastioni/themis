use actix_web::web;

use actix_web::{delete, get, post, put, Responder};

#[post("")]
async fn create_doc() -> impl Responder {
    format!("Creating a new doc...")
}

#[get("/{uid}")]
async fn get_doc(uid: web::Path<String>) -> impl Responder {
    format!("Fetching {}.", &uid)
}

#[get("")]
async fn get_all_docs() -> impl Responder {
    format!("Fetching all docs.")
}

#[put("")]
async fn update_doc_no_uid() -> impl Responder {
    format!("Updating with given ID or create if no ID or non-existent ID.")
}

#[put("/{uid}")]
async fn update_doc(uid: web::Path<String>) -> impl Responder {
    format!("Updating {}.", &uid)
}

#[delete("/{uid}")]
async fn delete_doc(uid: web::Path<String>) -> impl Responder {
    format!("Deleting {}.", &uid)
}

pub fn scope() -> actix_web::Scope {
    web::scope("/doc")
        .service(create_doc)
        .service(get_doc)
        .service(get_all_docs)
        .service(update_doc_no_uid)
        .service(update_doc)
        .service(delete_doc)
}
