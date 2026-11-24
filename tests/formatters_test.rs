// Copyright (c) 2026 Zheng-Bote
// SPDX-License-Identifier: Apache-2.0

use wasm_sys_metrics::formatters::{format_snapshot, OutputFormat};
use wasm_sys_metrics::models::*;

// configurable-system-metrics AC-3: Formatters for Prometheus text exposition and JSON
#[test]
fn test_ac3_formatters_output() {
    let snapshot = SystemMetricsSnapshot {
        timestamp_ms: 1711280000000,
        host: "cluster-worker-01".to_string(),
        prefix: "sys_".to_string(),
        cpu: Some(CpuMetrics {
            total: CpuTimes {
                user: 100,
                nice: 10,
                system: 50,
                idle: 800,
                iowait: 20,
                irq: 5,
                softirq: 5,
                steal: 0,
                guest: 0,
                guest_nice: 0,
            },
            cores: vec![CpuCoreMetrics {
                core_id: 0,
                times: CpuTimes {
                    user: 50,
                    nice: 5,
                    system: 25,
                    idle: 400,
                    iowait: 10,
                    irq: 2,
                    softirq: 3,
                    steal: 0,
                    guest: 0,
                    guest_nice: 0,
                },
            }],
        }),
        memory: Some(MemoryMetrics {
            total_bytes: 8589934592,
            free_bytes: 4294967296,
            available_bytes: 6442450944,
            buffers_bytes: 104857600,
            cached_bytes: 1073741824,
            swap_total_bytes: Some(2147483648),
            swap_free_bytes: Some(2147483648),
        }),
        loadavg: Some(LoadAvgMetrics {
            one_min: 0.12,
            five_min: 0.24,
            fifteen_min: 0.36,
            runnable_tasks: 1,
            total_tasks: 200,
            last_pid: 1234,
        }),
        uptime: Some(UptimeMetrics {
            uptime_seconds: 7200.5,
            idle_seconds: 6800.2,
        }),
        disk: vec![DiskMetrics {
            device: "nvme0n1".to_string(),
            reads_completed: 5000,
            sectors_read: 200000,
            writes_completed: 8000,
            sectors_written: 400000,
            io_in_progress: 1,
            size_bytes: Some(1000204886016),
            used_bytes: Some(500000000000),
            available_bytes: Some(500204886016),
        }],
        network: vec![NetworkMetrics {
            interface: "eth0".to_string(),
            rx_bytes: 5242880,
            rx_packets: 4000,
            rx_errors: 0,
            rx_dropped: 0,
            tx_bytes: 1048576,
            tx_packets: 1500,
            tx_errors: 0,
            tx_dropped: 0,
        }],
    };

    // Test Prometheus text format
    let prom = format_snapshot(&snapshot, OutputFormat::Prometheus)
        .expect("prometheus formatting should succeed");
    assert!(prom.contains("sys_cpu_seconds_total{host=\"cluster-worker-01\",mode=\"user\"} 100"));
    assert!(prom.contains("sys_cpu_core_seconds_total{core=\"0\",host=\"cluster-worker-01\",mode=\"user\"} 50"));
    assert!(prom.contains("sys_memory_bytes{host=\"cluster-worker-01\",type=\"total\"} 8589934592"));
    assert!(prom.contains("sys_load1{host=\"cluster-worker-01\"} 0.12"));
    assert!(prom.contains("sys_uptime_seconds{host=\"cluster-worker-01\"} 7200.5"));
    assert!(prom.contains("sys_disk_reads_completed_total{device=\"nvme0n1\",host=\"cluster-worker-01\"} 5000"));
    assert!(prom.contains("sys_disk_size_bytes{device=\"nvme0n1\",host=\"cluster-worker-01\"} 1000204886016"));
    assert!(prom.contains("sys_disk_used_bytes{device=\"nvme0n1\",host=\"cluster-worker-01\"} 500000000000"));
    assert!(prom.contains("sys_disk_available_bytes{device=\"nvme0n1\",host=\"cluster-worker-01\"} 500204886016"));
    assert!(prom.contains("sys_network_receive_bytes_total{host=\"cluster-worker-01\",interface=\"eth0\"} 5242880"));

    // Test JSON format
    let json = format_snapshot(&snapshot, OutputFormat::Json)
        .expect("json formatting should succeed");
    assert!(json.contains("\"host\": \"cluster-worker-01\""));
    assert!(json.contains("\"user\": 100"));
    assert!(json.contains("\"uptime_seconds\": 7200.5"));
}
