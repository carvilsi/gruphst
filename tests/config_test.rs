use gruphst::config::*;

#[test]
fn configuration() {
    let config_mem = get_max_mem_usage();
    assert_eq!(config_mem, 50 * 1024 * 1024);
    let config_log_level = get_log_level();
    assert_eq!(config_log_level, log::Level::Debug);
}