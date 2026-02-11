# Common failure modes

## 1) Oversized PRs
- Symptom: 800–2000 line diffs, slow reviews
- Fix: split into vertical slices; enforce PR size budgets

## 2) Unit-test-only validation
- Symptom: code passes unit tests but breaks contracts/wiring
- Fix: require integration/contract tests for boundary changes

## 3) Bypassing merge queues
- Symptom: main breaks intermittently
- Fix: protect `main`, require checks, use merge queue

## 4) Agents inventing scope
- Symptom: surprise features in PRs
- Fix: no agent execution without a GSD task; enforce constraints
