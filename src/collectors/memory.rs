// Copyright (c) 2026 Zheng-Bote
// SPDX-License-Identifier: Apache-2.0

use crate::models::MemoryMetrics;
use std::collections::HashMap;

pub fn parse_meminfo(content: &str) -> MemoryMetrics {
    let mut map: HashMap<&str, u64> = HashMap::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            let key = parts[0].trim();
            let val_part = parts[1].trim();
            let num_str = val_part.split_whitespace().next().unwrap_or("0");
            if let Ok(num) = num_str.parse::<u64>() {
                // meminfo is in kB, convert to bytes
                map.insert(key, num * 1024);
            }
        }
    }

    MemoryMetrics {
        total_bytes: map.get("MemTotal").copied().unwrap_or(0),
        free_bytes: map.get("MemFree").copied().unwrap_or(0),
        available_bytes: map.get("MemAvailable").copied().unwrap_or(0),
        buffers_bytes: map.get("Buffers").copied().unwrap_or(0),
        cached_bytes: map.get("Cached").copied().unwrap_or(0),
        swap_total_bytes: map.get("SwapTotal").copied(),
        swap_free_bytes: map.get("SwapFree").copied(),
    }
}
