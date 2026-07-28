use chrono::Utc;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::iter::chain;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::thread::sleep;
use crate::types::*;
use std::path::PathBuf;
use crate::utilities::*;

pub fn run(usr_name: &str, problem: &Problem, source_code: &str) -> std::io::Result<JudgeResult> {
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
    let work_dir = PathBuf::from("/tmp/rust_oj")
        .join(usr_name)
        .join(&problem.name)
        .join(&timestamp);

    let mut judge_res = JudgeResult {
        score: 0.0,
        result: "".to_string(),
        cases: vec![],
    };
    /*
      structure:
      /tmp/rust_oj/ [user_name/problem_name/TIME/]
        - code.rs (code)
        - a (compiled program)
        - case_i/
            - output (result file)
            - record of time/correct
     */

    /* Save the user program*/
    fs::create_dir_all(&work_dir)?;
    let source_path = work_dir.join("code.rs");
    let exec_path = work_dir.join("a");
    let mut source_file = File::create(&source_path)?;
    source_file.write_all(&source_code.as_bytes())?;

    /* Compile */
    let status = Command::new("rustc")
        .arg("-o")
        .arg(&exec_path)
        .arg(&source_path)
        .status()?;

    if !status.success() {
        judge_res.result = "Compilation Error".to_string();
        judge_res.score = 0.0;
        judge_res.cases.push(
            TestJobCase {
                id: 0,
                result: "Compilation Error".to_string(),
                time: Some(0),
                memory: Some(0),
                info: Some("".to_string()),
            }
        );

        let cases = &problem.cases;
        for i in 0..cases.len() {
            judge_res.cases.push(
                TestJobCase {
                    id: (i + 1) as i32,
                    result: "Waiting".to_string(),
                    time: Some(0),
                    memory: Some(0),
                    info: Some("".to_string()),
                }
            )
        }
        return Ok(judge_res);
    } else {
        judge_res.cases.push(
            TestJobCase {
                id: 0,
                result: "Compilation Success".to_string(),
                time: None,
                memory: None,
                info: None,
            }
        );
    }

    /* Judge case by case */
    let cases = &problem.cases;
    let mut final_result = "Accepted".to_string();

    for i in 0..cases.len(){
        let case = &cases[i];
        let case_dir = work_dir.join(format!("case_{i}"));
        let output_path = case_dir.join("output");
        fs::create_dir_all(&case_dir)?;

        let start_time = Instant::now();
        let in_file = File::open(&case.input_file)?;
        let mut out_file = File::create(&output_path)?;

        /* Use multi thread to run user program */
        let mut child = Command::new(&exec_path)
            .stdin(in_file)
            .stdout(out_file)
            .stderr(Stdio::null())
            .spawn()?;

        let mut result = String::from("");
        loop {
            if let Some(status) = child.try_wait()? {
                if !status.success() {
                    result = "Runtime Error".to_string();
                } else {
                    let output = fs::read_to_string(&output_path)?;
                    let answer = fs::read_to_string(&case.answer_file)?;

                    if problem.problem_type == "strict" {
                        /* Strict mode */
                        if output == answer {
                            result = "Accepted".to_string();
                            judge_res.score += case.score;
                        } else {
                            result = "Wrong Answer".to_string();
                        }
                    } else {
                        /* Standard mode */
                        if standard_equal(&output, &answer) {
                           result = "Accepted".to_string();
                            judge_res.score += case.score;
                        } else {
                           result = "Wrong Answer".to_string();
                        }
                    }
                }

                break;
            }
            if start_time.elapsed().as_micros() > case.time_limit as u128 {
                child.kill()?;
                child.wait()?;
                result = "Time Limit Exceeded".to_string();
                break;
            }

            sleep(Duration::from_millis(10));
        };

        let end_time = Instant::now();

        if result != "Accepted" && final_result == "Accepted" {
            final_result = result.clone();
        }
        judge_res.cases.push(
            TestJobCase {
                id: (i + 1) as i32,
                result: result,
                time: Some((end_time - start_time).as_micros()),
                memory: Some(0),
                info: Some("".to_string()),
            }
        )
    }

    println!("process finished with: {status}");

    judge_res.result = final_result;
    Ok(judge_res)
}
