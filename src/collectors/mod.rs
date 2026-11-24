// Copyright (c) 2026 Zheng-Bote
// SPDX-License-Identifier: Apache-2.0

pub mod cpu;
pub mod disk;
pub mod loadavg;
pub mod memory;
pub mod network;
pub mod uptime;

use crate::config::MetricsConfig;
use crate::models::{CpuCoreMetrics, CpuMetrics, DiskMetrics, NetworkMetrics, SystemMetricsSnapshot};
use std::fs;
use std::path::Path;

/// Collects system metrics based on the given configuration and pseudo-filesystem root.
/// Never panics: if pseudo-files are missing or inaccessible, graceful fallbacks are returned.
pub fn collect_snapshot(config: &MetricsConfig, proc_root: &Path) -> SystemMetricsSnapshot {
    let host = if config.general.host.is_empty() {
        "localhost".to_string()
    } else {
        config.general.host.clone()
    };

    let prefix = if config.general.prefix.is_empty() {
        "sys_".to_string()
    } else {
        config.general.prefix.clone()
    };

    let mut snapshot = SystemMetricsSnapshot {
        timestamp_ms: get_timestamp_ms(),
        host,
        prefix,
        cpu: None,
        memory: None,
        loadavg: None,
        uptime: None,
        disk: Vec::new(),
        network: Vec::new(),
    };

    // 1. CPU Metrics (/proc/stat)
    if config.cpu.enabled {
        let stat_path = proc_root.join("stat");
        if let Ok(content) = fs::read_to_string(&stat_path) {
            let mut total_opt = None;
            let mut cores = Vec::new();

            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }
                if parts[0] == "cpu" {
                    total_opt = cpu::parse_cpu_times(&parts[1..]);
                } else if config.cpu.per_core && parts[0].starts_with("cpu") {
                    let core_str = &parts[0][3..];
                    if let Ok(core_id) = core_str.parse::<usize>() {
                        if let Some(times) = cpu::parse_cpu_times(&parts[1..]) {
                            cores.push(CpuCoreMetrics { core_id, times });
                        }
                    }
                }
            }

            if let Some(total) = total_opt {
                snapshot.cpu = Some(CpuMetrics { total, cores });
            }
        }

        // Graceful fallback if /proc/stat was missing or unreadable
        if snapshot.cpu.is_none() {
            snapshot.cpu = Some(CpuMetrics {
                total: Default::default(),
                cores: Vec::new(),
            });
        }
    }

    // 2. Memory Metrics (/proc/meminfo)
    if config.memory.enabled {
        let meminfo_path = proc_root.join("meminfo");
        if let Ok(content) = fs::read_to_string(&meminfo_path) {
            let mut mem = memory::parse_meminfo(&content);
            if !config.memory.swap {
                mem.swap_total_bytes = None;
                mem.swap_free_bytes = None;
            }
            snapshot.memory = Some(mem);
        } else {
            snapshot.memory = Some(Default::default());
        }
    }

    // 3. Load Average (/proc/loadavg)
    if config.loadavg.enabled {
        let loadavg_path = proc_root.join("loadavg");
        if let Ok(content) = fs::read_to_string(&loadavg_path) {
            snapshot.loadavg = loadavg::parse_loadavg(&content);
        }
        if snapshot.loadavg.is_none() {
            snapshot.loadavg = Some(Default::default());
        }
    }

    // 4. Uptime (/proc/uptime)
    if config.uptime.enabled {
        let uptime_path = proc_root.join("uptime");
        if let Ok(content) = fs::read_to_string(&uptime_path) {
            snapshot.uptime = uptime::parse_uptime(&content);
        }
        if snapshot.uptime.is_none() {
            snapshot.uptime = Some(Default::default());
        }
    }

    // 5. Disk Stats (/proc/diskstats and /proc/partitions)
    if config.disk.enabled {
        let partitions_path = proc_root.join("partitions");
        let partitions_map = if let Ok(part_content) = fs::read_to_string(&partitions_path) {
            Some(disk::parse_partitions(&part_content))
        } else {
            None
        };

        let diskstats_path = proc_root.join("diskstats");
        if let Ok(content) = fs::read_to_string(&diskstats_path) {
            snapshot.disk = disk::parse_diskstats(&content, &config.disk.mounts, partitions_map.as_ref());
        }
        // Fallback default disk entry if empty
        if snapshot.disk.is_empty() {
            snapshot.disk = vec![DiskMetrics {
                device: "system".to_string(),
                reads_completed: 0,
                sectors_read: 0,
                writes_completed: 0,
                sectors_written: 0,
                io_in_progress: 0,
                size_bytes: None,
                used_bytes: None,
                available_bytes: None,
            }];
        }
    }

    // 6. Network Stats (/proc/net/dev)
    if config.network.enabled {
        let net_path = proc_root.join("net").join("dev");
        if let Ok(content) = fs::read_to_string(&net_path) {
            snapshot.network = network::parse_net_dev(&content, &config.network.interfaces);
        }
        // Fallback default network entry if empty
        if snapshot.network.is_empty() {
            snapshot.network = vec![NetworkMetrics {
                interface: "default".to_string(),
                rx_bytes: 0,
                rx_packets: 0,
                rx_errors: 0,
                rx_dropped: 0,
                tx_bytes: 0,
                tx_packets: 0,
                tx_errors: 0,
                tx_dropped: 0,
            }];
        }
    }

    snapshot
}

fn get_timestamp_ms() -> u64 {
    // Portable timestamp fallback:
    // In wasi / std environments, SystemTime works
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
