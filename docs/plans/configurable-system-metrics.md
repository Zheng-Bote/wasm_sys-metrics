---
feature: configurable-system-metrics
status: in_progress
archive-after: 2027-01-01
---

# Plan — Konfigurierbare System-Metriken per Abruf (`configurable-system-metrics`)

Reihenfolge der Umsetzung. Jede Task nennt das Akzeptanzkriterium, auf das sie einzahlt.

## Tasks

- [ ] AC-1: Cargo Projekt initialisieren und INI Konfigurationsparser mit robusten Standardwerten implementieren
- [ ] AC-2: Modulare Metrik-Kollektoren für CPU, Memory, LoadAvg, Uptime, Disk und Netzwerk basierend auf `/proc` Filesystem implementieren
- [ ] AC-3: Formatierungsmodule für Prometheus Text Exposition Format und JSON Serialisierung implementieren
- [ ] AC-4: Spin HTTP Handler mit Routen-Dispatching für `/metrics`, `/metrics/json`, `/health` und `/config` implementieren
- [ ] AC-5: Fehlertolerante Fallback-Strategie und saubere Fehlerbehandlung für Umgebungen ohne `/proc` Filesystem absichern
- [ ] AC-6: `spin.toml` konfigurieren, Integrationstests erstellen und WASM-Kompilierung für `wasm32-wasip2` validieren
