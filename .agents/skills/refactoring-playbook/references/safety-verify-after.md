---
title: Rerun Tests, Type-Check, and Static Analysis After Every Refactor Batch
impact: CRITICAL
tags: safety, verification, differential-testing, behavior-preservation
---

## Rerun Tests, Type-Check, and Static Analysis After Every Refactor Batch

Post-transformation verification is empirically mandatory because even semantically-aware refactoring engines ship behavior-altering defects. AlOmar et al.'s systematic mapping study (Information and Software Technology 2021, 28 surveyed behavior-preservation approaches) concludes that no single preservation tactic is sufficient across all refactoring types and that several refactoring classes remain under-researched; Gligoric et al. (ECOOP 2013) systematically tested Eclipse's Java and C refactoring engines on real projects and reported 77 newly-discovered Java bugs and 43 C bugs; Soares, Gheyi, & Massoni's SafeRefactor (TSE 2012) uncovered >100 behavior-changing defects across Eclipse JDT, NetBeans, and JastAdd by generating differential test suites across the pre- and post-transformation programs; Wang et al.'s RefactorBench (2024) extends the finding with 518 categorized refactoring-engine bug reports. Treat the verification pipeline (`pnpm test` + `pnpm tsc --noEmit` + project static analysis) as the gating step of the refactor batch, not as a downstream CI safety net.

**Incorrect (no local verification — CI carries the cost of broken imports and blocks other PRs):**

```bash
# IDE rename + save + commit + push
git commit -am "refactor: rename"
git push
# remote CI fails 11 minutes later; the failing import blocks 3 unrelated PRs
# the refactor is rolled back; the wasted CI minutes are gone
```

**Correct (local verification gate runs the full triad before push):**

```bash
# verification triad: behavior, types, lint
pnpm test && pnpm tsc --noEmit && pnpm lint --max-warnings 0
# 142 passed, 0 failed
# 0 type errors
# 0 lint warnings
git commit -am "refactor: rename OrderService.process -> OrderService.submit"
git push   # CI confirms what we already verified locally
```

References:
- [AlOmar, Mkaouer, Newman, Ouni (2021) — On Preserving the Behavior in Software Refactoring: A Systematic Mapping Study, IST](https://arxiv.org/abs/2106.13900)
- [Gligoric et al. (2013) — Systematic Testing of Refactoring Engines on Real Software Projects, ECOOP](https://users.ece.utexas.edu/~gligoric/papers/GligoricETAL13RTR.pdf)
- [Soares, Gheyi, Massoni (2012) — Automated Behavioral Testing of Refactoring Engines, IEEE TSE (SafeRefactor)](https://www.microsoft.com/en-us/research/wp-content/uploads/2020/08/tse12.pdf)
- [Wang et al. (2024) — An Empirical Study of Refactoring Engine Bugs (RefactorBench)](https://arxiv.org/abs/2409.14610)
