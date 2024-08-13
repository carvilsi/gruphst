use gruphst::config::*;

#[test]
fn should_retrieve_max_memory_configuration() {
    let config_mem = get_max_mem_usage();
    let max_mem = 0.1_f32 * 1024.0_f32 * 1024.0_f32;
    assert_eq!(config_mem, max_mem as usize);
}

#[test]
fn should_retrieve_logging_level_configuration() {
    let config_log_level = get_log_level();
    assert_eq!(config_log_level, log::Level::Error);
}
