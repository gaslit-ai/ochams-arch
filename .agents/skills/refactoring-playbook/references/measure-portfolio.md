---
title: Judge Refactor Success With a Multi-Dimensional Portfolio of Indicators
impact: MEDIUM
tags: measure, indicators, success, portfolio
---

## Judge Refactor Success With a Multi-Dimensional Portfolio of Indicators

Single-metric refactor evaluation is empirically unreliable and biases toward perverse outcomes. Pantiuchina, Lanza, & Bavota (ICSME 2018, *Improving Code: The (Mis)Perception of Quality Metrics*) demonstrated that developer-perceived quality improvements frequently fail to manifest in standard metric deltas — and conversely metric improvements do not necessarily reflect perceived improvement — establishing that single-axis evaluation systematically diverges from outcome; the SPACE framework (Forsgren, Storey, Maddila, Zimmermann, Houck, Butler, ACM Queue / CACM 2021) operationalizes the multi-dimensional principle by mandating coverage of at least three of {Satisfaction, Performance, Activity, Communication, Efficiency} for any developer-productivity assessment; the DORA program (Forsgren, Humble, Kim, *Accelerate*, 2018) confirms the same property at delivery-performance level — elite performers simultaneously excel on deployment frequency, lead time, change failure rate, and MTTR. Decide success criteria up front using a small portfolio: behavior preservation, reviewability, comprehension, hotspot impact, and future change cost.

**Incorrect (single-metric declaration of success — vulnerable to misalignment with perceived outcome):**

```text
Refactor retro:
  smell count: 142 -> 78 (-45%)  ✓
  → ship it
```

**Correct (multi-dimensional portfolio with disjoint indicators, pre-committed thresholds):**

```text
Refactor retro: api/orders.ts hotspot extraction

  axis                      indicator                                  threshold   measured   verdict
  ----                      ---------                                  ---------   --------   -------
  behavior preservation     suite pass rate post-refactor              100%        100%       ✓
                            production incidents in window (14 d)      0           0          ✓
  reviewability             merge latency (PR open -> merged)          ≤ 1 day     6 h        ✓
                            rework loops (force-push count)            ≤ 1         0          ✓
  comprehension (SPACE-S)   new-reader walkthrough rating (n=3)        ≥ 4/5       4.3/5      ✓
  hotspot impact            churn rank over rolling 30 d               drop ≥ 3    #1 -> #6   ✓
  future change cost        next feature files-touched (vs baseline)   ≤ 50%       1 / 6      ✓

  → success criteria met across all axes; refactor accepted.
```

References:
- [Pantiuchina, Lanza, Bavota (2018) — Improving Code: The (Mis)Perception of Quality Metrics, ICSME](https://www.researchgate.net/publication/328905356_Improving_Code_The_Mis_Perception_of_Quality_Metrics)
- [Forsgren, Storey, Maddila, Zimmermann, Houck, Butler (2021) — The SPACE of Developer Productivity, ACM Queue / CACM](https://queue.acm.org/detail.cfm?id=3454124)
- [Forsgren, Humble, Kim (2018) — Accelerate: The Science of Lean Software and DevOps (DORA metrics)](https://itrevolution.com/product/accelerate/)
