use chrono::Utc;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::thread::sleep;
use crate::configs::config;
use crate::types::OJConfig;

pub fn run(usr_name: &str, problem_id: &i32, source_code: &str) -> Result<(), std::io::Error> {
    let problem = &config.problems[*problem_id as usize];
    let usr_program_name =
        *usr_name + "/" + problem.name + "/" + Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

    /*
      structure:
      /tmp/rust_oj/ [user_name/problem_name/TIME/]
        - code.rs
        - a
        - case_i/
            - .ans
            - record of time/correct
     */

    /* Save the user program*/
    fs::create_dir_all("/tmp/rust_oj/" + usr_program_name)?;
    let mut f = File::create("/tmp/rust_oj/" + usr_program_name + "/code.rs")?;
    f.write_all(&source_code.to_be_bytes())?;

    /* Compile */
    let status = Command::new("rustc")
        .arg("-o")
        .arg("/tmp/rust_oj" + usr_program_name + "/a" )
        .arg("/tmp/rust_oj" + usr_program_name + "/code.rs")
        .status()?;

    /* Judge case by case */
    let cases = &problem.cases;
    for i in 0..cases.len(){
        let case = &cases[i];
        let start_time = Instant::now();
        let in_file = File::open(case.input_file)?;
        let answer_file = File::open(case.answer_file)?;
        let out_file = File::create("/tmp/rust_oj/" + usr_program_name + "/case_" + i.to_string());

        let status = Command::new("./" + )
            .args(["--random", "-t"])
            .stdin(Stdio::from(in_file))
            .stdout(Stdio::from(out_file))
            .stderr(Stdio::null())
            .status()?;


    }

    println!("process finished with: {status}");
}
