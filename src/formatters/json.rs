// Copyright (c) 2026 Zheng-Bote
// SPDX-License-Identifier: Apache-2.0

use crate::models::SystemMetricsSnapshot;

pub fn format_json(snapshot: &SystemMetricsSnapshot) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(snapshot)
}
