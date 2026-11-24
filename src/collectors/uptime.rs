// Copyright (c) 2026 Zheng-Bote
// SPDX-License-Identifier: Apache-2.0

use crate::models::UptimeMetrics;

pub fn parse_uptime(content: &str) -> Option<UptimeMetrics> {
    let parts: Vec<&str> = content.split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }

    let uptime_seconds = parts[0].parse::<f64>().unwrap_or(0.0);
    let idle_seconds = parts.get(1).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);

    Some(UptimeMetrics {
        uptime_seconds,
        idle_seconds,
    })
}
