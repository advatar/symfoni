# Governance

This project is intentionally conservative.

## Roles

- **Maintainers**: merge PRs, cut releases, set direction.
- **Contributors**: propose and implement changes via PRs.

## Decision-making

- Default decision style: **maintainer consensus**.
- Breaking changes require a written rationale in the PR description and a release note.

## Compatibility policy

- Repo bootstrapping should be additive and idempotent.
- Presets may evolve, but core invariants must remain stable.

## Releases

- Tag releases with semantic versioning.
- Patch releases should not change behavior unexpectedly.
