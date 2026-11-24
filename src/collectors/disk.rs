// Copyright (c) 2026 Zheng-Bote
// SPDX-License-Identifier: Apache-2.0

use crate::models::DiskMetrics;
use std::collections::HashMap;

/// Parses /proc/partitions and returns mapping of device/partition name to size in bytes.
pub fn parse_partitions(content: &str) -> HashMap<String, u64> {
    let mut map = HashMap::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // Line format: major minor #blocks name
        if parts.len() >= 4 {
            if let (Ok(blocks), name) = (parts[2].parse::<u64>(), parts[3]) {
                // #blocks in /proc/partitions is in 1024-byte blocks (1 KiB)
                map.insert(name.to_string(), blocks * 1024);
            }
        }
    }

    map
}

/// Parses /proc/diskstats and optionally combines with size information from /proc/partitions.
pub fn parse_diskstats(
    content: &str,
    target_devices: &[String],
    partition_sizes: Option<&HashMap<String, u64>>,
) -> Vec<DiskMetrics> {
    let mut results = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // /proc/diskstats format has at least 14 columns
        if parts.len() >= 14 {
            let dev_name = parts[2].to_string();

            // If target_devices is not empty and doesn't match wildcard, filter
            if !target_devices.is_empty()
                && !target_devices.iter().any(|d| d == "/" || d == "*" || d == &dev_name)
            {
                continue;
            }

            // Exclude loop devices and ram devices unless explicitly requested
            if (dev_name.starts_with("loop") || dev_name.starts_with("ram"))
                && !target_devices.iter().any(|d| d == &dev_name)
            {
                continue;
            }

            let reads_completed = parts[3].parse::<u64>().unwrap_or(0);
            let sectors_read = parts[5].parse::<u64>().unwrap_or(0);
            let writes_completed = parts[7].parse::<u64>().unwrap_or(0);
            let sectors_written = parts[9].parse::<u64>().unwrap_or(0);
            let io_in_progress = parts[11].parse::<u64>().unwrap_or(0);

            let size_bytes = partition_sizes.and_then(|sizes| sizes.get(&dev_name).copied());

            results.push(DiskMetrics {
                device: dev_name,
                reads_completed,
                sectors_read,
                writes_completed,
                sectors_written,
                io_in_progress,
                size_bytes,
                used_bytes: None,
                available_bytes: None,
            });
        }
    }

    results
}
