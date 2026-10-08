// Windows process attribution via IP Helper API(procnet v1:ETW 未接,桩化;源件 vendor-pending/windows/etw.rs 留待后续批)。

mod process;

use process::WindowsProcessLookup;

use crate::host::ProcessLookup;
use anyhow::Result;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Take the read lock, recovering the data behind a poisoned lock with a
/// warning that names `what`.
fn read_recovering<'a, T>(lock: &'a RwLock<T>, what: &str) -> RwLockReadGuard<'a, T> {
    lock.read().unwrap_or_else(|poisoned| {
        log::warn!("{what} lock was poisoned, recovering data");
        poisoned.into_inner()
    })
}

/// Take the write lock, recovering the data behind a poisoned lock with a
/// warning that names `what`.
fn write_recovering<'a, T>(lock: &'a RwLock<T>, what: &str) -> RwLockWriteGuard<'a, T> {
    lock.write().unwrap_or_else(|poisoned| {
        log::warn!("{what} lock was poisoned, recovering data");
        poisoned.into_inner()
    })
}

/// Create a Windows process lookup implementation and its initial IP Helper
/// snapshot. ETW collection remains deferred until
/// [`ProcessLookup::start_runtime`] is called.
/// The `_use_pktap` parameter is ignored on Windows (macOS only).
pub fn create_process_lookup(_use_pktap: bool) -> Result<Box<dyn ProcessLookup>> {
    log::info!("Preparing Windows process lookup (IP Helper API; ETW stubbed v1)");
    Ok(Box::new(WindowsProcessLookup::new()?))
}
