---
title: Refactoring Preserves Observable Behavior
impact: CRITICAL
tags: triggers, behavior-preservation, observational-equivalence
---

## Refactoring Preserves Observable Behavior

Foundational refactoring theory — Opdyke (1992), Mens & Tourwé (TSE 2004), Fowler (2018) — converges on a single invariant: the transformation must preserve observational equivalence on the input/output relation, so every externally visible response is indistinguishable pre- and post-transformation provided the refactoring's preconditions hold. Commits that bundle structural change with semantic change defeat `git bisect` blame attribution (each hunk carries dual responsibility for any regression) and inflate review effort; Paixão et al.'s MSR 2020 study of 1,780 reviewed code changes across six systems found that mixed-intent refactoring sequences crosscut multiple code elements and disproportionately drive review turn-over. Treat behavior-preserving and behavior-changing edits as disjoint commits with separate test gates.

**Incorrect (a single commit conflates Extract Method with two new validation guards):**

```ts
// commit: "refactor: extract OrderService"
function placeOrder(order: Order): Receipt {
  if (order.items.length === 0) throw new EmptyOrderError()   // new guard
  if (order.total < 0)          throw new InvalidTotalError() // new guard
  return orderService.submit(order)                            // pure extraction
}
```

**Correct (two commits with disjoint intent and independent test coverage):**

```ts
// commit 1: "refactor: extract OrderService.submit" — observationally equivalent
function placeOrder(order: Order): Receipt {
  return orderService.submit(order)
}

// commit 2: "feat: reject empty and negative-total orders" — behavior change
function placeOrder(order: Order): Receipt {
  if (order.items.length === 0) throw new EmptyOrderError()
  if (order.total < 0)          throw new InvalidTotalError()
  return orderService.submit(order)
}
```

References:
- [Opdyke (1992) — Refactoring Object-Oriented Frameworks (PhD thesis)](https://www.laputan.org/pub/papers/opdyke-thesis.pdf)
- [Mens & Tourwé (2004) — A Survey of Software Refactoring, IEEE TSE](https://ieeexplore.ieee.org/document/1265817)
- [Fowler (2018) — Refactoring: Improving the Design of Existing Code, 2nd ed.](https://martinfowler.com/books/refactoring.html)
- [Paixão et al. (2020) — Behind the Intents, MSR](http://www0.cs.ucl.ac.uk/staff/jkrinke/publications/msr20intent.pdf)
