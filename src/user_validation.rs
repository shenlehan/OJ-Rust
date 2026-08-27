use crate::types::*;
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use chrono::Utc;
use crate::run_usr_program::*;
use std::cmp::Ordering;
use std::sync::Mutex;

#[post("/jobs")]
pub async fn post_jobs(body: web::Json<PostJob>, config: web::Data<OJConfig>,
                       jobs: web::Data<Mutex<Vec<TestJob>>>,
                       users: web::Data<Mutex<Vec<User>>>) -> impl Responder {
    let mut code = 0;

    let user_exists = users
        .lock()
        .unwrap()
        .iter()
        .any(|user| user.id == body.user_id);

    if !user_exists {
        return HttpResponse::NotFound().json(Error {
            code: 3,
            reason: "ERR_NOT_FOUND".to_string(),
            message: format!("User {} not found.", body.user_id),
        });
    }

    let language = config.languages.iter().find(|lang| lang.name == body.language);
    let problem = config.problems.iter().find(|problem| problem.id == body.problem_id);

    if language.is_none() || problem.is_none() || body.contest_id != 0 {
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

#[get("/jobs/{job_id}")]
pub async fn get_job_id(
    job_id: web::Path<i32>,
    jobs: web::Data<Mutex<Vec<TestJob>>>
) -> impl Responder {
    let jobs = jobs.lock().unwrap();
    let job_id = job_id.into_inner();
    let result = jobs.iter().find(|job|
        {
            job.id == job_id
        }
    );

    match result {
        Some(job) => HttpResponse::Ok().json(job),
        None => HttpResponse::NotFound().json(Error {
            code: 3,
            reason: "ERR_NOT_FOUND".to_string(),
            message: format!("Job {} not found.", job_id),
        }),
    }
}

#[put("/jobs/{job_id}")]
pub async fn put_job_id(
    job_id: web::Path<i32>,
    jobs: web::Data<Mutex<Vec<TestJob>>>,
    config: web::Data<OJConfig>
) -> impl Responder {

    let job_id = job_id.into_inner();

    let submission = {
        let jobs = jobs.lock().unwrap();
        let job = match jobs.iter().find(|job| { job.id == job_id }) {
            Some(job) => job,
            None => {
                return HttpResponse::NotFound().json(Error {
                    code: 3,
                    reason: "ERR_NOT_FOUND".to_string(),
                    message: format!("Job {} not found.", job_id),
                })
            }
        };

        if job.state != "Finished" {
            return HttpResponse::BadRequest().json(Error {
                code: 2,
                reason: "ERR_INVALID_STATE".to_string(),
                message: format!("Job {} not finished.", job_id),
            });
        }

        job.submission.clone()
    };

    let language = config.languages.iter()
        .find(|language| language.name == submission.language)
        .unwrap();

    let problem = config.problems.iter()
        .find(|problem| problem.id == submission.problem_id)
        .unwrap();

    let res = match run("0", problem, &submission.source_code, language) {
        Ok(res) => res,
        Err(_) => {
            return HttpResponse::InternalServerError().json(Error {
                code: 0,
                reason: "".to_string(),
                message: "".to_string(),
            });
        }
    };

    let mut jobs = jobs.lock().unwrap();
    let job = jobs.iter_mut().find(|job| { job.id == job_id }).unwrap();
    job.updated_time = Utc::now()
        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
        .to_string();
    job.state = "Finished".to_string();
    job.result = res.result;
    job.score = res.score;
    job.cases = res.cases;
    HttpResponse::Ok().json(job)
}

#[get("/users")]
pub async fn get_users(
    users: web::Data<Mutex<Vec<User>>>
) -> impl Responder {
    let users = users.lock().unwrap();
    HttpResponse::Ok().json(&*users)
}

#[post("/users")]
pub async fn post_users(
    body: web::Json<PostUser>,
    users: web::Data<Mutex<Vec<User>>>
) -> impl Responder {
    let body = body.into_inner();
    let mut users = users.lock().unwrap();

    let duplicate = users
        .iter()
        .any(|user| user.name == body.name && Some(user.id) != body.id);

    if duplicate {
        return HttpResponse::BadRequest().json(Error {
            code: 1,
            reason: "ERR_INVALID_ARGUMENT".to_string(),
            message: format!("User name '{}' already exists.", body.name),
        });
    }

    match body.id {
        Some(id) => {
            let user = match users.iter_mut().find(|user| user.id == id) {
                Some(user) => user,
                None => {
                    return HttpResponse::NotFound().json(Error {
                        code: 3,
                        reason: "ERR_NOT_FOUND".to_string(),
                        message: format!("User {} not found.", id),
                    });
                }
            };

            user.name = body.name;
            HttpResponse::Ok().json(user)
        }
        None => {
            let id = users.len() as i32;
            users.push(User {
                id,
                name: body.name,
            });
            HttpResponse::Ok().json(users.last().unwrap())
        }
    }
}

struct RanklistRow {
    user: User,
    rank: i32,
    scores: Vec<f64>,
    total_score: f64,
    submission_count: usize,
    submission_time: Option<String>,
}

fn compare_submission_time(a: &Option<String>, b: &Option<String>) -> Ordering {
    match (a, b) {
        (Some(a), Some(b)) => a.cmp(b),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

fn same_rank(a: &RanklistRow, b: &RanklistRow, tie_breaker: Option<&str>) -> bool {
    if a.total_score != b.total_score {
        return false;
    }

    match tie_breaker {
        Some("user_id") => a.user.id == b.user.id,
        Some("submission_count") => a.submission_count == b.submission_count,
        Some("submission_time") => a.submission_time == b.submission_time,
        _ => true,
    }
}

#[get("/contests/{contest_id}/ranklist")]
pub async fn get_ranklist(
    contest_id: web::Path<i32>,
    query: web::Query<RanklistQuery>,
    config: web::Data<OJConfig>,
    jobs: web::Data<Mutex<Vec<TestJob>>>,
    users: web::Data<Mutex<Vec<User>>>,
) -> impl Responder {
    let contest_id = contest_id.into_inner();
    if contest_id != 0 {
        return HttpResponse::NotFound().json(Error {
            code: 3,
            reason: "ERR_NOT_FOUND".to_string(),
            message: format!("Contest {} not found.", contest_id),
        });
    }

    let scoring_rule = query.scoring_rule.as_deref().unwrap_or("latest");
    if !matches!(scoring_rule, "latest" | "highest") {
        return HttpResponse::BadRequest().json(Error {
            code: 1,
            reason: "ERR_INVALID_ARGUMENT".to_string(),
            message: "Invalid argument scoring_rule".to_string(),
        });
    }

    let tie_breaker = query.tie_breaker.as_deref();
    if !matches!(
        tie_breaker,
        None | Some("user_id" | "submission_count" | "submission_time")
    ) {
        return HttpResponse::BadRequest().json(Error {
            code: 1,
            reason: "ERR_INVALID_ARGUMENT".to_string(),
            message: "Invalid argument tie_breaker".to_string(),
        });
    }

    let mut problem_ids: Vec<i32> = config.problems.iter().map(|problem| problem.id).collect();
    problem_ids.sort_unstable();

    let users = users.lock().unwrap().clone();
    let jobs = jobs.lock().unwrap();
    let mut rows: Vec<RanklistRow> = users
        .into_iter()
        .map(|user| {
            let user_jobs: Vec<&TestJob> = jobs
                .iter()
                .filter(|job| {
                    job.submission.user_id == user.id
                        && job.submission.contest_id == contest_id
                })
                .collect();
            let submission_count = user_jobs.len();
            let mut submission_time: Option<String> = None;
            let mut scores = Vec::with_capacity(problem_ids.len());

            for problem_id in &problem_ids {
                let problem_jobs: Vec<&TestJob> = user_jobs
                    .iter()
                    .copied()
                    .filter(|job| job.submission.problem_id == *problem_id)
                    .collect();

                let selected = if scoring_rule == "highest" {
                    problem_jobs.into_iter().max_by(|a, b| {
                        a.score
                            .partial_cmp(&b.score)
                            .unwrap_or(Ordering::Equal)
                            .then_with(|| b.created_time.cmp(&a.created_time))
                    })
                } else {
                    problem_jobs
                        .into_iter()
                        .max_by(|a, b| a.created_time.cmp(&b.created_time))
                };

                if let Some(job) = selected {
                    scores.push(job.score);
                    if submission_time
                        .as_ref()
                        .map_or(true, |time| job.created_time.as_str() > time.as_str())
                    {
                        submission_time = Some(job.created_time.clone());
                    }
                } else {
                    scores.push(0.0);
                }
            }

            let total_score = scores.iter().sum();
            RanklistRow {
                user,
                rank: 0,
                scores,
                total_score,
                submission_count,
                submission_time,
            }
        })
        .collect();

    rows.sort_by(|a, b| {
        let score_order = b
            .total_score
            .partial_cmp(&a.total_score)
            .unwrap_or(Ordering::Equal);
        if score_order != Ordering::Equal {
            return score_order;
        }

        let tie_order = match tie_breaker {
            Some("user_id") => a.user.id.cmp(&b.user.id),
            Some("submission_count") => a.submission_count.cmp(&b.submission_count),
            Some("submission_time") => {
                compare_submission_time(&a.submission_time, &b.submission_time)
            }
            _ => Ordering::Equal,
        };
        if tie_order != Ordering::Equal {
            return tie_order;
        }

        a.user.id.cmp(&b.user.id)
    });

    for index in 0..rows.len() {
        let rank = if index == 0 {
            1
        } else if same_rank(&rows[index - 1], &rows[index], tie_breaker) {
            rows[index - 1].rank
        } else {
            (index + 1) as i32
        };
        rows[index].rank = rank;
    }

    let ranklist: Vec<RanklistEntry> = rows
        .into_iter()
        .map(|row| RanklistEntry {
            user: row.user,
            rank: row.rank,
            scores: row.scores,
        })
        .collect();

    HttpResponse::Ok().json(ranklist)
}

// #[post("/jobs/jobId")]
// async fn post_job_id(name: web::Path<String>) -> impl Responder {}
//
//
// #[delete("/jobs/jobId")]
// async fn delete_job_id(name: web::Path<String>) -> impl Responder {}
