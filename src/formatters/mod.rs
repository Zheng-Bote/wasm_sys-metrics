// Copyright (c) 2026 Zheng-Bote
// SPDX-License-Identifier: Apache-2.0

pub mod json;
pub mod prometheus;

use crate::models::SystemMetricsSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Prometheus,
    Json,
}

impl OutputFormat {
    pub fn content_type(&self) -> &'static str {
        match self {
            OutputFormat::Prometheus => "text/plain; version=0.0.4; charset=utf-8",
            OutputFormat::Json => "application/json",
        }
    }
}

pub fn format_snapshot(snapshot: &SystemMetricsSnapshot, format: OutputFormat) -> Result<String, String> {
    match format {
        OutputFormat::Prometheus => Ok(prometheus::format_prometheus(snapshot)),
        OutputFormat::Json => json::format_json(snapshot).map_err(|e| e.to_string()),
    }
}
