# Contributing

Thanks for contributing.

## Principles

- **Coordination correctness > feature velocity**
- **Small changes** are preferred
- **Backwards compatibility** is a default expectation
- Presets must not change core invariants (small batches, intent as executable, merge safety)

## How to contribute

1. Open an issue describing the problem and proposed change.
2. Keep PRs small and focused.
3. Include tests where behavior changes.
4. Follow the PR template.

## Adding a new stack preset

A preset is a set of repo files (CI + scripts + instructions) that:

- provides fast feedback in CI
- enforces formatting/lint
- includes at least one integration/contract test strategy

A preset must **not**:

- weaken review/merge safety
- bypass required checks
- expand scope into a new platform

## Maintainer review

Maintainers will primarily review:

- clarity and safety of workflows
- testability
- minimalism

Maintainers will likely reject:

- large multi-topic PRs
- stack-specific complexity that doesn’t generalize
- anything that turns this into a dashboard-first product
