//! procnet:迷你进程↔套接字关联库(rustnet-host 吸收;REQ-076)。
//! 上游 github.com/domcyrus/rustnet tree 36601b9,Apache-2.0;吸收面 =
//! rustnet-host 整 crate + core 类型子集(identity + 最小 Connection),
//! 去 eBPF/TUI/捕获/沙箱。v1 仅 linux 后端在编;macos/windows/freebsd
//! 源件留 vendor-pending/ 待后续批接线。
pub mod host;
pub mod types;
pub use host::{create_process_lookup, HostSocket, ProcessLookup};
pub use types::{Connection, Protocol};
