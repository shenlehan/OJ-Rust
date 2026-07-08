use actix_web::{get, web, Responder, post, put, delete};

#[post("/jobs")]
async fn post_jobs(name: web::Path<String>) -> impl Responder {

}

#[get("/jobs")]
async fn get_jobs(name: web::Path<String>) -> impl Responder {

}

#[post("/jobs/jobId")]
async fn post_jobId(name: web::Path<String>) -> impl Responder {

}

#[get("/jobs/jobId")]
async fn get_jobId(name: web::Path<String>) -> impl Responder {

}

#[put("/jobs/jobId")]
async fn put_jobId(name: web::Path<String>) -> impl Responder {

}

#[delete("/jobs/jobId")]
async fn delete_jobId(name: web::Path<String>) -> impl Responder {

}