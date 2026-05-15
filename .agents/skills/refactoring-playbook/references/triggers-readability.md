---
title: Readability Improvements Are Evidence-Supported Refactor Goals
impact: HIGH
tags: triggers, readability, comprehension
---

## Readability Improvements Are Evidence-Supported Refactor Goals

Code readability is empirically measurable and correlates with downstream maintenance outcomes. Buse & Weimer's TSE 2010 learned readability metric (trained on 120 human annotators, evaluated across 2.2M+ LOC of multi-release codebases) achieves ~80% prediction accuracy versus human judgment and exhibits statistically significant correlation with code-change rate, automated defect reports, and defect-log messages; Scalabrino et al.'s comprehensive readability model (JSEP 2018) — calibrated against 5K+ human evaluators on 600+ snippets — combines structural and textual features to further improve classification accuracy; Daka et al. (ESEC/FSE 2015) demonstrate readability-targeted transformation can preserve coverage. Extract Method, Rename, and Inline are therefore evidence-supported transformations when the goal is reducing time-to-comprehension on the next reader's hot path, not minimizing line count.

**Incorrect (extraction adds indirection without naming intermediate concepts):**

```ts
function priceFor(o: Order) {
  return calc(o.items, o.discount)
}

function calc(items: Item[], d: number) {
  return items.reduce((a, it) => a + it.price * it.qty, 0) * (1 - d)
}
```

**Correct (Extract Method names the intermediate concepts the reader needs):**

```ts
function priceFor(order: Order): Money {
  const subtotal = sumLineItems(order.items)
  return applyPercentDiscount(subtotal, order.discount)
}

function sumLineItems(items: Item[]): Money {
  return items.reduce((acc, item) => acc + item.price * item.qty, 0)
}

function applyPercentDiscount(amount: Money, discountFraction: number): Money {
  return amount * (1 - discountFraction)
}
```

References:
- [Buse & Weimer (2010) — Learning a Metric for Code Readability, IEEE TSE](https://web.eecs.umich.edu/~weimerw/p/weimer-tse2010-readability-preprint.pdf)
- [Scalabrino et al. (2018) — A Comprehensive Model for Code Readability, JSEP](https://sscalabrino.github.io/files/2018/JSEP2018AComprehensiveModel.pdf)
- [Daka, Campos, Fraser, Dorn, Weimer (2015) — Modeling Readability to Improve Unit Tests, ESEC/FSE](https://web.eecs.umich.edu/~weimerw/p/p107-daka.pdf)
