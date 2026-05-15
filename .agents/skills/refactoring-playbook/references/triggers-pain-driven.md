---
title: Start From Concrete Maintenance Pain, Not Aesthetic Preference
impact: CRITICAL
tags: triggers, motivation, prioritization
---

## Start From Concrete Maintenance Pain, Not Aesthetic Preference

Empirical motivation taxonomies converge on a non-aesthetic origin for refactoring. Silva, Tsantalis, & Valente (FSE 2016, ACM SIGSOFT Distinguished Paper) catalogued 44 motivations across 12 refactoring types and conclude refactoring is "mainly driven by changes in the requirements and much less by code smells"; Pantiuchina et al. (TOSEM 2020) corroborate this on 287,813 mined refactoring operations across 150 systems with 1,223 hand-coded motivation tags showing feature-, readability-, and review-driven intent dominate smell-driven intent; Kim, Zimmermann, & Nagappan (TSE 2014) report the same pattern at Microsoft via survey, semi-structured interview, and Windows 7 version-history triangulation. A refactor without a stated, concrete pain — a feature blocked by an awkward seam, a hotspot collision in modern code review, an explicit reviewer ask — is speculative and consistently loses priority competition against feature work.

**Incorrect (refactor with no stated pain — high probability of deferral or rejection):**

```text
PR #482  refactor: clean up utils/format.ts
---
Description:
Renamed a few variables, split a function. The file looked dated.
```

**Correct (refactor anchored to a measured, recurring pain signal):**

```text
PR #482  refactor: extract parseCurrency from utils/format.ts
---
Motivation (process signal):
  - utils/format.ts modified in 14 of the last 50 commits
  - 3 of last 4 feature PRs touching it carried a rounding regression
    (#471, #475, #479)
  - 2 reviewer comments explicitly asked to "split the parser out"
Hypothesis: extracting parseCurrency localizes rounding logic, so
the next feature PR becomes a single-file change.
```

References:
- [Silva, Tsantalis, & Valente (2016) — Why We Refactor? Confessions of GitHub Contributors, FSE](https://users.encs.concordia.ca/~nikolaos/publications/FSE_2016.pdf)
- [Pantiuchina et al. (2020) — Why Developers Refactor Source Code, TOSEM](https://arxiv.org/pdf/2101.01430)
- [Kim, Zimmermann, & Nagappan (2014) — Refactoring Challenges and Benefits at Microsoft, IEEE TSE](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/kim-tse-2014.pdf)
