use dotenv::dotenv;
use log::debug;

const GRUPHST_MAX_MEM_USAGE: &str = "GRUPHST_MAX_MEM_USAGE";
const DEFAULT_GRUPHST_MAX_MEM_USAGE: usize = 25 * 1024 * 1024;

pub fn get_max_mem_usage() -> usize {
    // reading limit of memory usage
    dotenv().ok();
    match dotenv::var(GRUPHST_MAX_MEM_USAGE) {
        Ok(value) => {
            let mut max = value.parse().unwrap();
            debug!("max_mem usage set to {} MB", max);
            max = max * 1024 * 1024;
            max
        },
        Err(_) => {
            debug!(
                "using default max_mem usage {}",
                DEFAULT_GRUPHST_MAX_MEM_USAGE
            );
            DEFAULT_GRUPHST_MAX_MEM_USAGE
        }
    }
}

const GRUPHST_LOG_LEVEL: &str = "GRUPHST_LOG_LEVEL";
const DEFAULT_GRUPHST_LOG_LEVEL: log::Level = log::Level::Info;

pub fn get_log_level() -> log::Level {
    dotenv().ok();
    match dotenv::var(GRUPHST_LOG_LEVEL) {
        Ok(value) => {
            match value.to_lowercase() {
                "info".to_string() => log::Level::Info,
                _ => log::Level::Info,
            }
        },
        Err(_) => {
            debug!("using default log level: Info");
            log::Level::Info
        }
    }
}
