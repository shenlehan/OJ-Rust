pub fn read_config_file(config_file: &mut String) -> std::io::Result<()> {
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
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "missing config file",
        ));
    }

    Ok(())
}