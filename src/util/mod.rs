use log::{debug, error, trace, warn};

use crate::graphs::Graphs;

/// Watches the memory that is in use for Graphs
///
/// Triggered on a new Graph addition or update.
/// The limit is set on .env file or as environmental variable
/// with GRUPHST_MAX_MEM_USAGE in MB.
/// In case that the memory is close to the configured max value,
/// the data will be persisted on fs, and the process will exit.
pub(crate) fn graphs_memory_watcher(graphs: &Graphs) {
    let mem = graphs.get_mem().unwrap();
    let max_mem = graphs.get_graphs_stats().get_max_mem();
    let mem_prss = (mem as f32 * 100_f32) / max_mem as f32;
    trace!("memory preassure: {:.2}", mem_prss);
    match mem_prss {
        mem_prss if (95_f32..99_f32).contains(&mem_prss) => {
            warn!("memory close to the limit: {:.2}", mem_prss);
        }
        mem_prss if mem_prss >= 99_f32 => {
            error!("memory usage critical: {:.2}", mem_prss);
            error!("auto persisting current graphs: {}, then panicking", graphs.get_label());
            let _ = graphs.persists();
            panic!("memory usage critical, auto-persisted current graphs");
        }
        _ => debug!("memory ok: {:.2}", mem_prss),
    }
}
