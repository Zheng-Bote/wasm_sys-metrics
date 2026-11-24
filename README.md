# wasm_sys-metrics

Configurable system metrics exporter written in Rust as a WebAssembly (WASI 0.2 / Component Model) microservice for Fermyon Spin.

## Table of Contents

- [Features](#features)
- [Installing Fermyon Spin](#installing-fermyon-spin)
- [Configuration (config/metrics.ini)](#configuration-configmetricsini)
- [Building & Compilation](#building--compilation)
- [Running the Application](#running-the-application)
- [HTTP Endpoints](#http-endpoints)
- [Development & Testing](#development--testing)
- [License](#license)

---

## Features

- **INI-Configurable:** Control and filter metric collectors (`cpu`, `memory`, `loadavg`, `uptime`, `disk`, `network`) via `config/metrics.ini` or `/config/metrics.ini`.
- **WASM & Spin (Component Model):** Built against the `wasm32-wasip2` target using `spin-sdk` v3.
- **Multiple Output Formats:**
  - Prometheus Text Exposition Format (`text/plain; version=0.0.4`)
  - Structured JSON (`application/json`)
- **Content Negotiation & Routing:**
  - `GET /metrics` — Prometheus text format (default) or JSON via query `?format=json` or `Accept: application/json`
  - `GET /metrics/json` — Direct JSON format
  - `GET /health` — Status and uptime (`{"status":"ok", ...}`)
  - `GET /config` — Sanitized summary of active configuration switches
- **Graceful Degradation:** Safely falls back to default values if pseudo-filesystems (`/proc`) are unmounted or restricted (Zero Panic policy).
- **Spec-Driven Development (SDD):** Developed and verified according to `sdd` quality gates.

---

## Installing Fermyon Spin

> [!CAUTION]
> **Important:** Do **not** run `sudo apt install spin`!
> The `spin` APT package in Ubuntu/Debian is a formal verification tool (Bell Labs Logic Model Checker), **not** the Fermyon Spin WebAssembly CLI.

### Recommended Installation

#### Option 1: Official Install Script
```bash
curl -fsSL https://developer.fermyon.com/downloads/install.sh | bash
mv spin ~/.local/bin/
```

#### Option 2: Precompiled Binary from GitHub Releases
Download the release archive from [GitHub Releases (spinframework/spin)](https://github.com/spinframework/spin/releases):
```bash
# Example for Linux x86_64:
curl -sL https://github.com/spinframework/spin/releases/download/v4.3.0/spin-v4.3.0-linux-amd64.tar.gz -o /tmp/spin.tar.gz
tar -xzf /tmp/spin.tar.gz -C ~/.local/bin spin
chmod +x ~/.local/bin/spin
rm /tmp/spin.tar.gz
```

#### Verify Installation
```bash
spin --version
# Output e.g.: spin 4.3.0
```

---

## Configuration (`config/metrics.ini`)

Exporter behavior is controlled through an INI file:

```ini
; Configuration for wasm_sys-metrics
[general]
format = prometheus
prefix = sys_
host = wasm-node

[cpu]
enabled = true
per_core = true

[memory]
enabled = true
swap = true

[loadavg]
enabled = true

[uptime]
enabled = true

[disk]
enabled = true
mounts = /

[network]
enabled = true
interfaces = eth0,lo
```

---

## Building & Compilation

Ensure the Rust `wasm32-wasip2` target is installed:
```bash
rustup target add wasm32-wasip2
```

Build with Spin:
```bash
spin build
```
*(Alternatively using Cargo: `cargo build --target wasm32-wasip2 --release --features spin`)*

The optimized WebAssembly component will be placed at `target/wasm32-wasip2/release/wasm_sys_metrics.wasm`.

---

## Running the Application

Start the Spin application with the `--direct-mounts` flag:

```bash
spin up --direct-mounts
```

> [!NOTE]
> **Why `--direct-mounts`?**
> WebAssembly components run inside an isolated WASI sandbox. By default, Spin copies mounted assets to a temporary folder. Because Linux kernel pseudo-files under `/proc` are virtual 0-byte streams, copying them fails.
> By utilizing the repository's `host_proc -> /proc` symlink and the `--direct-mounts` flag, Spin mounts the pseudo-filesystem directly into the sandbox, enabling live kernel metric collection.

---

## HTTP Endpoints

By default, Spin listens on `http://127.0.0.1:3000`.

### 1. Prometheus Metrics (`GET /metrics`)
```bash
curl http://127.0.0.1:3000/metrics
```
*Response Header:* `Content-Type: text/plain; version=0.0.4; charset=utf-8`

### 2. JSON Metrics (`GET /metrics/json` or `GET /metrics?format=json`)
```bash
curl http://127.0.0.1:3000/metrics/json
# or
curl -H "Accept: application/json" http://127.0.0.1:3000/metrics
```
*Response Header:* `Content-Type: application/json`

### 3. Health & Uptime Check (`GET /health`)
```bash
curl http://127.0.0.1:3000/health
```
*Sample Response:*
```json
{
  "host": "wasm-node",
  "status": "ok",
  "timestamp_ms": 1795531275545,
  "uptime_seconds": 16237.56,
  "version": "0.1.0"
}
```

### 4. Configuration Summary (`GET /config`)
```bash
curl http://127.0.0.1:3000/config
```
Returns active collectors and flags without leaking sensitive credentials or filesystem paths.

---

## Development & Testing

### Unit and Integration Tests
```bash
cargo test
```

### Linting and Code Formatting
```bash
cargo clippy -- -D warnings
cargo clippy --features spin -- -D warnings
```

### Spec-Driven Development (SDD) Verification
```bash
sdd lint
sdd gate pre-merge
sdd report
```

---

## License

Apache-2.0
