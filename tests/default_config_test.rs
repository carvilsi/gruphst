use gruphst::config::*;
use std::fs;

#[test]
fn should_retrieve_default_max_memory_and_logging_level_configuration() {
    fs::rename(".env", "env").unwrap();

    let config_mem = get_max_mem_usage();
    let max_mem = 25 * 1024 * 1024;
    assert_eq!(config_mem, max_mem);
    
    let config_log_level = get_log_level();
    assert_eq!(config_log_level, log::Level::Error);

    let config_csv_delimiter: u8 = get_csv_delimiter();
    assert_eq!(config_csv_delimiter, b';');

    fs::rename("env", ".env").unwrap();
}
