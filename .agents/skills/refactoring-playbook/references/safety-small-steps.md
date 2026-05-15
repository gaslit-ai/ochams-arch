---
title: Many Small Behavior-Preserving Transformations Beat One Large Redesign
impact: CRITICAL
tags: safety, atomic-steps, baby-steps, characterization
---

## Many Small Behavior-Preserving Transformations Beat One Large Redesign

Atomic-step discipline is the structural risk control across the refactoring canon. Opdyke (1992) formalized refactoring as sequences of atomic transformations gated by checkable preconditions; Fowler (2018) operationalizes this as "a series of small behavior-preserving transformations" with the system kept fully working after each step; Beck's *Test-Driven Development: By Example* (2003) targets 1–10-minute red-green-refactor cycles ("baby steps") and treats cycle duration as a step-size diagnostic; Feathers's *Working Effectively with Legacy Code* (2004) prescribes a smallest-safe-change algorithm — identify change point, find test point, break dependencies, characterization-test, refactor — as the only reliable traversal through untrusted code. Cedrim et al.'s longitudinal study (ESEC/FSE 2017, 16,566 refactorings across 23 projects) found 33.3% of refactorings induced new smells, with >95% of those smells persisting in subsequent commits, reinforcing that composite refactorings must be staged as independently-green atomic steps, not bundled into one tangled diff.

**Incorrect (one PR conflates rename + base-class change + method reorder; long-lived branch, painful merge):**

```text
git diff --stat origin/main..refactor/cart-overhaul
 src/cart/CartService.ts      | 412 ++++++++++++++++-----------
 src/cart/ICart.ts            |  84 +++---
 src/cart/index.ts            |  12 +-
 ...
 12 files changed, 612 insertions(+), 318 deletions(-)
 branch age: 11 days, 4 force-pushes, 2 merge conflicts resolved
```

**Correct (three short-lived branches, each behavior-preserving, each independently green):**

```bash
# step 1: pure rename, codemod-applied — no semantic change
git checkout -b refactor/rename-cart-service && \
  pnpm codemod rename-symbol --from Cart --to ShoppingCart && \
  pnpm test && git commit -am "refactor: rename Cart -> ShoppingCart" && \
  gh pr create --title "refactor: rename Cart -> ShoppingCart" && \
  gh pr merge --squash

# step 2: change base class — preconditions checked, characterization tests in place
git checkout -b refactor/cart-extends-domain-entity && \
  pnpm test && git commit -am "refactor: ShoppingCart extends DomainEntity" && \
  gh pr merge --squash

# step 3: method reorder to match interface declaration order — mechanical
git checkout -b refactor/order-cart-methods && \
  pnpm test && git commit -am "refactor: reorder methods to match interface" && \
  gh pr merge --squash
```

References:
- [Opdyke (1992) — Refactoring Object-Oriented Frameworks (PhD thesis)](https://www.laputan.org/pub/papers/opdyke-thesis.pdf)
- [Fowler (2018) — Refactoring: Improving the Design of Existing Code, 2nd ed.](https://martinfowler.com/books/refactoring.html)
- [Beck (2003) — Test-Driven Development: By Example](https://www.oreilly.com/library/view/test-driven-development/0321146530/)
- [Feathers (2004) — Working Effectively with Legacy Code](https://www.oreilly.com/library/view/working-effectively-with/0131177052/)
- [Cedrim et al. (2017) — Understanding the Impact of Refactoring on Smells, ESEC/FSE](https://dl.acm.org/doi/10.1145/3106237.3106259)
