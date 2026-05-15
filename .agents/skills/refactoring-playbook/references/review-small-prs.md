---
title: Keep Refactor PRs Small and Short-Lived
impact: HIGH
tags: review, pr-size, churn, merge-latency
---

## Keep Refactor PRs Small and Short-Lived

Refactoring-bearing PRs measurably degrade review economics. Coelho, Tsantalis, Massoni, & Alves (ESEM 2021) compared refactoring-inducing PRs against control PRs and found statistically significant elevations in line-churn, file-fan-out, and discussion-comment volume in the refactoring set; Rigby & Bird (ESEC/FSE 2013), analyzing convergent code-review practices across Google (Android, Chromium OS), Microsoft (Bing, Office, MS SQL), AMD, and OSS, demonstrate that review-cycle count scales with initial patch size and added-line count, an effect that holds across organizational boundaries; Gousios, Pinzger, & van Deursen (ICSE 2014, GHTorrent corpus + 291-project sample) plus follow-up qualitative work identify PR churn (changed lines) as a primary determinant of merge latency along with project hotness and submitter track record. Empirical convergence: cap each refactor PR at one cohesive intent, ideally under ~400 LOC changed, with branch lifetime under one working day.

**Incorrect (one PR conflates five intents, long-lived branch, reviewer attrition):**

```text
PR #482  "refactor: cart cleanup"
  diff:       +2138 / -1907 across 47 files
  branch age: 14 days
  reviews:    23 comments, 4 merge-conflict resolutions, 1 reviewer disengaged
  outcome:    merged after rebase + force-push; review fatigue evident in final approvals
```

**Correct (intent-disjoint PR sequence, each independently mergeable in a working day):**

```text
PR #482  refactor: extract CartValidator              +180 / -120,  6 files, merged day 1
PR #483  refactor: rename Cart -> ShoppingCart        +402 / -402, 31 files, merged day 1
PR #484  refactor: collapse three pricing functions   +95  / -210,  4 files, merged day 2
PR #485  refactor: inline unused CartFactory          +0   / -85,   2 files, merged day 2
```

References:
- [Coelho, Tsantalis, Massoni, Alves (2021) — An Empirical Study on Refactoring-Inducing Pull Requests, ESEM](https://users.encs.concordia.ca/~nikolaos/publications/ESEM_2021.pdf)
- [Rigby & Bird (2013) — Convergent Contemporary Software Peer Review Practices, ESEC/FSE](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/rigby2013convergent.pdf)
- [Gousios, Pinzger, van Deursen (2014) — An Exploratory Study of the Pull-based Software Development Model, ICSE](https://www.gousios.org/pub/exploratory-study-pull-based.pdf)
