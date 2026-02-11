# AI Coordination Toolkit

A **CLI-first toolkit** for coordinating humans and AI coding agents without losing velocity to merge conflicts, review overload, or intent drift.

AI makes writing code fast — but **coordination, verification, and integration** become the bottleneck. This toolkit preserves speedups by installing repo-level standards and automation:

- small, vertical slices
- executable intent (tests + contracts)
- merge-queue-friendly workflows
- agent instruction files (Codex CLI / Claude Code)
- boring, auditable defaults

## Who this is for

- Teams using AI coding agents (Codex CLI, Claude Code, GitHub agents)
- Full-stack teams shipping frontend + backend + persistence
- Teams that want higher throughput **without** lower stability

## What this toolkit does

It installs coordination primitives into a repository:

- `ai-coord` CLI to bootstrap/configure repos
- GitHub Actions workflows (Swift + Rust preset)
- PR + issue templates
- local verification scripts
- optional MCP server stub (tool bridge)

It does **not** replace GitHub, CI/CD, or human judgment.

## Quick start

```bash
cargo install --path crates/ai_coord_cli

cd /path/to/your/repo
ai-coord init --preset swift-rust
```

## Works with

- Codex CLI (via `.codex/repo-instructions.md` and/or `AGENTS.md`)
- Claude Code (via `CLAUDE.md`)

## Why CLI-first

- works in CI and headless environments
- versionable and auditable
- portable across stacks

A macOS app can exist later as a **dashboard**, but the CLI is the source of truth.

## Docs

- `docs/how-it-works.md`
- `docs/get-shit-done.md`
- `docs/onboarding/`
- `docs/common-failure-modes.md`

## License

MIT
