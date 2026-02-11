# AGENTS — Repository-Level Instructions

This repo uses the Symfoni conventions (GSD tasks + verification scripts) to keep AI work auditable and merge-queue-friendly.

## Source of Truth
- Work from a GSD task file under `tasks/` (or one linked from the issue/PR).
- Do not invent scope. Implement only what the task's Acceptance Criteria requires.

## Workflow
- Prefer small, vertical slices (code + tests + docs as needed).
- Keep PRs small (target <= ~400 changed lines excluding generated files).
- Use feature flags when a change is risky or user-visible.

## Verification
- Run `tools/verify.sh` before handing off for review.
- Run `tools/format.sh` (or `cargo fmt`) before committing Rust changes.

`tools/verify.sh` runs:
- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `xcodebuild -version`

## Swift Notes
If this repo contains Swift/Xcode projects, customize:
- `.github/workflows/swift.yml` scheme(s) and destinations
- `tools/verify.sh` to build/test the correct scheme(s)

## PR Hygiene
- Fill in `.github/pull_request_template.md` (Intent, Behavior delta, Verification, Rollout).

