# Sections

This file defines section ordering, impact, and reference filename prefixes.

---

## 1. Triggers — When and Why to Refactor (triggers)

**Impact:** CRITICAL
**Description:** Refactoring without a clear trigger costs effort and creates merge cost without buying back maintenance time. Mining studies show real refactoring is driven by concrete pain — change-frequency, comprehension difficulty, review feedback, or impending API change — not by aesthetic preference.

## 2. Safety — How to Refactor Without Breaking Behavior (safety)

**Impact:** CRITICAL
**Description:** Refactoring is defined by behavior preservation. Most refactor-induced bugs trace to one of four failures: oversized step, weak regression net, mechanical edits tangled with behavior changes, or no verification after the change. Treat all four as non-negotiable.

## 3. Review — Collaboration and Merge Discipline (review)

**Impact:** HIGH
**Description:** Refactoring PRs carry measurably higher churn, longer merge latency, and more review discussion than other PRs. Discipline at the PR boundary — small scope, explicit intent, narrated commits — is what keeps that overhead bounded.

## 4. Automation — Tools, Codemods, and LLM Assistance (automation)

**Impact:** HIGH
**Description:** Automation pays off for repetitive, semantically analyzable change at repository scale. It is unreliable for context-heavy design decisions. Pick the lightest tool that understands the language semantics, and never let LLM speed remove human oversight.

## 5. Measurement — Judging Refactor Success (measure)

**Impact:** MEDIUM
**Description:** No single metric captures refactor value. Smell counts drop while behavior breaks; readability improves while review cost spikes. Use a portfolio of indicators and tie success to future change cost, not immediate style conformance.
