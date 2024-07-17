use log::{debug, error, info, trace, warn};
use std::process;
use std::thread;

use crate::config::get_max_mem_usage;
use crate::graphs::Graphs;

/// Watches the memory that is in use for Graphs
///
/// Triggered on a new Graph addition or update.
/// The limit is set on .env file or as environmental variable
/// with GRUPHST_MAX_MEM_USAGE in MB.
/// In case that the memory is close to the configured max value,
/// the data will be persisted on fs, and the process will exit.
pub fn graphs_memory_watcher(graphs: &Graphs) {
    let g = graphs.clone();
    thread::spawn(move || {
        let max_mem = get_max_mem_usage();
        let mem = g.stats().unwrap().get_mem();
        let mem_prss = (mem as f32 * 100_f32) / max_mem as f32;
        trace!("memory preassure: {:.2}", mem_prss);
        match mem_prss {
            mem_prss if mem_prss < 70_f32 => debug!("memory ok: {:.2}", mem_prss),
            mem_prss if (80_f32..95_f32).contains(&mem_prss) => {
                info!("memory high: {:.2}", mem_prss);
            }
            mem_prss if (95_f32..99_f32).contains(&mem_prss) => {
                warn!("memory close to the limit: {:.2}", mem_prss);
            }
            mem_prss if mem_prss >= 99_f32 => {
                error!("memory usage critical: {:.2}", mem_prss);
                error!(
                    "auto persisting current graphs: {}, and stoping execution",
                    g.get_name()
                );
                let _ = g.persists();
                process::exit(1);
            }
            _ => todo!(),
        }
    });
}
