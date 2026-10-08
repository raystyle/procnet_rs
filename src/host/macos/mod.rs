// macOS process attribution: PKTAP packet metadata when active, else lsof.

mod process;

use process::MacOSProcessLookup;

use crate::host::{DegradationReason, MatchQuality, ProcessAttribution, ProcessLookup, SocketSnapshot};
use anyhow::Result;
use crate::types::Connection;
use std::sync::OnceLock;
use std::sync::atomic::AtomicBool;

/// Why the PKTAP fast path is unavailable, as reported by the orchestrator.
///
/// PKTAP availability is decided by the capture layer, not by process
/// attribution. Rather than depend on `rustnet-capture`, this crate lets the
/// application inject the reason via [`report_pktap_degradation`]; the lsof
/// lookup reads it back in `get_degradation_reason`.
static PKTAP_DEGRADATION: OnceLock<DegradationReason> = OnceLock::new();

/// Record why PKTAP could not be used so process-attribution degradation can be
/// surfaced to the user. Intended to be called once, by the application, after
/// it has determined PKTAP availability from the capture layer. No-op if already
/// set.
pub fn report_pktap_degradation(reason: DegradationReason) {
    let _ = PKTAP_DEGRADATION.set(reason);
}

/// The reported PKTAP degradation reason, or the conservative default
/// (missing root) when nothing has been reported.
pub(crate) fn pktap_degradation() -> DegradationReason {
    PKTAP_DEGRADATION
        .get()
        .cloned()
        .unwrap_or(DegradationReason::MissingRootPrivileges)
}

/// Enrich process identity carried directly in PKTAP packet metadata.

/// Create a macOS process lookup implementation.
/// Enriches PKTAP packet metadata when active, otherwise falls back to lsof.
pub fn create_process_lookup(_use_pktap: bool) -> Result<Box<dyn ProcessLookup>> {
    // procnet v1:PKTAP 未接(捕获面不存在);lsof + libproc 道即足
    log::info!("Using macOS process lookup (lsof)");
    Ok(Box::new(MacOSProcessLookup::new()?))
}

