---
title: Strengthen the Regression Net Before Risky Structural Change
impact: CRITICAL
tags: safety, characterization-tests, behavior-oracle
---

## Strengthen the Regression Net Before Risky Structural Change

The regression test suite is the only practical behavior-preservation oracle for refactoring, and its trustworthiness bounds the safety of every downstream transformation. Kim, Zimmermann, & Nagappan (Microsoft TSE 2014) identified inadequate test coverage as the most-cited blocker to refactoring across surveyed and interviewed engineers; Feathers (*Working Effectively with Legacy Code*, 2004) resolves the legacy-code circular dependency — to change safely you need tests, to add tests you often need small changes — via the characterization-test workflow (capture observed behavior, including latent bugs, before restructuring); Bavota et al.'s SCAM 2012 longitudinal analysis of Apache Ant, Xerces, and ArgoUML showed certain refactoring classes (notably hierarchy-involving operations) induce faults at elevated rates, so the regression net must explicitly cover the structures being touched; Wang et al.'s RefactorBench (2024, 518 categorized refactoring-engine bugs across Eclipse, IntelliJ IDEA, and NetBeans, with 130 newly-discovered transferable failures) further demonstrates that "trust the IDE refactor button" is no substitute for executing tests after every step.

**Incorrect (refactor lands first, suite never characterized the affected path — false-green is indistinguishable from coverage):**

```bash
git checkout -b refactor/extract-pricing
# 600 LOC moved across 8 files, no new tests
pnpm test
# 142 passed, 0 failed   ← suite never executed the rounding branch
git push   # silent regression ships
```

**Correct (characterization tests pin observed behavior, including quirks; refactor proceeds against verified-green):**

```bash
# step 1: pin current behavior — quirks included — via characterization tests
git checkout -b test/characterize-pricing
pnpm test -- --coverage src/pricing
# branch coverage on src/pricing/*.ts went 41% -> 92%
# 6 new tests document existing rounding and tax-edge behavior
git commit -am "test: characterize pricing module (no behavior change)"
gh pr merge --squash

# step 2: refactor against the now-trustworthy oracle
git checkout -b refactor/extract-pricing
pnpm test && pnpm tsc --noEmit
# 148 passed, 0 type errors — any red is a real regression, not unknown territory
```

References:
- [Kim, Zimmermann, & Nagappan (2014) — Refactoring Challenges and Benefits at Microsoft, IEEE TSE](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/kim-tse-2014.pdf)
- [Feathers (2004) — Working Effectively with Legacy Code](https://www.oreilly.com/library/view/working-effectively-with/0131177052/)
- [Bavota et al. (2012) — When Does a Refactoring Induce Bugs? An Empirical Study, SCAM](https://people.lu.usi.ch/bavotg/papers/scam2012.pdf)
- [Wang et al. (2024) — An Empirical Study of Refactoring Engine Bugs (RefactorBench)](https://arxiv.org/abs/2409.14610)
