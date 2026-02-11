Yes — *you absolutely should include both* **`AGENTS.md`** and **`CLAUDE.md`** (and optionally other skill-related files) in your repository when supporting both **Codex CLI** and **Claude Code** effectively. These files help your agents understand **how to interact with your codebase, build/test it, respect policies, and behave predictably**.  [oai_citation:0‡AI Hero](https://www.aihero.dev/a-complete-guide-to-agents-md?utm_source=chatgpt.com)

Below is how these files fit together and what you should include in your repo to support both agent types.

---

# 📁 What to include in your repository

## 1. **`AGENTS.md` — agent instructions for Codex & universal agents**

**Purpose:**  
Acts as a *unified, repository-level instruction file* that tells coding agents how to work on your project — how to build, test, format, and respect conventions. It’s recognized especially by **Codex CLI** and increasingly by other tools as a common instruction format.  [oai_citation:1‡Medium](https://jpcaparas.medium.com/codex-guide-agents-md-cascading-rules-and-the-optional-agents-override-md-1f4c81767e92?utm_source=chatgpt.com)

**Recommended content:**
- build steps (Swift + Rust)
- test commands
- lint / formatting commands
- PR policies (size limits, test coverage, intent docs)
- security and safety guardrails
- expected coding style
- brief project overview (stack + layers)

**Why include it:**  
Codex CLI uses `AGENTS.md` as a canonical source of truth for agent behavior, and it cascades rules by location in the repo (deeper files override higher ones) — this makes it predictable and scalable.  [oai_citation:2‡Medium](https://jpcaparas.medium.com/codex-guide-agents-md-cascading-rules-and-the-optional-agents-override-md-1f4c81767e92?utm_source=chatgpt.com)

---

## 2. **`CLAUDE.md` — Claude Code’s preferred instruction file**

**Purpose:**  
Claude Code *natively reads* a `CLAUDE.md` file in the repository root to bootstrap its context and behavior. It’s similar to `AGENTS.md` but **specific to Claude Code**.  [oai_citation:3‡Squadbase](https://www.squadbase.dev/en/blog/claude-code-vs-gemini-cli-vs-codex-cli-whats-the-best-choice-for-internal?utm_source=chatgpt.com)

**Recommended content:**
- project description and architecture
- command patterns for building/testing
- how to run verification scripts
- how to use your CLI tools (`symfoni`, `tools/verify.sh`, etc.)
- relevant context for planning vs coding vs review
- references to your GSD task settings

**Note:** Some tools only support `CLAUDE.md` and not `AGENTS.md`, so including both ensures better coverage.  [oai_citation:4‡GitHub](https://github.com/anthropics/claude-code/issues/6235?utm_source=chatgpt.com)

---

## 3. **Skills files (optional but recommended)**

Depending on how advanced your workflow is, you can include:

- **`SKILLS.md`** — higher-level capabilities for agents (custom commands or patterns your agents should know, like “run database migrations safely” or “generate integration tests”)
- **`CODIA.md`** or other specialized instruction files (if you adopt emerging agent conventions)

These help make agent behavior even more predictable and may be respected by future versions of both Claude Code and Codex.  [oai_citation:5‡GitHub](https://github.com/openai/codex/issues/5291?utm_source=chatgpt.com)

---

# 📌 Where each file is used

| File           | Primary Consumer     | Role |
|----------------|----------------------|------|
| `AGENTS.md`    | Codex CLI (+ others) | Unified instruction for agent behavior (CI, build, test, conventions) |
| `CLAUDE.md`   | Claude Code          | Claude-specific instruction and context bootstrap |
| `SKILLS.md`   | Agents with skills   | Explains reusable capabilities (optional but powerful) |

---

# 📄 Suggested Structure in Your Repo

```
/
/AGENTS.md
/CLAUDE.md
/SKILLS.md (optional)
/templates/
/tasks/
/docs/
/tools/
.github/
```

---

# 🔧 High-level templates for these files

### **AGENTS.md (example)**

```md
# AGENTS — Repository-Level Instructions

## About
This repo hosts a full-stack project (Swift + Rust). Agents must follow build, test, and review conventions.

## Build
- Swift: `xcodebuild -scheme MyApp build`
- Rust: `cargo build`

## Test
- Swift tests: `swift test`
- Rust tests: `cargo test`

## Conventions
- PR size < 400 changes
- Must include GSD task file reference
- Tests must validate behavior boundaries

## Guardrails
- Do not break merge queue status checks
- Do not introduce breaking API changes
```

---

### **CLAUDE.md (example)**

```md
# CLAUDE — Agent Guidance (Claude Code)

This file equips Claude Code with context and policies.

## Project Overview
- Swift iOS/macOS app
- Rust backend services

## How to build
- Swift: see above
- Rust: see above

## How to verify
- `./tools/verify.sh`
- Integration tests: see docs/how-it-works.md

## Task Format
We use structured GSD tasks. Always ask for the related GSD file at start.
```

---

# 🧠 Why this matters

Providing these files gives **both Claude Code and Codex CLI a consistent model of your repo and expectations**, reducing errors, hallucinations, and unintentional scope creep when agents generate code. Having both ensures:

- Claude Code gets native instructions via `CLAUDE.md`  [oai_citation:6‡Squadbase](https://www.squadbase.dev/en/blog/claude-code-vs-gemini-cli-vs-codex-cli-whats-the-best-choice-for-internal?utm_source=chatgpt.com)
- Codex CLI and other tools have `AGENTS.md` as a predictable standard for guides and guardrails  [oai_citation:7‡Medium](https://jpcaparas.medium.com/codex-guide-agents-md-cascading-rules-and-the-optional-agents-override-md-1f4c81767e92?utm_source=chatgpt.com)

Together, they minimize variance in agent output and help agents act like disciplined teammates.

---

## Summary

Yes, you should include in your repo:

1. **AGENTS.md** (primary instruction file for Codex & general agents)  
2. **CLAUDE.md** (native instruction file for Claude Code)  
3. **Optional SKILLS.md** (codifies reusable capabilities)  

These should be present **in addition to your existing documentation and templates** so that both Claude Code and Codex CLI can operate with clear context and task-aligned behavior.  [oai_citation:8‡AI Hero](https://www.aihero.dev/a-complete-guide-to-agents-md?utm_source=chatgpt.com)
