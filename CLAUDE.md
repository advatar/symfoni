# CLAUDE — Agent Guidance (Claude Code)

Follow `AGENTS.md` for repo-wide rules. This file is a quick index and command crib sheet.

## Where to look
- Intent/spec: `tasks/*.gsd.md` (GSD tasks)
- Process docs: `docs/how-it-works.md`, `docs/get-shit-done.md`
- Verification scripts: `tools/verify.sh`, `tools/format.sh`
- Codex rules (if present): `.codex/repo-instructions.md`

## Common commands
- Verify: `tools/verify.sh`
- Format Rust: `tools/format.sh`
- Rust tests only: `cargo test`
- Swift build/test example (adjust scheme/destination):
  - `xcodebuild -scheme "<Scheme>" -destination 'platform=macOS' build`
  - `xcodebuild -scheme "<Scheme>" -destination 'platform=macOS' test`

