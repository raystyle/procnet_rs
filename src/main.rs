//! procnet CLI(REQ-076):socket_snapshot 出 NDJSON(每行一连接带属主)。
//! snapshot = 一次性;watch --interval N = 周期流式(Vector exec source 消费形)。
use procnet::host::create_process_lookup;
use serde_json::json;
use std::io::Write;
use std::time::Duration;

fn emit(lookup: &dyn procnet::host::ProcessLookup, host: &str) -> anyhow::Result<usize> {
    let snap = lookup.socket_snapshot();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let mut n = 0;
    for s in snap.sockets.iter() {
        let owner = &s.owner;
        let line = json!({
            "host": host,
            "ts": chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            "proto": format!("{}", s.protocol),
            "local_ip": s.local_addr.ip().to_string(),
            "local_port": s.local_addr.port(),
            "remote_ip": s.remote_addr.map(|a| a.ip().to_string()),
            "remote_port": s.remote_addr.map(|a| a.port()),
            "state": match &s.state { procnet::host::HostSocketState::Tcp(t) => format!("{t}"), procnet::host::HostSocketState::UdpBound => "UDP_BOUND".into() },
            "pid": owner.as_ref().map(|o| o.pid),
            "name": owner.as_ref().map(|o| o.name.clone()),
            "uid": owner.as_ref().map(|o| o.uid),
        });
        writeln!(out, "{line}")?;
        n += 1;
    }
    Ok(n)
}

fn hostname_default() -> String {
    std::fs::read_to_string("/proc/sys/kernel/hostname")
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

fn usage() -> ! {
    eprintln!("procnet 0.1.0 (rustnet-host absorption; Apache-2.0 upstream domcyrus/rustnet)");
    eprintln!("usage: procnet snapshot | procnet watch [--interval SECS]   (NDJSON on stdout)");
    std::process::exit(2);
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut interval = 30u64;
    let mut mode = "";
    let mut host = String::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "snapshot" => mode = "snapshot",
            "watch" => mode = "watch",
            "--host" => {
                i += 1;
                host = args.get(i).cloned().unwrap_or_default();
            }
            "--interval" => {
                i += 1;
                interval = args.get(i).and_then(|v| v.parse().ok()).unwrap_or(30);
            }
            _ => usage(),
        }
        i += 1;
    }
    if mode.is_empty() {
        usage();
    }
    let host = if host.is_empty() { hostname_default() } else { host };
    let lookup = create_process_lookup(false)?;
    lookup.refresh()?;
    if mode == "snapshot" {
        let n = emit(lookup.as_ref(), &host)?;
        eprintln!("procnet: {n} sockets");
        return Ok(());
    }
    loop {
        let n = emit(lookup.as_ref(), &host)?;
        eprintln!("procnet: {n} sockets");
        lookup.refresh()?;
        std::thread::sleep(Duration::from_secs(interval.max(1)));
    }
}
