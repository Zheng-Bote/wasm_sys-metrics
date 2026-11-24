// Copyright (c) 2026 Zheng-Bote
// SPDX-License-Identifier: Apache-2.0

use crate::models::LoadAvgMetrics;

pub fn parse_loadavg(content: &str) -> Option<LoadAvgMetrics> {
    let parts: Vec<&str> = content.split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }

    let one_min = parts[0].parse::<f64>().unwrap_or(0.0);
    let five_min = parts[1].parse::<f64>().unwrap_or(0.0);
    let fifteen_min = parts[2].parse::<f64>().unwrap_or(0.0);

    let mut runnable_tasks = 0;
    let mut total_tasks = 0;
    let mut last_pid = 0;

    if parts.len() >= 4 {
        let task_parts: Vec<&str> = parts[3].split('/').collect();
        if task_parts.len() == 2 {
            runnable_tasks = task_parts[0].parse::<u32>().unwrap_or(0);
            total_tasks = task_parts[1].parse::<u32>().unwrap_or(0);
        }
    }

    if parts.len() >= 5 {
        last_pid = parts[4].parse::<u32>().unwrap_or(0);
    }

    Some(LoadAvgMetrics {
        one_min,
        five_min,
        fifteen_min,
        runnable_tasks,
        total_tasks,
        last_pid,
    })
}
