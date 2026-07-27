use crate::types::*;

use actix_web::{HttpResponse, Responder, delete, get, post, put, web};

#[post("/jobs")]
pub async fn post_jobs(body: web::Json<PostJob>, config: web::Data<OJConfig>) -> impl Responder {
    let mut code = 0;

    let language = config.languages.iter().find(|lang| lang.name == body.language);
    let problem = config.problems.iter().find(|problem| problem.id == body.problem_id);

    if language.is_none() || problem.is_none() || body.user_id != 0 || body.contest_id != 0 {
        code = 1;
        let reason = "ERR_INVALID_ARGUMENT";
        let message = "HTTP 400 Bad Request";
        return HttpResponse::BadRequest().json(Error {
            code: code,
            reason: String::from(reason),
            message: String::from(message),
        });
    }

    let language = language.unwrap();
    let problem = problem.unwrap();

    HttpResponse::Ok().json(TestJob::default())
}

// #[get("/jobs")]
// async fn get_jobs(name: web::Path<String>) -> impl Responder {}
//
// #[post("/jobs/jobId")]
// async fn post_job_id(name: web::Path<String>) -> impl Responder {}
//
// #[get("/jobs/jobId")]
// async fn get_job_id(name: web::Path<String>) -> impl Responder {}
//
// #[put("/jobs/jobId")]
// async fn put_job_id(name: web::Path<String>) -> impl Responder {}
//
// #[delete("/jobs/jobId")]
// async fn delete_job_id(name: web::Path<String>) -> impl Responder {}
