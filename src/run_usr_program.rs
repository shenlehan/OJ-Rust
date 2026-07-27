use chrono::Utc;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::thread::sleep;
use crate::types::*;
use std::path::PathBuf;

pub fn run(usr_name: &str, problem: &Problem, source_code: &str) -> Result<(), std::io::Error> {
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
    let work_dir = PathBuf::from("/tmp/rust_oj")
        .join(usr_name)
        .join(&problem.name)
        .join(&timestamp);

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
    let mut source_path = work_dir.join("code.rs");
    let mut exec_path = work_dir.join("a");
    let mut source_file = File::create(&source_path)?;
    source_file.write_all(&source_code.as_bytes())?;

    /* Compile */
    let status = Command::new("rustc")
        .arg("-o")
        .arg(&exec_path)
        .arg(&source_path)
        .status()?;

    /* Judge case by case */
    let cases = &problem.cases;
    for i in 0..cases.len(){
        let case = &cases[i];
        let mut case_dir = work_dir.join(format!("case_{i}"));
        let mut output_path = case_dir.join("output");
        fs::create_dir_all(&case_dir)?;

        let start_time = Instant::now();
        let in_file = File::open(&case.input_file)?;
        let answer_file = File::open(&case.answer_file)?;
        let mut out_file = File::create(&output_path)?;

        let status = Command::new(&exec_path)
            .stdin(in_file)
            .stdout(out_file)
            .stderr(Stdio::null())
            .status()?;
    }

    println!("process finished with: {status}");
    Ok(())
}
