-- nsm.proc_conns(REQ-076):procnet → Vector → CH 的进程连接表
CREATE TABLE IF NOT EXISTS nsm.proc_conns
(
    ts          DateTime64(3),
    host        LowCardinality(String),
    proto       LowCardinality(String),
    local_ip    String,
    local_port  UInt16,
    remote_ip   Nullable(String),
    remote_port Nullable(UInt16),
    state       LowCardinality(String),
    pid         Nullable(UInt32),
    name        Nullable(String),
    uid         Nullable(UInt32)
)
ENGINE = MergeTree
ORDER BY (host, ts)
TTL toDateTime(ts) + INTERVAL 90 DAY;
