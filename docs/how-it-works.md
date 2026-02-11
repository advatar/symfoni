# How it works

This toolkit prevents AI-driven coordination collapse by making intent explicit, keeping batches small, and automating merge safety.

## The pipeline

1. **GSD Task** defines intent (goal, constraints, acceptance criteria).
2. **Issue/PR templates** force traceability.
3. **Tests** make intent executable.
4. **Required checks** + **merge queue** keep `main` stable.

Humans own intent and risk judgment; machines enforce everything else.
