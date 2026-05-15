---
title: LLM Migration Tools Accelerate Mechanics; Per-Patch Oversight Stays In-Loop
impact: HIGH
tags: automation, llm, migration, oversight, review
---

## LLM Migration Tools Accelerate Mechanics; Per-Patch Oversight Stays In-Loop

LLM-assisted code migration is empirically validated at industrial scale, but only with strict per-patch test gating and human review. Ziftci et al. (FSE 2025, "Migrating Code At Scale With LLMs At Google", 39 migrations executed by 3 developers over 12 months) report 595 code changes and 93,574 edits with LLMs generating 74.45% of changes and 69.46% of edits, alongside a developer-estimated 50% reduction in migration time — achieved while keeping developers in-loop on every accepted edit. Independent enterprise studies of GitHub Copilot (GitHub × Accenture; Zoominfo) report suggestion-acceptance rates of ~30–33% — i.e., the majority of LLM output is rejected before merge — and agentic-coding PR studies (567 PRs across 157 OSS projects) find Agentic-PRs accepted at lower rates than human-authored PRs, with high task-stratified variance (bug-fix acceptance 45.6%–83.0% across agents). Per-patch test execution plus standard code review are therefore non-negotiable: the speedup is real, but stripping the oversight loop converts it into a silent regression pipeline.

**Incorrect (bulk-apply 240 LLM patches, single commit, CI is the only oracle):**

```bash
# 240 LLM-generated patches applied in one transaction
git apply patches/*.diff
git add -A && git commit -m "migrate to v2 SDK"
git push

# CI fails 3 hours later on 14 files
# rolling back a 240-file commit invalidates all downstream branches
# bisect lands on a tangled diff that hides which intent failed
```

**Correct (per-patch test gate + per-patch commit + per-patch reviewable diff):**

```bash
# replicate the Google migration pattern: bounded scope, gated patches, human-in-loop
for patch in patches/*.diff; do
  git apply "$patch"

  # patch-scoped test gate (only the modules the patch touched)
  scope="$(scripts/scope-for-patch.sh "$patch")"
  pnpm test --filter "$scope" && pnpm tsc --noEmit -p tsconfig.$scope.json || {
    echo "rejected: $patch (failed scoped gate)"
    git checkout -- .
    continue
  }

  git add -A
  git commit -m "migrate($scope): apply $(basename "$patch" .diff) (LLM-drafted, gate-verified)"
done

# every commit is independently green, independently reviewable, independently revertable
gh pr create --title "migrate to v2 SDK (LLM-assisted, $(git rev-list --count HEAD ^main) gated commits)"
```

References:
- [Ziftci et al. (2025) — Migrating Code At Scale With LLMs At Google, FSE Industry](https://arxiv.org/abs/2504.09691)
- [Dakhel et al. (2023) — GitHub Copilot AI Pair Programmer: Asset or Liability? (empirical correctness of Copilot suggestions)](https://arxiv.org/abs/2206.15331)
- [Bhatia et al. (2024) — On the Use of Agentic Coding: An Empirical Study of Pull Requests on GitHub](https://arxiv.org/abs/2509.14745)
