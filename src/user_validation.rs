use crate::types::*;
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use chrono::Utc;
use crate::run_usr_program::*;
use std::sync::Mutex;

#[post("/jobs")]
pub async fn post_jobs(body: web::Json<PostJob>, config: web::Data<OJConfig>,
                       jobs: web::Data<Mutex<Vec<TestJob>>>) -> impl Responder {
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
    let mut submission = body.into_inner();

    let res = match run("0", problem, &submission.source_code, &language) {
        Ok(res) => res,
        Err(_) => {
            return HttpResponse::InternalServerError().json(
                Error {
                    code: code,
                    reason: String::from(""),
                    message: String::from(""),
                }
            );
        }
    };

    let mut jobs = jobs.lock().unwrap();
    let id = jobs.len() as i32;
    let now = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
    let job = TestJob {
        id,
        created_time: now.clone(),
        updated_time: now,
        submission,
        state: "Finished".to_string(),
        result: res.result,
        score: res.score,
        cases: res.cases,
    };

    jobs.push(job);
    HttpResponse::Ok().json(jobs.last().unwrap())
    // run()
}

#[get("/jobs")]
pub async fn get_jobs(
    filter: web::Query<JobFilter>,
    jobs: web::Data<Mutex<Vec<TestJob>>>,
) -> impl Responder {
    let jobs = jobs.lock().unwrap();
    let filtered_jobs: Vec<&TestJob> = jobs
        .iter()
        .filter(|job| {
            filter
                .user_id
                .map_or(true, |user_id| job.submission.user_id == user_id)
                && filter.user_name.as_deref().map_or(true, |user_name| {
                    job.submission.user_id == 0 && user_name == "root"
                })
                && filter.contest_id.map_or(true, |contest_id| {
                    job.submission.contest_id == contest_id
                })
                && filter.problem_id.map_or(true, |problem_id| {
                    job.submission.problem_id == problem_id
                })
                && filter.language.as_deref().map_or(true, |language| {
                    job.submission.language == language
                })
                && filter.from.as_deref().map_or(true, |from| {
                    job.created_time.as_str() >= from
                })
                && filter
                    .to
                    .as_deref()
                    .map_or(true, |to| job.created_time.as_str() <= to)
                && filter
                    .state
                    .as_deref()
                    .map_or(true, |state| job.state == state)
                && filter
                    .result
                    .as_deref()
                    .map_or(true, |result| job.result == result)
        })
        .collect();

    HttpResponse::Ok().json(filtered_jobs)
}

// #[get("/jobs/jobId")]
// async fn get_job_id(name: web::Path<String>) -> impl Responder {}
//
// #[put("/jobs/jobId")]
// async fn put_job_id(name: web::Path<String>) -> impl Responder {}

// #[post("/jobs/jobId")]
// async fn post_job_id(name: web::Path<String>) -> impl Responder {}
//
//
// #[delete("/jobs/jobId")]
// async fn delete_job_id(name: web::Path<String>) -> impl Responder {}
