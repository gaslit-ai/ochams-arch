---
title: Prioritize Files With High Relative Churn (Top-Decile Hotspots)
impact: CRITICAL
tags: triggers, churn, hotspots, defect-prediction
---

## Prioritize Files With High Relative Churn (Top-Decile Hotspots)

Relative-churn is among the most reliably-validated process metrics for prioritizing maintenance. Nagappan & Ball's ICSE 2005 logistic-regression study attained 89.0% classification accuracy discriminating fault-prone from non-fault-prone binaries on Windows Server 2003 using relative-churn predictors; Pantiuchina et al. (TOSEM 2020) mined 287,813 refactoring operations across 150 systems and found prior-change frequency is a strong correlate of where developer-initiated refactoring concentrates; behavioral-code-analysis (Tornhill, *Your Code as a Crime Scene*) further quantifies a Pareto regime where ~1–2% of files typically account for ~70% of edit activity, with defects clustering in the same subset. The merge-cost of restructuring a rarely-edited file rarely amortizes; restructuring a top-decile hotspot pays back on the next feature touching it.

**Incorrect (refactoring a low-churn legacy helper — defect risk and ROI are both negligible):**

```bash
# legacy/format-1999.ts: 1 commit in 24 months
git log --since="24 months ago" --pretty=oneline -- legacy/format-1999.ts
# a1b2c3d  fix typo in comment

# yet the file gets a 400-line "modernization" PR
```

**Correct (invoke the bundled `scripts/hotspots.sh`; thin wrapper around [`hotspot`](https://github.com/huangsam/hotspot) implementing Tornhill ROI scoring with the empirical 90-day window):**

```bash
# JSON to stdout, status to stderr — pipeable into jq, dashboards, CI.
scripts/hotspots.sh . --limit 10 --mode roi --since 90d \
  | jq -r '.results[] | "\(.rank)\t\(.score | round)\t\(.churn)\t\(.lines_of_code)\t\(.path)"'
# 1   824   18   240   api/orders.ts          <-- top-decile refactor target
# 2   612   12   180   api/payments.ts
# 3   504    9   140   web/checkout.tsx
# 4    24    2    12   legacy/format-1999.ts

# Refactor priority list = top-decile rows; refactor in score-descending order.
# Prerequisite: `go install github.com/huangsam/hotspot@latest` (script
# exits with install instructions if `hotspot` is not on PATH).
```

References:
- [Nagappan & Ball (2005) — Use of Relative Code Churn Measures to Predict System Defect Density, ICSE](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/icse05churn.pdf)
- [Pantiuchina et al. (2020) — Why Developers Refactor Source Code, TOSEM](https://arxiv.org/pdf/2101.01430)
- [Tornhill — Your Code as a Crime Scene (2nd ed., Pragmatic Bookshelf)](https://pragprog.com/titles/atcrime2/your-code-as-a-crime-scene-second-edition/)
