use dotenv::dotenv;

const GRUPHST_MAX_MEM_USAGE: &str = "GRUPHST_MAX_MEM_USAGE";
const DEFAULT_GRUPHST_MAX_MEM_USAGE: usize = 25 * 1024 * 1024;

/// Read maximum memory usage value from .env file
/// or setted environmental variable
/// if not exists the default value is use.
///
/// The GRUPHST_MAX_MEM_USAGE defines the limit of
/// memory used to store data, for in-memory mode and
/// persistence.
///
/// # Example
/// ```rust
/// use gruphst::config::get_max_mem_usage;
///
/// let max_mem = get_max_mem_usage();
/// ```

pub fn get_max_mem_usage() -> usize {
    dotenv().ok();
    match dotenv::var(GRUPHST_MAX_MEM_USAGE) {
        Ok(value) => {
            let mut max_conf: f32 = value.parse().unwrap();
            max_conf = max_conf * 1024.0_f32 * 1024.0_f32;
            max_conf as usize
        }
        Err(_) => DEFAULT_GRUPHST_MAX_MEM_USAGE,
    }
}

const GRUPHST_LOG_LEVEL: &str = "GRUPHST_LOG_LEVEL";
const DEFAULT_GRUPHST_LOG_LEVEL: log::Level = log::Level::Error;

/// Read log level configuration from .env file
/// or setted environmental variable
/// if not exists returns default leve that is Info
///
/// # Example
/// ```rust
/// use gruphst::config::get_log_level;
///
/// let log_level = get_log_level();
/// ```
pub fn get_log_level() -> log::Level {
    dotenv().ok();
    match dotenv::var(GRUPHST_LOG_LEVEL) {
        Ok(value) => match value.to_lowercase().as_str() {
            "trace" => log::Level::Trace,
            "debug" => log::Level::Debug,
            "info" => log::Level::Info,
            "warn" | "warning" => log::Level::Warn,
            "err" | "error" => log::Level::Error,
            _ => DEFAULT_GRUPHST_LOG_LEVEL,
        },
        Err(_) => DEFAULT_GRUPHST_LOG_LEVEL,
    }
}
