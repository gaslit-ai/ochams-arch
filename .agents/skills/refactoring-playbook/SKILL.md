---
name: refactoring-playbook
description: Evidence-based refactoring playbook distilled from empirical software engineering research. Use when planning, executing, or reviewing refactoring work — when to refactor, how to refactor safely, how to use automation and LLM assistance, and how to measure success. Triggers on tasks framed as "refactor", "clean up", "extract method", "rename", "migrate", "modernize", or "reduce tech debt".
license: MIT
allowed-tools: Bash(scripts/hotspots.sh:*) Bash(hotspot:*) Bash(git:*) Bash(jq:*)
metadata:
  author: duke
  version: "0.1.0"
---

# Refactoring Playbook

Evidence-based guidance for refactoring code bases. Grounded in primary studies from ACM/IEEE venues, industrial reports, and recent AI-assisted refactoring evaluations.

Each reference below is a separate file in `references/`; the agent loads only the references it needs for the current task. The fully expanded long-form guide (with bad/good code examples inlined) lives in `AGENTS.md` for tools that follow the [AGENTS.md convention](https://agents.md) rather than the [Agent Skills spec](https://agentskills.io/specification).

## References

### 1. Triggers — When and Why to Refactor — **CRITICAL**

Refactoring without a clear trigger costs effort and creates merge cost without buying back maintenance time. Mining studies show real refactoring is driven by concrete pain — change-frequency, comprehension difficulty, review feedback, or impending API change — not by aesthetic preference.

- [`triggers-hot-files`](./references/triggers-hot-files.md) — Prioritize Files With High Relative Churn (Top-Decile Hotspots)
- [`triggers-readability`](./references/triggers-readability.md) — Readability Improvements Are Evidence-Supported Refactor Goals
- [`triggers-behavior-preserving`](./references/triggers-behavior-preserving.md) — Refactoring Preserves Observable Behavior
- [`triggers-pain-driven`](./references/triggers-pain-driven.md) — Start From Concrete Maintenance Pain, Not Aesthetic Preference

### 2. Safety — How to Refactor Without Breaking Behavior — **CRITICAL**

Refactoring is defined by behavior preservation. Most refactor-induced bugs trace to one of four failures: oversized step, weak regression net, mechanical edits tangled with behavior changes, or no verification after the change. Treat all four as non-negotiable.

- [`safety-small-steps`](./references/safety-small-steps.md) — Many Small Behavior-Preserving Transformations Beat One Large Redesign
- [`safety-verify-after`](./references/safety-verify-after.md) — Rerun Tests, Type-Check, and Static Analysis After Every Refactor Batch
- [`safety-separate-mechanical`](./references/safety-separate-mechanical.md) — Separate Mechanical Edits From Behavior-Changing Edits at the Commit Boundary
- [`safety-tests-first`](./references/safety-tests-first.md) — Strengthen the Regression Net Before Risky Structural Change

### 3. Review — Collaboration and Merge Discipline — **HIGH**

Refactoring PRs carry measurably higher churn, longer merge latency, and more review discussion than other PRs. Discipline at the PR boundary — small scope, explicit intent, narrated commits — is what keeps that overhead bounded.

- [`review-intent-commits`](./references/review-intent-commits.md) — Commit Messages Encode Refactor Intent, Not Just "Cleanup
- [`review-small-prs`](./references/review-small-prs.md) — Keep Refactor PRs Small and Short-Lived

### 4. Automation — Tools, Codemods, and LLM Assistance — **HIGH**

Automation pays off for repetitive, semantically analyzable change at repository scale. It is unreliable for context-heavy design decisions. Pick the lightest tool that understands the language semantics, and never let LLM speed remove human oversight.

- [`automation-keep-oversight`](./references/automation-keep-oversight.md) — LLM Migration Tools Accelerate Mechanics; Per-Patch Oversight Stays In-Loop
- [`automation-llm-local`](./references/automation-llm-local.md) — LLM-Assisted Refactoring Is Strongest on Local, Mechanically Scoped Transformations
- [`automation-semantic-tools`](./references/automation-semantic-tools.md) — Prefer Semantic-Aware Codemods Over Regex Search-and-Replace

### 5. Measurement — Judging Refactor Success — **MEDIUM**

No single metric captures refactor value. Smell counts drop while behavior breaks; readability improves while review cost spikes. Use a portfolio of indicators and tie success to future change cost, not immediate style conformance.

- [`measure-portfolio`](./references/measure-portfolio.md) — Judge Refactor Success With a Multi-Dimensional Portfolio of Indicators
- [`measure-smell-not-safety`](./references/measure-smell-not-safety.md) — Smell Reduction and Static-Analysis Clean Are Not Behavior-Preservation Oracles

## Full Compiled Document

For the complete guide with every reference expanded inline (bad/good examples, citations, prerequisites), see [`AGENTS.md`](./AGENTS.md). It is the [AGENTS.md-convention](https://agents.md) fallback for tools that do not load individual references on demand.
