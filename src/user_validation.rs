use crate::configs::*;
use crate::types::*;
use crate::utilities::*;

use actix_web::{HttpResponse, Responder, delete, get, post, put, web};

#[post("/jobs")]
async fn post_jobs(body: web::Json<PostJob>) -> impl Responder {
    let mut has_error = false;
    let mut code = 0;
    let mut reason: &str;
    let mut message: &str;
    if !elem_in_arr(&body.language.make_ascii_lowercase(), LANGUAGES)
        || !elem_in_arr(&body.problem_id, PROBLEMS)
        || !elem_in_arr(&body.user_id, USERS)
        || !elem_in_arr(&body.contest_id, CONTESTS)
    {
        has_error = true;
        code = 1;
        reason = "ERR_INVALID_ARGUMENT";
        message = "HTTP 400 Bad Request";
    }

    if has_error {
        return HttpResponse::BadRequest().json(Error {
            code: code,
            reason: String::from(reason),
            message: String::from(message),
        });
    }


    
}

#[get("/jobs")]
async fn get_jobs(name: web::Path<String>) -> impl Responder {}

#[post("/jobs/jobId")]
async fn post_job_id(name: web::Path<String>) -> impl Responder {}

#[get("/jobs/jobId")]
async fn get_job_id(name: web::Path<String>) -> impl Responder {}

#[put("/jobs/jobId")]
async fn put_job_id(name: web::Path<String>) -> impl Responder {}

#[delete("/jobs/jobId")]
async fn delete_job_id(name: web::Path<String>) -> impl Responder {}
