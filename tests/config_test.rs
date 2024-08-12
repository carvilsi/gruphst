use gruphst::config::*;

#[test]
fn should_retrieve_max_memory_configuration() {
    let config_mem = get_max_mem_usage();
    assert_eq!(config_mem, 1 * 1024 * 1024);
}

#[test]
fn should_retrieve_logging_level_configuration() {
    let config_log_level = get_log_level();
    assert_eq!(config_log_level, log::Level::Error);
}
