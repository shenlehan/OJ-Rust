use crate::types;
use crate::types::OJConfig;

pub static config: OJConfig = OJConfig::Default();

pub static LANGUAGES: &Vec<&str> = &vec![
    "rust", "c", "c++"
];

pub static PROBLEMS: &Vec<i32> = &vec![0];
pub static USERS: &Vec<i32> = &vec![0];
pub static CONTESTS: &Vec<i32> = &vec![0];


pub static JOB_ID: i32 = 0;
pub static USER_ID: i32 = 0;
pub static CONTEST_ID: i32 = 0;

pub fn read_config_file(config_file: &mut String) -> Result<(), Box<dyn std::error::Error>> {
    let mut has_config_file = 0;
    let mut meet_config_file = 0;

    let mut flush_data = false;

    for arg in std::env::args() {
        if arg == "-c" || arg == "--config" {
            has_config_file = 1;
            meet_config_file = 1;
        } else if meet_config_file == 1 {
            *config_file = arg.clone();
            meet_config_file = 0;
        } else if arg == "-f" || arg == "--flush-data" {
            flush_data = true;
        }
    }

    if has_config_file == 0 || meet_config_file == 1 {
        return Err(Box::from("Error! Missing config file!"));
    }

    Ok(())
}