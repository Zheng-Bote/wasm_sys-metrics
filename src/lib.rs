// Copyright (c) 2026 Zheng-Bote
// SPDX-License-Identifier: Apache-2.0

pub mod collectors;
pub mod config;
pub mod formatters;
pub mod models;

use collectors::collect_snapshot;
use config::MetricsConfig;
use formatters::{format_snapshot, OutputFormat};
use std::path::Path;

pub use config::MetricsConfig as Config;
pub use models::SystemMetricsSnapshot;

#[derive(Debug, Clone, PartialEq)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn new(status: u16, content_type: &str, body: impl Into<Vec<u8>>) -> Self {
        Self {
            status,
            headers: vec![("Content-Type".to_string(), content_type.to_string())],
            body: body.into(),
        }
    }
}

/// Handles incoming HTTP request routing and execution.
/// Pure logic independent of runtime trigger for high testability.
pub fn handle_request_route(
    method: &str,
    path: &str,
    query: Option<&str>,
    accept_header: Option<&str>,
    config: &MetricsConfig,
    proc_root: &Path,
) -> HttpResponse {
    if method != "GET" && method != "HEAD" {
        return HttpResponse::new(
            405,
            "text/plain",
            format!("Method {} not allowed. Only GET is supported.", method),
        );
    }

    // Normalize path (strip trailing slash if not root)
    let clean_path = if path.len() > 1 && path.ends_with('/') {
        &path[..path.len() - 1]
    } else {
        path
    };

    match clean_path {
        "/metrics" => {
            let format = determine_format(query, accept_header, config);
            let snapshot = collect_snapshot(config, proc_root);
            match format_snapshot(&snapshot, format) {
                Ok(content) => HttpResponse::new(200, format.content_type(), content),
                Err(err) => HttpResponse::new(
                    500,
                    "text/plain",
                    format!("Failed to format metrics: {}", err),
                ),
            }
        }
        "/metrics/json" => {
            let snapshot = collect_snapshot(config, proc_root);
            match format_snapshot(&snapshot, OutputFormat::Json) {
                Ok(content) => HttpResponse::new(200, OutputFormat::Json.content_type(), content),
                Err(err) => HttpResponse::new(
                    500,
                    "text/plain",
                    format!("Failed to format json metrics: {}", err),
                ),
            }
        }
        "/health" => {
            let uptime_snapshot = collect_snapshot(config, proc_root);
            let uptime_sec = uptime_snapshot
                .uptime
                .as_ref()
                .map(|u| u.uptime_seconds)
                .unwrap_or(0.0);

            let json_body = serde_json::json!({
                "status": "ok",
                "version": env!("CARGO_PKG_VERSION"),
                "host": config.general.host,
                "uptime_seconds": uptime_sec,
                "timestamp_ms": uptime_snapshot.timestamp_ms
            });

            HttpResponse::new(200, "application/json", json_body.to_string())
        }
        "/config" => match serde_json::to_string_pretty(config) {
            Ok(json_str) => HttpResponse::new(200, "application/json", json_str),
            Err(e) => HttpResponse::new(
                500,
                "text/plain",
                format!("Failed to serialize config: {}", e),
            ),
        },
        _ => HttpResponse::new(
            404,
            "text/plain",
            format!("Route not found: {}", path),
        ),
    }
}

fn determine_format(
    query: Option<&str>,
    accept_header: Option<&str>,
    config: &MetricsConfig,
) -> OutputFormat {
    if let Some(q) = query {
        for param in q.split('&') {
            let mut kv = param.split('=');
            if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                if k == "format" {
                    if v.eq_ignore_ascii_case("json") {
                        return OutputFormat::Json;
                    } else if v.eq_ignore_ascii_case("prometheus") {
                        return OutputFormat::Prometheus;
                    }
                }
            }
        }
    }

    if let Some(accept) = accept_header {
        if accept.contains("application/json") {
            return OutputFormat::Json;
        }
    }

    if config.general.format.eq_ignore_ascii_case("json") {
        OutputFormat::Json
    } else {
        OutputFormat::Prometheus
    }
}

#[cfg(feature = "spin")]
#[spin_sdk::http_component]
fn handle_spin_http(req: spin_sdk::http::Request) -> anyhow::Result<spin_sdk::http::Response> {
    let method_str = format!("{}", req.method());

    let uri_str = req.uri();
    let (path, query) = match uri_str.split_once('?') {
        Some((p, q)) => (p, Some(q)),
        None => (uri_str, None),
    };

    let accept = req
        .headers()
        .find(|(k, _)| k.eq_ignore_ascii_case("accept"))
        .and_then(|(_, v)| v.as_str());

    let config = MetricsConfig::load_or_default(None);
    let proc_root = Path::new("/proc");

    let res = handle_request_route(&method_str, path, query, accept, &config, proc_root);

    let mut response_builder = spin_sdk::http::Response::builder();
    response_builder.status(res.status);

    for (k, v) in res.headers {
        response_builder.header(k, v);
    }

    Ok(response_builder.body(res.body).build())
}
