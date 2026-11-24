// Copyright (c) 2026 Zheng-Bote
// SPDX-License-Identifier: Apache-2.0

use crate::models::CpuTimes;

pub fn parse_cpu_times(fields: &[&str]) -> Option<CpuTimes> {
    if fields.is_empty() {
        return None;
    }

    let parse_field = |idx: usize| -> u64 {
        fields
            .get(idx)
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0)
    };

    Some(CpuTimes {
        user: parse_field(0),
        nice: parse_field(1),
        system: parse_field(2),
        idle: parse_field(3),
        iowait: parse_field(4),
        irq: parse_field(5),
        softirq: parse_field(6),
        steal: parse_field(7),
        guest: parse_field(8),
        guest_nice: parse_field(9),
    })
}
