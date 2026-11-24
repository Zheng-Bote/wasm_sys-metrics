# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-11-24

### Added
- Initial release of `wasm_sys-metrics`, a WebAssembly/Spin (WASI 0.2 / Component Model) microservice in Rust.
- INI configuration parser (`src/config/`) reading `config/metrics.ini` with fallback to safe defaults.
- Linux pseudo-filesystem collectors (`src/collectors/`) for:
  - CPU usage (`/proc/stat`), including aggregate and per-core counters.
  - Memory statistics (`/proc/meminfo`), including total, free, available, buffers, cache, and swap.
  - Load average (`/proc/loadavg`) for 1, 5, and 15 minute averages and task counts.
  - System uptime and idle time (`/proc/uptime`).
  - Disk I/O operations and sectors read/written (`/proc/diskstats`).
  - Disk capacity (`/proc/partitions`) exposing `size_bytes` (SIZE), `used_bytes` (USED), and `available_bytes` (AVAIL).
  - Network interface throughput and packet counters (`/proc/net/dev`).
- Exporters/Formatters (`src/formatters/`) supporting:
  - Prometheus Text Exposition Format (`text/plain; version=0.0.4; charset=utf-8`).
  - Structured JSON format (`application/json`).
- Spin HTTP handler with route dispatching and content negotiation:
  - `GET /metrics` (Prometheus or JSON format).
  - `GET /metrics/json` (direct JSON format).
  - `GET /health` (service health and uptime check).
  - `GET /config` (active configuration overview without leaking sensitive data).
- Graceful degradation: returns safe zero-value metrics on restricted environments or when `/proc` is missing.
- Manifest `spin.toml` pre-configured with directory mounts (`config` and `host_proc -> /proc`).
- Comprehensive unit and integration test suite (`tests/`) following Spec-Driven Development (SDD) standards with criteria `AC-1` through `AC-7`.
- Complete documentation in English (`README.md`) and German (`README_de.md`).
