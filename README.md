# procnet_rs — 迷你进程↔套接字关联库 + CLI

[rustnet](https://github.com/domcyrus/rustnet) 的 `rustnet-host` crate 吸收面
(tree `36601b9`,Apache-2.0):进程归属 + 主机套接字清单,`socket_snapshot()`
单原语枚举全 TCP/UDP 连接带属主。**无 TUI、无包捕获、无沙箱**;vendor 了
rustnet-core 的类型子集(identity.rs + static_names 宏 + 最小 Connection 三字段),
不拉 DPI/GeoIP/rates 重依赖;eBPF 特性未启(procfs 轮询道)。

## 用法

```sh
procnet snapshot                 # 一次性 NDJSON(每行一连接带属主)
procnet watch --interval 30      # 周期流式(Vector exec source 消费形)
```

行形:`{ts,proto,local_ip,local_port,remote_ip,remote_port,state,pid,name,uid}`。
Linux v1 在编;macos/windows/freebsd 源件在 `vendor-pending/` 待后续批接线
(上游 lsof+libproc / IP Helper+ETW / sockstat 道)。

## 管道位(REQ-076)

```
procnet watch → Vector(exec source)→ ClickHouse nsm.proc_conns → actl nsm procs
```

deploy/ 内含 vector.toml / systemd unit / 建表 SQL / 安装脚本。

## 归因

上游 domcyrus/rustnet Apache-2.0;吸收清单与重写点见 git 历史。MSRV 1.88
(edition 2024,上游 let-chains)。
