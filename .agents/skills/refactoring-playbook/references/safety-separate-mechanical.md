---
title: Separate Mechanical Edits From Behavior-Changing Edits at the Commit Boundary
impact: CRITICAL
tags: safety, tangled-commits, floss-refactoring, bisect
---

## Separate Mechanical Edits From Behavior-Changing Edits at the Commit Boundary

Tangled commits are an empirically-quantified anti-pattern. Murphy-Hill, Parnin, & Black (ICSE 2009, "How We Refactor, and How We Know It", drawn from datasets covering 13K+ developers and 240K+ tool-assisted refactorings) introduced the term *floss refactoring* for restructuring interleaved with feature/bug edits, and document that developers routinely omit refactoring intent from commit logs; Herzig & Zeller (EMSE) report 7–20% of bug-fixing commits across five Java open-source projects are tangled and demonstrate that untangling improves regression-prediction accuracy by 5–200% with 16.6% of source files otherwise incorrectly associated with bug reports; Paixão et al. (MSR 2020) reproduce the pattern in modern code review, where mixed-intent sequences inflate reviewer effort and defeat `git bisect`-based blame attribution. Enforce a strict commit boundary between mechanical refactoring (codemod-applied rename / move / extract with no logic change) and behavior-changing edits so each commit has single responsibility, single failure mode, and single-revert semantics.

**Incorrect (one commit conflates a behavior fix with a 30-file rename — bisect lands on a tangled diff):**

```bash
git log -1 --stat HEAD
# commit a1b2c3d
# Author: dev
#
#     fix off-by-one in pagination + rename Cart -> ShoppingCart
#
#  30 files changed, 412 insertions(+), 408 deletions(-)

git bisect bad a1b2c3d
# the regression is in 1 of 30 files; the diff hides which
```

**Correct (mechanical rename and behavior fix in disjoint commits — bisect identifies the right intent):**

```bash
git log --oneline -2
# b2c3d4e  fix: off-by-one in pagination (api/orders.ts:142)
# a1b2c3d  refactor: rename Cart -> ShoppingCart (codemod-applied, no logic change)

git bisect start b2c3d4e a1b2c3d
# a1b2c3d is mechanical-only; bisect attributes the regression to b2c3d4e
git bisect bad
# Bisecting: 0 revisions left to test after this
# the failing intent is identified, not just a 30-file haystack
```

References:
- [Murphy-Hill, Parnin, & Black (2009) — How We Refactor, and How We Know It, ICSE](https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=3afed538ca9d8fe84c71ae6af4fc987965474082)
- [Herzig & Zeller — The Impact of Tangled Code Changes (EMSE)](https://link.springer.com/article/10.1007/s10664-015-9376-6)
- [Paixão et al. (2020) — Behind the Intents, MSR](http://www0.cs.ucl.ac.uk/staff/jkrinke/publications/msr20intent.pdf)
