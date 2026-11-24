---
feature: configurable-system-metrics
status: in_progress
archive-after: 2027-01-01
touches:
  - "Cargo.toml"
  - "spin.toml"
  - "src/**"
  - "tests/**"
  - "config/**"
  - "README.md"
  - "README_de.md"
  - ".gitignore"
---

# PRD — Konfigurierbare System-Metriken per Abruf (`configurable-system-metrics`)

## Problem

Monitoring- und Observability-Lösungen benötigen verlässliche, leichtgewichtige Metrik-Endpunkte auf Host- und Edge-Knoten. Bisherige Lösungen sind oft monolithische Daemons mit hohem Ressourcenverbrauch und unflexibler Konfiguration. Entwickler und Betreiber im WebAssembly- und Spin-Umfeld benötigen einen hochgradig isolierten, ressourcenschonenden Microservice, der konfigurierbare Systemmetriken gezielt und sicher über HTTP abrufbar macht.

## Ziel

Ein performanter, sicherer Rust-basierter WebAssembly/Spin Microservice, der Systemmetriken (wie CPU-Auslastung, Speicherverbrauch, Load Average, Uptime und Netzwerktraffic) ausliest, anhand einer INI-Konfigurationsdatei filtert und strukturiert über standardisierte HTTP-Endpunkte im Prometheus- und JSON-Format bereitstellt.

## Nutzer

- Plattform- und System-Administratoren, die Edge- oder Servermetriken abrufen wollen.
- Observability-Tools (z. B. Prometheus, Grafana Alloy, Datadog), die `/metrics` scrapen.
- Entwickler, die Uptime und Health-Checks in Spin-Umgebungen ausführen.

## Umfang

- Parser für INI-Konfiguration (`metrics.ini` bzw. Standardpfad `/config.ini`).
- System-Metrik-Kollektoren für Linux pseudo-Dateisysteme (`/proc/stat`, `/proc/meminfo`, `/proc/loadavg`, `/proc/uptime`, `/proc/net/dev`, `/proc/diskstats`).
- Exporter/Formatierer für Prometheus Text Exposition Format (`text/plain; version=0.0.4`) und JSON (`application/json`).
- Spin HTTP Handler für Endpunkte:
  - `GET /metrics` (unterstützt Format-Aushandlung via `Accept` oder Query-Parameter `?format=json|prometheus`)
  - `GET /metrics/json`
  - `GET /health`
  - `GET /config` (Übersicht der aktiven Schalter ohne sensible Informationen)
- Robuste Ausfallsicherheit (Graceful Degradation bei fehlenden `/proc`-Mounts oder nicht-Linux-Umgebungen).
- `spin.toml` Manifest für Spin-Deployment mit entsprechenden Pfad-Mappings.

## Nicht-Ziele

- Kein Schreiben oder Verändern von Systemeinstellungen oder Hostdateien.
- Keine native Ausführung von Subprozessen oder Shell-Befehlen.
- Kein Push-Gateway (reines Pull/Scrape-Modell per Abruf).
- Keine Authentifizierung/TLS-Terminierung im WASM-Code (dies übernimmt der vorgeschaltete Spin-Host / Ingress / Proxy).

## Akzeptanzkriterien

- **AC-1**: Das System liest eine INI-Konfigurationsdatei ein, validiert Abschnitte (`[general]`, `[cpu]`, `[memory]`, `[disk]`, `[loadavg]`, `[network]`) und fällt bei ungültiger oder fehlender Konfiguration auf sichere Standardwerte zurück, ohne zu panicken.
- **AC-2**: Das System implementiert modulare Kollektoren zur Erfassung von Systemmetriken aus Host-Pseudodateien (`/proc/stat`, `/proc/meminfo`, `/proc/loadavg`, `/proc/uptime`, `/proc/net/dev`, `/proc/diskstats`).
- **AC-3**: Die erfassten Metriken können wahlweise im standardisierten Prometheus Text Exposition Format (`text/plain; version=0.0.4`) oder im JSON-Format (`application/json`) serialisiert werden.
- **AC-4**: Der Spin HTTP Handler stellt die Endpunkte `GET /metrics`, `GET /metrics/json`, `GET /health` und `GET /config` mit korrekten HTTP-Statuscodes und Content-Type-Headern bereit.
- **AC-5**: Wenn Pseudodateien nicht lesbar oder in der Sandbox nicht eingehängt sind, verarbeitet das System I/O-Fehler fehlertolerant und liefert Fallback-Metriken oder Statusinformationen, ohne abzustürzen oder einen unkontrollierten 500-Fehler zu werfen.
- **AC-6**: Das Projekt kompiliert fehlerfrei gegen das Rust-WASI-Target `wasm32-wasip2`, verfügt über eine gültige `spin.toml` und besteht alle Unit- und Integrationstests.
