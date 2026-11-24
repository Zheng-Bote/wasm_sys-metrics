// Copyright (c) 2026 Zheng-Bote
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

/// High-level snapshot of collected system metrics at a point in time.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemMetricsSnapshot {
    pub timestamp_ms: u64,
    pub host: String,
    pub prefix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<CpuMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<MemoryMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loadavg: Option<LoadAvgMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime: Option<UptimeMetrics>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub disk: Vec<DiskMetrics>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub network: Vec<NetworkMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CpuMetrics {
    pub total: CpuTimes,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub cores: Vec<CpuCoreMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CpuTimes {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
    pub steal: u64,
    pub guest: u64,
    pub guest_nice: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CpuCoreMetrics {
    pub core_id: usize,
    pub times: CpuTimes,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MemoryMetrics {
    pub total_bytes: u64,
    pub free_bytes: u64,
    pub available_bytes: u64,
    pub buffers_bytes: u64,
    pub cached_bytes: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_total_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_free_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LoadAvgMetrics {
    pub one_min: f64,
    pub five_min: f64,
    pub fifteen_min: f64,
    pub runnable_tasks: u32,
    pub total_tasks: u32,
    pub last_pid: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UptimeMetrics {
    pub uptime_seconds: f64,
    pub idle_seconds: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiskMetrics {
    pub device: String,
    pub reads_completed: u64,
    pub sectors_read: u64,
    pub writes_completed: u64,
    pub sectors_written: u64,
    pub io_in_progress: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkMetrics {
    pub interface: String,
    pub rx_bytes: u64,
    pub rx_packets: u64,
    pub rx_errors: u64,
    pub rx_dropped: u64,
    pub tx_bytes: u64,
    pub tx_packets: u64,
    pub tx_errors: u64,
    pub tx_dropped: u64,
}
