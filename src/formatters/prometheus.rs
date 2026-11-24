// Copyright (c) 2026 Zheng-Bote
// SPDX-License-Identifier: Apache-2.0

use crate::models::SystemMetricsSnapshot;
use std::fmt::Write;

pub fn format_prometheus(snapshot: &SystemMetricsSnapshot) -> String {
    let mut out = String::with_capacity(2048);
    let p = &snapshot.prefix;
    let h = &snapshot.host;

    // CPU Metrics
    if let Some(ref cpu) = snapshot.cpu {
        let _ = writeln!(out, "# HELP {p}cpu_seconds_total CPU time spent in various modes");
        let _ = writeln!(out, "# TYPE {p}cpu_seconds_total counter");
        let _ = writeln!(out, r#"{p}cpu_seconds_total{{host="{h}",mode="user"}} {}"#, cpu.total.user);
        let _ = writeln!(out, r#"{p}cpu_seconds_total{{host="{h}",mode="nice"}} {}"#, cpu.total.nice);
        let _ = writeln!(out, r#"{p}cpu_seconds_total{{host="{h}",mode="system"}} {}"#, cpu.total.system);
        let _ = writeln!(out, r#"{p}cpu_seconds_total{{host="{h}",mode="idle"}} {}"#, cpu.total.idle);
        let _ = writeln!(out, r#"{p}cpu_seconds_total{{host="{h}",mode="iowait"}} {}"#, cpu.total.iowait);
        let _ = writeln!(out, r#"{p}cpu_seconds_total{{host="{h}",mode="irq"}} {}"#, cpu.total.irq);
        let _ = writeln!(out, r#"{p}cpu_seconds_total{{host="{h}",mode="softirq"}} {}"#, cpu.total.softirq);
        let _ = writeln!(out, r#"{p}cpu_seconds_total{{host="{h}",mode="steal"}} {}"#, cpu.total.steal);

        if !cpu.cores.is_empty() {
            let _ = writeln!(out, "# HELP {p}cpu_core_seconds_total Per-core CPU time spent in various modes");
            let _ = writeln!(out, "# TYPE {p}cpu_core_seconds_total counter");
            for core in &cpu.cores {
                let cid = core.core_id;
                let _ = writeln!(out, r#"{p}cpu_core_seconds_total{{core="{cid}",host="{h}",mode="user"}} {}"#, core.times.user);
                let _ = writeln!(out, r#"{p}cpu_core_seconds_total{{core="{cid}",host="{h}",mode="system"}} {}"#, core.times.system);
                let _ = writeln!(out, r#"{p}cpu_core_seconds_total{{core="{cid}",host="{h}",mode="idle"}} {}"#, core.times.idle);
            }
        }
    }

    // Memory Metrics
    if let Some(ref mem) = snapshot.memory {
        let _ = writeln!(out, "# HELP {p}memory_bytes System memory statistics in bytes");
        let _ = writeln!(out, "# TYPE {p}memory_bytes gauge");
        let _ = writeln!(out, r#"{p}memory_bytes{{host="{h}",type="total"}} {}"#, mem.total_bytes);
        let _ = writeln!(out, r#"{p}memory_bytes{{host="{h}",type="free"}} {}"#, mem.free_bytes);
        let _ = writeln!(out, r#"{p}memory_bytes{{host="{h}",type="available"}} {}"#, mem.available_bytes);
        let _ = writeln!(out, r#"{p}memory_bytes{{host="{h}",type="buffers"}} {}"#, mem.buffers_bytes);
        let _ = writeln!(out, r#"{p}memory_bytes{{host="{h}",type="cached"}} {}"#, mem.cached_bytes);
        if let Some(st) = mem.swap_total_bytes {
            let _ = writeln!(out, r#"{p}memory_bytes{{host="{h}",type="swap_total"}} {st}"#);
        }
        if let Some(sf) = mem.swap_free_bytes {
            let _ = writeln!(out, r#"{p}memory_bytes{{host="{h}",type="swap_free"}} {sf}"#);
        }
    }

    // Load Average
    if let Some(ref load) = snapshot.loadavg {
        let _ = writeln!(out, "# HELP {p}load1 1 minute load average");
        let _ = writeln!(out, "# TYPE {p}load1 gauge");
        let _ = writeln!(out, r#"{p}load1{{host="{h}"}} {}"#, load.one_min);

        let _ = writeln!(out, "# HELP {p}load5 5 minute load average");
        let _ = writeln!(out, "# TYPE {p}load5 gauge");
        let _ = writeln!(out, r#"{p}load5{{host="{h}"}} {}"#, load.five_min);

        let _ = writeln!(out, "# HELP {p}load15 15 minute load average");
        let _ = writeln!(out, "# TYPE {p}load15 gauge");
        let _ = writeln!(out, r#"{p}load15{{host="{h}"}} {}"#, load.fifteen_min);
    }

    // Uptime
    if let Some(ref uptime) = snapshot.uptime {
        let _ = writeln!(out, "# HELP {p}uptime_seconds Total system uptime in seconds");
        let _ = writeln!(out, "# TYPE {p}uptime_seconds counter");
        let _ = writeln!(out, r#"{p}uptime_seconds{{host="{h}"}} {}"#, uptime.uptime_seconds);
    }

    // Disk
    if !snapshot.disk.is_empty() {
        let _ = writeln!(out, "# HELP {p}disk_reads_completed_total Total reads completed on device");
        let _ = writeln!(out, "# TYPE {p}disk_reads_completed_total counter");
        for d in &snapshot.disk {
            let dev = &d.device;
            let _ = writeln!(out, r#"{p}disk_reads_completed_total{{device="{dev}",host="{h}"}} {}"#, d.reads_completed);
        }

        let _ = writeln!(out, "# HELP {p}disk_writes_completed_total Total writes completed on device");
        let _ = writeln!(out, "# TYPE {p}disk_writes_completed_total counter");
        for d in &snapshot.disk {
            let dev = &d.device;
            let _ = writeln!(out, r#"{p}disk_writes_completed_total{{device="{dev}",host="{h}"}} {}"#, d.writes_completed);
        }

        let has_size = snapshot.disk.iter().any(|d| d.size_bytes.is_some());
        if has_size {
            let _ = writeln!(out, "# HELP {p}disk_size_bytes Total disk size in bytes");
            let _ = writeln!(out, "# TYPE {p}disk_size_bytes gauge");
            for d in &snapshot.disk {
                if let Some(s) = d.size_bytes {
                    let dev = &d.device;
                    let _ = writeln!(out, r#"{p}disk_size_bytes{{device="{dev}",host="{h}"}} {s}"#);
                }
            }
        }

        let has_used = snapshot.disk.iter().any(|d| d.used_bytes.is_some());
        if has_used {
            let _ = writeln!(out, "# HELP {p}disk_used_bytes Used disk space in bytes");
            let _ = writeln!(out, "# TYPE {p}disk_used_bytes gauge");
            for d in &snapshot.disk {
                if let Some(u) = d.used_bytes {
                    let dev = &d.device;
                    let _ = writeln!(out, r#"{p}disk_used_bytes{{device="{dev}",host="{h}"}} {u}"#);
                }
            }
        }

        let has_avail = snapshot.disk.iter().any(|d| d.available_bytes.is_some());
        if has_avail {
            let _ = writeln!(out, "# HELP {p}disk_available_bytes Available disk space in bytes");
            let _ = writeln!(out, "# TYPE {p}disk_available_bytes gauge");
            for d in &snapshot.disk {
                if let Some(a) = d.available_bytes {
                    let dev = &d.device;
                    let _ = writeln!(out, r#"{p}disk_available_bytes{{device="{dev}",host="{h}"}} {a}"#);
                }
            }
        }
    }

    // Network
    if !snapshot.network.is_empty() {
        let _ = writeln!(out, "# HELP {p}network_receive_bytes_total Total network bytes received");
        let _ = writeln!(out, "# TYPE {p}network_receive_bytes_total counter");
        for net in &snapshot.network {
            let iface = &net.interface;
            let _ = writeln!(out, r#"{p}network_receive_bytes_total{{host="{h}",interface="{iface}"}} {}"#, net.rx_bytes);
        }

        let _ = writeln!(out, "# HELP {p}network_transmit_bytes_total Total network bytes transmitted");
        let _ = writeln!(out, "# TYPE {p}network_transmit_bytes_total counter");
        for net in &snapshot.network {
            let iface = &net.interface;
            let _ = writeln!(out, r#"{p}network_transmit_bytes_total{{host="{h}",interface="{iface}"}} {}"#, net.tx_bytes);
        }
    }

    out
}
