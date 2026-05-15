# Refactoring Playbook

**Version 0.1.0**  
Duke Engineering  
May 2026

> **Note:**  
> This guide is optimized for agents and AI-assisted engineering workflows.
> It prioritizes deterministic behavior, explicit contracts, and maintainable defaults.

---

## Abstract

Evidence-based playbook for refactoring code bases. Synthesizes foundational refactoring theory (Opdyke, Fowler, Mens & Tourwé), industrial field studies at Microsoft and Google, modern code-review and mining studies, and recent LLM-assisted refactoring evaluations into a small set of high-leverage rules covering triggers, safety, review, automation, and measurement.

---

## Table of Contents

1. [Triggers — When and Why to Refactor](#1-triggers-—-when-and-why-to-refactor) — **CRITICAL**
   - 1.1 [Prioritize Files With High Relative Churn (Top-Decile Hotspots)](#11-prioritize-files-with-high-relative-churn-top-decile-hotspots)
   - 1.2 [Readability Improvements Are Evidence-Supported Refactor Goals](#12-readability-improvements-are-evidence-supported-refactor-goals)
   - 1.3 [Refactoring Preserves Observable Behavior](#13-refactoring-preserves-observable-behavior)
   - 1.4 [Start From Concrete Maintenance Pain, Not Aesthetic Preference](#14-start-from-concrete-maintenance-pain-not-aesthetic-preference)
2. [Safety — How to Refactor Without Breaking Behavior](#2-safety-—-how-to-refactor-without-breaking-behavior) — **CRITICAL**
   - 2.1 [Many Small Behavior-Preserving Transformations Beat One Large Redesign](#21-many-small-behavior-preserving-transformations-beat-one-large-redesign)
   - 2.2 [Rerun Tests, Type-Check, and Static Analysis After Every Refactor Batch](#22-rerun-tests-type-check-and-static-analysis-after-every-refactor-batch)
   - 2.3 [Separate Mechanical Edits From Behavior-Changing Edits at the Commit Boundary](#23-separate-mechanical-edits-from-behavior-changing-edits-at-the-commit-boundary)
   - 2.4 [Strengthen the Regression Net Before Risky Structural Change](#24-strengthen-the-regression-net-before-risky-structural-change)
3. [Review — Collaboration and Merge Discipline](#3-review-—-collaboration-and-merge-discipline) — **HIGH**
   - 3.1 [Commit Messages Encode Refactor Intent, Not Just "Cleanup](#31-commit-messages-encode-refactor-intent-not-just-cleanup)
   - 3.2 [Keep Refactor PRs Small and Short-Lived](#32-keep-refactor-prs-small-and-short-lived)
4. [Automation — Tools, Codemods, and LLM Assistance](#4-automation-—-tools,-codemods,-and-llm-assistance) — **HIGH**
   - 4.1 [LLM Migration Tools Accelerate Mechanics; Per-Patch Oversight Stays In-Loop](#41-llm-migration-tools-accelerate-mechanics-per-patch-oversight-stays-in-loop)
   - 4.2 [LLM-Assisted Refactoring Is Strongest on Local, Mechanically Scoped Transformations](#42-llm-assisted-refactoring-is-strongest-on-local-mechanically-scoped-transformations)
   - 4.3 [Prefer Semantic-Aware Codemods Over Regex Search-and-Replace](#43-prefer-semantic-aware-codemods-over-regex-search-and-replace)
5. [Measurement — Judging Refactor Success](#5-measurement-—-judging-refactor-success) — **MEDIUM**
   - 5.1 [Judge Refactor Success With a Multi-Dimensional Portfolio of Indicators](#51-judge-refactor-success-with-a-multi-dimensional-portfolio-of-indicators)
   - 5.2 [Smell Reduction and Static-Analysis Clean Are Not Behavior-Preservation Oracles](#52-smell-reduction-and-static-analysis-clean-are-not-behavior-preservation-oracles)

---

## 1. Triggers — When and Why to Refactor

**Impact: CRITICAL**

Refactoring without a clear trigger costs effort and creates merge cost without buying back maintenance time. Mining studies show real refactoring is driven by concrete pain — change-frequency, comprehension difficulty, review feedback, or impending API change — not by aesthetic preference.

### 1.1 Prioritize Files With High Relative Churn (Top-Decile Hotspots)

**Impact: CRITICAL**

Relative-churn is among the most reliably-validated process metrics for prioritizing maintenance. Nagappan & Ball's ICSE 2005 logistic-regression study attained 89.0% classification accuracy discriminating fault-prone from non-fault-prone binaries on Windows Server 2003 using relative-churn predictors; Pantiuchina et al. (TOSEM 2020) mined 287,813 refactoring operations across 150 systems and found prior-change frequency is a strong correlate of where developer-initiated refactoring concentrates; behavioral-code-analysis (Tornhill, *Your Code as a Crime Scene*) further quantifies a Pareto regime where ~1–2% of files typically account for ~70% of edit activity, with defects clustering in the same subset. The merge-cost of restructuring a rarely-edited file rarely amortizes; restructuring a top-decile hotspot pays back on the next feature touching it.

**Incorrect: refactoring a low-churn legacy helper — defect risk and ROI are both negligible**

```bash
# legacy/format-1999.ts: 1 commit in 24 months
git log --since="24 months ago" --pretty=oneline -- legacy/format-1999.ts
# a1b2c3d  fix typo in comment

# yet the file gets a 400-line "modernization" PR
```

**Correct: invoke the bundled `scripts/hotspots.sh`; thin wrapper around [`hotspot`](https://github.com/huangsam/hotspot) implementing Tornhill ROI scoring with the empirical 90-day window**

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

Reference: [https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/icse05churn.pdf](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/icse05churn.pdf), [https://arxiv.org/pdf/2101.01430](https://arxiv.org/pdf/2101.01430), [https://pragprog.com/titles/atcrime2/your-code-as-a-crime-scene-second-edition/](https://pragprog.com/titles/atcrime2/your-code-as-a-crime-scene-second-edition/)

### 1.2 Readability Improvements Are Evidence-Supported Refactor Goals

**Impact: HIGH**

Code readability is empirically measurable and correlates with downstream maintenance outcomes. Buse & Weimer's TSE 2010 learned readability metric (trained on 120 human annotators, evaluated across 2.2M+ LOC of multi-release codebases) achieves ~80% prediction accuracy versus human judgment and exhibits statistically significant correlation with code-change rate, automated defect reports, and defect-log messages; Scalabrino et al.'s comprehensive readability model (JSEP 2018) — calibrated against 5K+ human evaluators on 600+ snippets — combines structural and textual features to further improve classification accuracy; Daka et al. (ESEC/FSE 2015) demonstrate readability-targeted transformation can preserve coverage. Extract Method, Rename, and Inline are therefore evidence-supported transformations when the goal is reducing time-to-comprehension on the next reader's hot path, not minimizing line count.

**Incorrect: extraction adds indirection without naming intermediate concepts**

```ts
function priceFor(o: Order) {
  return calc(o.items, o.discount)
}

function calc(items: Item[], d: number) {
  return items.reduce((a, it) => a + it.price * it.qty, 0) * (1 - d)
}
```

**Correct: Extract Method names the intermediate concepts the reader needs**

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

Reference: [https://web.eecs.umich.edu/~weimerw/p/weimer-tse2010-readability-preprint.pdf](https://web.eecs.umich.edu/~weimerw/p/weimer-tse2010-readability-preprint.pdf), [https://sscalabrino.github.io/files/2018/JSEP2018AComprehensiveModel.pdf](https://sscalabrino.github.io/files/2018/JSEP2018AComprehensiveModel.pdf), [https://web.eecs.umich.edu/~weimerw/p/p107-daka.pdf](https://web.eecs.umich.edu/~weimerw/p/p107-daka.pdf)

### 1.3 Refactoring Preserves Observable Behavior

**Impact: CRITICAL**

Foundational refactoring theory — Opdyke (1992), Mens & Tourwé (TSE 2004), Fowler (2018) — converges on a single invariant: the transformation must preserve observational equivalence on the input/output relation, so every externally visible response is indistinguishable pre- and post-transformation provided the refactoring's preconditions hold. Commits that bundle structural change with semantic change defeat `git bisect` blame attribution (each hunk carries dual responsibility for any regression) and inflate review effort; Paixão et al.'s MSR 2020 study of 1,780 reviewed code changes across six systems found that mixed-intent refactoring sequences crosscut multiple code elements and disproportionately drive review turn-over. Treat behavior-preserving and behavior-changing edits as disjoint commits with separate test gates.

**Incorrect: a single commit conflates Extract Method with two new validation guards**

```ts
// commit: "refactor: extract OrderService"
function placeOrder(order: Order): Receipt {
  if (order.items.length === 0) throw new EmptyOrderError()   // new guard
  if (order.total < 0)          throw new InvalidTotalError() // new guard
  return orderService.submit(order)                            // pure extraction
}
```

**Correct: two commits with disjoint intent and independent test coverage**

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

Reference: [https://www.laputan.org/pub/papers/opdyke-thesis.pdf](https://www.laputan.org/pub/papers/opdyke-thesis.pdf), [https://ieeexplore.ieee.org/document/1265817](https://ieeexplore.ieee.org/document/1265817), [https://martinfowler.com/books/refactoring.html](https://martinfowler.com/books/refactoring.html), [http://www0.cs.ucl.ac.uk/staff/jkrinke/publications/msr20intent.pdf](http://www0.cs.ucl.ac.uk/staff/jkrinke/publications/msr20intent.pdf)

### 1.4 Start From Concrete Maintenance Pain, Not Aesthetic Preference

**Impact: CRITICAL**

Empirical motivation taxonomies converge on a non-aesthetic origin for refactoring. Silva, Tsantalis, & Valente (FSE 2016, ACM SIGSOFT Distinguished Paper) catalogued 44 motivations across 12 refactoring types and conclude refactoring is "mainly driven by changes in the requirements and much less by code smells"; Pantiuchina et al. (TOSEM 2020) corroborate this on 287,813 mined refactoring operations across 150 systems with 1,223 hand-coded motivation tags showing feature-, readability-, and review-driven intent dominate smell-driven intent; Kim, Zimmermann, & Nagappan (TSE 2014) report the same pattern at Microsoft via survey, semi-structured interview, and Windows 7 version-history triangulation. A refactor without a stated, concrete pain — a feature blocked by an awkward seam, a hotspot collision in modern code review, an explicit reviewer ask — is speculative and consistently loses priority competition against feature work.

**Incorrect: refactor with no stated pain — high probability of deferral or rejection**

```text
PR #482  refactor: clean up utils/format.ts
---
Description:
Renamed a few variables, split a function. The file looked dated.
```

**Correct: refactor anchored to a measured, recurring pain signal**

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

Reference: [https://users.encs.concordia.ca/~nikolaos/publications/FSE_2016.pdf](https://users.encs.concordia.ca/~nikolaos/publications/FSE_2016.pdf), [https://arxiv.org/pdf/2101.01430](https://arxiv.org/pdf/2101.01430), [https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/kim-tse-2014.pdf](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/kim-tse-2014.pdf)

---

## 2. Safety — How to Refactor Without Breaking Behavior

**Impact: CRITICAL**

Refactoring is defined by behavior preservation. Most refactor-induced bugs trace to one of four failures: oversized step, weak regression net, mechanical edits tangled with behavior changes, or no verification after the change. Treat all four as non-negotiable.

### 2.1 Many Small Behavior-Preserving Transformations Beat One Large Redesign

**Impact: CRITICAL**

Atomic-step discipline is the structural risk control across the refactoring canon. Opdyke (1992) formalized refactoring as sequences of atomic transformations gated by checkable preconditions; Fowler (2018) operationalizes this as "a series of small behavior-preserving transformations" with the system kept fully working after each step; Beck's *Test-Driven Development: By Example* (2003) targets 1–10-minute red-green-refactor cycles ("baby steps") and treats cycle duration as a step-size diagnostic; Feathers's *Working Effectively with Legacy Code* (2004) prescribes a smallest-safe-change algorithm — identify change point, find test point, break dependencies, characterization-test, refactor — as the only reliable traversal through untrusted code. Cedrim et al.'s longitudinal study (ESEC/FSE 2017, 16,566 refactorings across 23 projects) found 33.3% of refactorings induced new smells, with >95% of those smells persisting in subsequent commits, reinforcing that composite refactorings must be staged as independently-green atomic steps, not bundled into one tangled diff.

**Incorrect: one PR conflates rename + base-class change + method reorder; long-lived branch, painful merge**

```text
git diff --stat origin/main..refactor/cart-overhaul
 src/cart/CartService.ts      | 412 ++++++++++++++++-----------
 src/cart/ICart.ts            |  84 +++---
 src/cart/index.ts            |  12 +-
 ...
 12 files changed, 612 insertions(+), 318 deletions(-)
 branch age: 11 days, 4 force-pushes, 2 merge conflicts resolved
```

**Correct: three short-lived branches, each behavior-preserving, each independently green**

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

Reference: [https://www.laputan.org/pub/papers/opdyke-thesis.pdf](https://www.laputan.org/pub/papers/opdyke-thesis.pdf), [https://martinfowler.com/books/refactoring.html](https://martinfowler.com/books/refactoring.html), [https://www.oreilly.com/library/view/test-driven-development/0321146530/](https://www.oreilly.com/library/view/test-driven-development/0321146530/), [https://www.oreilly.com/library/view/working-effectively-with/0131177052/](https://www.oreilly.com/library/view/working-effectively-with/0131177052/), [https://dl.acm.org/doi/10.1145/3106237.3106259](https://dl.acm.org/doi/10.1145/3106237.3106259)

### 2.2 Rerun Tests, Type-Check, and Static Analysis After Every Refactor Batch

**Impact: CRITICAL**

Post-transformation verification is empirically mandatory because even semantically-aware refactoring engines ship behavior-altering defects. AlOmar et al.'s systematic mapping study (Information and Software Technology 2021, 28 surveyed behavior-preservation approaches) concludes that no single preservation tactic is sufficient across all refactoring types and that several refactoring classes remain under-researched; Gligoric et al. (ECOOP 2013) systematically tested Eclipse's Java and C refactoring engines on real projects and reported 77 newly-discovered Java bugs and 43 C bugs; Soares, Gheyi, & Massoni's SafeRefactor (TSE 2012) uncovered >100 behavior-changing defects across Eclipse JDT, NetBeans, and JastAdd by generating differential test suites across the pre- and post-transformation programs; Wang et al.'s RefactorBench (2024) extends the finding with 518 categorized refactoring-engine bug reports. Treat the verification pipeline (`pnpm test` + `pnpm tsc --noEmit` + project static analysis) as the gating step of the refactor batch, not as a downstream CI safety net.

**Incorrect: no local verification — CI carries the cost of broken imports and blocks other PRs**

```bash
# IDE rename + save + commit + push
git commit -am "refactor: rename"
git push
# remote CI fails 11 minutes later; the failing import blocks 3 unrelated PRs
# the refactor is rolled back; the wasted CI minutes are gone
```

**Correct: local verification gate runs the full triad before push**

```bash
# verification triad: behavior, types, lint
pnpm test && pnpm tsc --noEmit && pnpm lint --max-warnings 0
# 142 passed, 0 failed
# 0 type errors
# 0 lint warnings
git commit -am "refactor: rename OrderService.process -> OrderService.submit"
git push   # CI confirms what we already verified locally
```

Reference: [https://arxiv.org/abs/2106.13900](https://arxiv.org/abs/2106.13900), [https://users.ece.utexas.edu/~gligoric/papers/GligoricETAL13RTR.pdf](https://users.ece.utexas.edu/~gligoric/papers/GligoricETAL13RTR.pdf), [https://www.microsoft.com/en-us/research/wp-content/uploads/2020/08/tse12.pdf](https://www.microsoft.com/en-us/research/wp-content/uploads/2020/08/tse12.pdf), [https://arxiv.org/abs/2409.14610](https://arxiv.org/abs/2409.14610)

### 2.3 Separate Mechanical Edits From Behavior-Changing Edits at the Commit Boundary

**Impact: CRITICAL**

Tangled commits are an empirically-quantified anti-pattern. Murphy-Hill, Parnin, & Black (ICSE 2009, "How We Refactor, and How We Know It", drawn from datasets covering 13K+ developers and 240K+ tool-assisted refactorings) introduced the term *floss refactoring* for restructuring interleaved with feature/bug edits, and document that developers routinely omit refactoring intent from commit logs; Herzig & Zeller (EMSE) report 7–20% of bug-fixing commits across five Java open-source projects are tangled and demonstrate that untangling improves regression-prediction accuracy by 5–200% with 16.6% of source files otherwise incorrectly associated with bug reports; Paixão et al. (MSR 2020) reproduce the pattern in modern code review, where mixed-intent sequences inflate reviewer effort and defeat `git bisect`-based blame attribution. Enforce a strict commit boundary between mechanical refactoring (codemod-applied rename / move / extract with no logic change) and behavior-changing edits so each commit has single responsibility, single failure mode, and single-revert semantics.

**Incorrect: one commit conflates a behavior fix with a 30-file rename — bisect lands on a tangled diff**

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

**Correct: mechanical rename and behavior fix in disjoint commits — bisect identifies the right intent**

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

Reference: [https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=3afed538ca9d8fe84c71ae6af4fc987965474082](https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=3afed538ca9d8fe84c71ae6af4fc987965474082), [https://link.springer.com/article/10.1007/s10664-015-9376-6](https://link.springer.com/article/10.1007/s10664-015-9376-6), [http://www0.cs.ucl.ac.uk/staff/jkrinke/publications/msr20intent.pdf](http://www0.cs.ucl.ac.uk/staff/jkrinke/publications/msr20intent.pdf)

### 2.4 Strengthen the Regression Net Before Risky Structural Change

**Impact: CRITICAL**

The regression test suite is the only practical behavior-preservation oracle for refactoring, and its trustworthiness bounds the safety of every downstream transformation. Kim, Zimmermann, & Nagappan (Microsoft TSE 2014) identified inadequate test coverage as the most-cited blocker to refactoring across surveyed and interviewed engineers; Feathers (*Working Effectively with Legacy Code*, 2004) resolves the legacy-code circular dependency — to change safely you need tests, to add tests you often need small changes — via the characterization-test workflow (capture observed behavior, including latent bugs, before restructuring); Bavota et al.'s SCAM 2012 longitudinal analysis of Apache Ant, Xerces, and ArgoUML showed certain refactoring classes (notably hierarchy-involving operations) induce faults at elevated rates, so the regression net must explicitly cover the structures being touched; Wang et al.'s RefactorBench (2024, 518 categorized refactoring-engine bugs across Eclipse, IntelliJ IDEA, and NetBeans, with 130 newly-discovered transferable failures) further demonstrates that "trust the IDE refactor button" is no substitute for executing tests after every step.

**Incorrect: refactor lands first, suite never characterized the affected path — false-green is indistinguishable from coverage**

```bash
git checkout -b refactor/extract-pricing
# 600 LOC moved across 8 files, no new tests
pnpm test
# 142 passed, 0 failed   ← suite never executed the rounding branch
git push   # silent regression ships
```

**Correct: characterization tests pin observed behavior, including quirks; refactor proceeds against verified-green**

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

Reference: [https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/kim-tse-2014.pdf](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/kim-tse-2014.pdf), [https://www.oreilly.com/library/view/working-effectively-with/0131177052/](https://www.oreilly.com/library/view/working-effectively-with/0131177052/), [https://people.lu.usi.ch/bavotg/papers/scam2012.pdf](https://people.lu.usi.ch/bavotg/papers/scam2012.pdf), [https://arxiv.org/abs/2409.14610](https://arxiv.org/abs/2409.14610)

---

## 3. Review — Collaboration and Merge Discipline

**Impact: HIGH**

Refactoring PRs carry measurably higher churn, longer merge latency, and more review discussion than other PRs. Discipline at the PR boundary — small scope, explicit intent, narrated commits — is what keeps that overhead bounded.

### 3.1 Commit Messages Encode Refactor Intent, Not Just "Cleanup

**Impact: HIGH**

Commit messages function as the atomic-grain documentation unit for refactoring activity. AlOmar et al.'s "On the Documentation of Refactoring Types" (2021, 5,004 commits) demonstrate via supervised classification that Rename Method and Extract Method are the best-textually-documented operations while Pull-Up / Push-Down Method are empirically unrecoverable from natural language alone; the same authors' Self-Affirmed Refactoring (SAR) study (Journal of Systems and Software, 2020, 2,867 commits, 40+ recovered SAR lexical patterns) shows that when developers do encode intent they reliably emit operation + target + motivation tokens classifiable at F-measure ≈ 0.90; the industry Conventional Commits 1.0.0 specification operationalizes this structure as `<type>(<scope>): <description>` with optional body and footer, supporting deterministic downstream tooling (release notes, semantic versioning, change classification). Generic "cleanup" subjects degrade `git blame`, bisect annotation, and release-automation; structured intent compounds in value with every future reader of the commit graph.

**Incorrect: generic subjects — refactor type, scope, and motivation are unrecoverable**

```text
git log --oneline -3 -- src/cart/
a1b2c3d  refactor
b2c3d4e  cleanup
c3d4e5f  small refactor
```

**Correct: Conventional Commits form: operation + scope + motivation, SAR-classifiable at ~0.90 F1**

```text
git log --oneline -3 -- src/cart/
a1b2c3d  refactor(cart): extract CartValidator to localize rounding logic
b2c3d4e  refactor(cart): rename Cart -> ShoppingCart for domain-model consistency
c3d4e5f  refactor(cart): inline single-use CartFactory into ShoppingCart.from

git log -1 --format=fuller a1b2c3d
# subject:  refactor(cart): extract CartValidator to localize rounding logic
# body:
#   Extracts cart-validation predicates from ShoppingCart.submit into
#   CartValidator so future rounding regressions land in a single file.
#   No behavior change; covered by existing tests/cart/validator.test.ts.
# footer: Refs: #482
```

Reference: [https://arxiv.org/abs/2112.01581](https://arxiv.org/abs/2112.01581), [https://mkaouer.net/publication/alomar-2021-toward/alomar-2021-toward.pdf](https://mkaouer.net/publication/alomar-2021-toward/alomar-2021-toward.pdf), [https://www.conventionalcommits.org/en/v1.0.0/](https://www.conventionalcommits.org/en/v1.0.0/)

### 3.2 Keep Refactor PRs Small and Short-Lived

**Impact: HIGH**

Refactoring-bearing PRs measurably degrade review economics. Coelho, Tsantalis, Massoni, & Alves (ESEM 2021) compared refactoring-inducing PRs against control PRs and found statistically significant elevations in line-churn, file-fan-out, and discussion-comment volume in the refactoring set; Rigby & Bird (ESEC/FSE 2013), analyzing convergent code-review practices across Google (Android, Chromium OS), Microsoft (Bing, Office, MS SQL), AMD, and OSS, demonstrate that review-cycle count scales with initial patch size and added-line count, an effect that holds across organizational boundaries; Gousios, Pinzger, & van Deursen (ICSE 2014, GHTorrent corpus + 291-project sample) plus follow-up qualitative work identify PR churn (changed lines) as a primary determinant of merge latency along with project hotness and submitter track record. Empirical convergence: cap each refactor PR at one cohesive intent, ideally under ~400 LOC changed, with branch lifetime under one working day.

**Incorrect: one PR conflates five intents, long-lived branch, reviewer attrition**

```text
PR #482  "refactor: cart cleanup"
  diff:       +2138 / -1907 across 47 files
  branch age: 14 days
  reviews:    23 comments, 4 merge-conflict resolutions, 1 reviewer disengaged
  outcome:    merged after rebase + force-push; review fatigue evident in final approvals
```

**Correct: intent-disjoint PR sequence, each independently mergeable in a working day**

```text
PR #482  refactor: extract CartValidator              +180 / -120,  6 files, merged day 1
PR #483  refactor: rename Cart -> ShoppingCart        +402 / -402, 31 files, merged day 1
PR #484  refactor: collapse three pricing functions   +95  / -210,  4 files, merged day 2
PR #485  refactor: inline unused CartFactory          +0   / -85,   2 files, merged day 2
```

Reference: [https://users.encs.concordia.ca/~nikolaos/publications/ESEM_2021.pdf](https://users.encs.concordia.ca/~nikolaos/publications/ESEM_2021.pdf), [https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/rigby2013convergent.pdf](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/rigby2013convergent.pdf), [https://www.gousios.org/pub/exploratory-study-pull-based.pdf](https://www.gousios.org/pub/exploratory-study-pull-based.pdf)

---

## 4. Automation — Tools, Codemods, and LLM Assistance

**Impact: HIGH**

Automation pays off for repetitive, semantically analyzable change at repository scale. It is unreliable for context-heavy design decisions. Pick the lightest tool that understands the language semantics, and never let LLM speed remove human oversight.

### 4.1 LLM Migration Tools Accelerate Mechanics; Per-Patch Oversight Stays In-Loop

**Impact: HIGH**

LLM-assisted code migration is empirically validated at industrial scale, but only with strict per-patch test gating and human review. Ziftci et al. (FSE 2025, "Migrating Code At Scale With LLMs At Google", 39 migrations executed by 3 developers over 12 months) report 595 code changes and 93,574 edits with LLMs generating 74.45% of changes and 69.46% of edits, alongside a developer-estimated 50% reduction in migration time — achieved while keeping developers in-loop on every accepted edit. Independent enterprise studies of GitHub Copilot (GitHub × Accenture; Zoominfo) report suggestion-acceptance rates of ~30–33% — i.e., the majority of LLM output is rejected before merge — and agentic-coding PR studies (567 PRs across 157 OSS projects) find Agentic-PRs accepted at lower rates than human-authored PRs, with high task-stratified variance (bug-fix acceptance 45.6%–83.0% across agents). Per-patch test execution plus standard code review are therefore non-negotiable: the speedup is real, but stripping the oversight loop converts it into a silent regression pipeline.

**Incorrect: bulk-apply 240 LLM patches, single commit, CI is the only oracle**

```bash
# 240 LLM-generated patches applied in one transaction
git apply patches/*.diff
git add -A && git commit -m "migrate to v2 SDK"
git push

# CI fails 3 hours later on 14 files
# rolling back a 240-file commit invalidates all downstream branches
# bisect lands on a tangled diff that hides which intent failed
```

**Correct: per-patch test gate + per-patch commit + per-patch reviewable diff**

```bash
# replicate the Google migration pattern: bounded scope, gated patches, human-in-loop
for patch in patches/*.diff; do
  git apply "$patch"

  # patch-scoped test gate (only the modules the patch touched)
  scope="$(scripts/scope-for-patch.sh "$patch")"
  pnpm test --filter "$scope" && pnpm tsc --noEmit -p tsconfig.$scope.json || {
    echo "rejected: $patch (failed scoped gate)"
    git checkout -- .
    continue
  }

  git add -A
  git commit -m "migrate($scope): apply $(basename "$patch" .diff) (LLM-drafted, gate-verified)"
done

# every commit is independently green, independently reviewable, independently revertable
gh pr create --title "migrate to v2 SDK (LLM-assisted, $(git rev-list --count HEAD ^main) gated commits)"
```

Reference: [https://arxiv.org/abs/2504.09691](https://arxiv.org/abs/2504.09691), [https://arxiv.org/abs/2206.15331](https://arxiv.org/abs/2206.15331), [https://arxiv.org/abs/2509.14745](https://arxiv.org/abs/2509.14745)

### 4.2 LLM-Assisted Refactoring Is Strongest on Local, Mechanically Scoped Transformations

**Impact: HIGH**

LLM refactoring efficacy is non-uniform across the refactoring taxonomy and degrades sharply with cross-file design context. Cordeiro, Noei, & Zou (TOSEM 2025 / arXiv 2411.02320, 30-project Java corpus) report StarCoder2 outperforming human developers on 10 of 16 syntactic/pattern-based code smells (Long Statement, Long Parameter List, Magic Number, Empty Catch Clause, …) with a ~20.1% greater smell-density reduction while developers retain the edge on context-dependent design issues; Liu et al. (arXiv 2411.04444, 180 ground-truth refactorings, 20 projects) measured ChatGPT identifying only 15.6% of opportunities at default prompting, rising to 86.7% when prompts narrow the search space, with 63.6% of recommended solutions judged comparable-to-superior to human experts and a non-trivial fraction unsafe (functionality change or syntax error); Pomian, Bellur, & Tsantalis (EM-Assist, FSE 2024 Demonstrations, 1,752-refactoring corpus) document a 76.3% hallucination rate that necessitates IDE static-analysis filtering, after which recall against ground-truth Extract Method reaches 53.4% versus 39.4% for prior state-of-the-art. Scope LLM use to local, type-checked, individually verifiable transformations; delegate cross-file architecture and invariant-bearing redesign to humans plus IDE static analysis.

**Incorrect: cross-file architectural rewrite from a thin prompt — high hallucination rate, no oracle**

```text
Prompt:
  "Refactor the cart module to follow hexagonal architecture."

Output:
  - 17 new files
  - invented ports/adapters with unresolved imports
  - existing tests untouched
  - no preconditions verified

Result: 76%+ of suggestions hallucinated (cf. EM-Assist 2024); manual
        recovery cost exceeds the value of the change.
```

**Correct: local, type-checked, single-method Extract Method with explicit pre/postconditions**

```text
Prompt (one-shot, scope-narrowed per Liu et al. 2024):
  "In `priceFor` below, extract the line-item summation into a pure helper
   `sumLineItems(items: Item[]): Money`. Preserve observable behavior.
   Do not change the public signature of `priceFor`. Return only the new
   function bodies."

Verification gate:
  pnpm tsc --noEmit                 # type system rejects symbol-resolution errors
  pnpm test src/pricing             # existing tests prove observational equivalence
  refactoring-detector --against=HEAD~1 \
    --expect "Extract Method: sumLineItems"   # detector confirms intent

Outcome: one independently verifiable atomic refactoring, revertable in one commit.
```

Reference: [https://arxiv.org/abs/2411.02320](https://arxiv.org/abs/2411.02320), [https://arxiv.org/abs/2411.04444](https://arxiv.org/abs/2411.04444), [https://arxiv.org/abs/2405.20551](https://arxiv.org/abs/2405.20551)

### 4.3 Prefer Semantic-Aware Codemods Over Regex Search-and-Replace

**Impact: HIGH**

Repository-scale refactoring is reliably automatable only via tools operating on a program's semantic representation — AST, symbol-resolution, type information — not surface text. Coccinelle / SmPL (Lawall et al., EuroSys 2008, *Test-of-Time*-awarded) applies semantic patches to C code and achieved 93%+ completion across 5,800+ Linux device-driver files for 62 representative collateral evolutions, contributing 6,000+ landed Linux-kernel commits over a decade (USENIX ATC 2018); ClangMR (Wright, Jasper, Klimek, Carruth, Wan, ICSM 2013) parallelizes Clang's AST traversal across MapReduce to refactor Google's C++ monorepo on API migrations at codebase scale; `jscodeshift` (Facebook OSS) operates on recast-mediated ASTs preserving original whitespace and identifier scope. Regex-based search-and-replace ignores identifier shadowing, string-literal and comment context, and overloaded-symbol resolution — defects propagate silently across the diff and accumulate review cost downstream.

**Incorrect: textual `sed` rename rewrites comments, string literals, and partial-match identifiers**

```bash
# rename `Cart` -> `ShoppingCart` repository-wide via regex
git ls-files '*.ts' | xargs sed -i '' 's/Cart/ShoppingCart/g'

# collateral damage:
git diff --stat | head
#   src/checkout/util.ts        # "AbandonedCart" log message rewritten
#   src/math/cartesian.ts       # CartesianProduct -> ShoppingCartesianProduct
#   tests/fixtures/orders.json  # string data "Cart-001" rewritten
#   docs/architecture.md        # comments rewritten
```

**Correct: AST-aware codemod with symbol scoping; only the Cart class binding is touched**

```bash
# scripts/rename-cart.ts (jscodeshift transform)
# - resolves the Cart class symbol via the language service
# - rewrites only declaration sites, type references, and import/export bindings
# - leaves strings, comments, and unrelated identifiers (CartesianProduct) untouched
npx jscodeshift -t scripts/rename-cart.ts --extensions=ts,tsx --parser=tsx src/

# verify semantic scope of the change
git diff --stat | head
#   src/cart/ShoppingCart.ts        (renamed from src/cart/Cart.ts)
#   src/cart/index.ts               (re-export updated)
#   src/checkout/CheckoutFlow.ts    (import + 3 type references updated)
git grep -nF 'CartesianProduct' src/math/   # untouched, as required
```

Reference: [https://who.paris.inria.fr/Julia.Lawall/eurosys08.pdf](https://who.paris.inria.fr/Julia.Lawall/eurosys08.pdf), [https://www.hyrumwright.org/papers/icsm2013.pdf](https://www.hyrumwright.org/papers/icsm2013.pdf), [https://github.com/facebook/jscodeshift](https://github.com/facebook/jscodeshift)

---

## 5. Measurement — Judging Refactor Success

**Impact: MEDIUM**

No single metric captures refactor value. Smell counts drop while behavior breaks; readability improves while review cost spikes. Use a portfolio of indicators and tie success to future change cost, not immediate style conformance.

### 5.1 Judge Refactor Success With a Multi-Dimensional Portfolio of Indicators

**Impact: MEDIUM**

Single-metric refactor evaluation is empirically unreliable and biases toward perverse outcomes. Pantiuchina, Lanza, & Bavota (ICSME 2018, *Improving Code: The (Mis)Perception of Quality Metrics*) demonstrated that developer-perceived quality improvements frequently fail to manifest in standard metric deltas — and conversely metric improvements do not necessarily reflect perceived improvement — establishing that single-axis evaluation systematically diverges from outcome; the SPACE framework (Forsgren, Storey, Maddila, Zimmermann, Houck, Butler, ACM Queue / CACM 2021) operationalizes the multi-dimensional principle by mandating coverage of at least three of {Satisfaction, Performance, Activity, Communication, Efficiency} for any developer-productivity assessment; the DORA program (Forsgren, Humble, Kim, *Accelerate*, 2018) confirms the same property at delivery-performance level — elite performers simultaneously excel on deployment frequency, lead time, change failure rate, and MTTR. Decide success criteria up front using a small portfolio: behavior preservation, reviewability, comprehension, hotspot impact, and future change cost.

**Incorrect: single-metric declaration of success — vulnerable to misalignment with perceived outcome**

```text
Refactor retro:
  smell count: 142 -> 78 (-45%)  ✓
  → ship it
```

**Correct: multi-dimensional portfolio with disjoint indicators, pre-committed thresholds**

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

Reference: [https://www.researchgate.net/publication/328905356_Improving_Code_The_Mis_Perception_of_Quality_Metrics](https://www.researchgate.net/publication/328905356_Improving_Code_The_Mis_Perception_of_Quality_Metrics), [https://queue.acm.org/detail.cfm?id=3454124](https://queue.acm.org/detail.cfm?id=3454124), [https://itrevolution.com/product/accelerate/](https://itrevolution.com/product/accelerate/)

### 5.2 Smell Reduction and Static-Analysis Clean Are Not Behavior-Preservation Oracles

**Impact: MEDIUM**

Code-smell counts and static-analysis scores are weakly correlated with maintenance outcomes and provide no semantic-safety guarantee. Sjøberg, Yamashita, Anda, Mockus, & Dybå (IEEE TSE 2013, *Quantifying the Effect of Code Smells on Maintenance Effort*) ran a controlled experiment with six professional developers performing 3 maintenance tasks on 4 functionally equivalent Java systems and found *none* of 12 investigated code smells was significantly associated with maintenance effort after adjusting for file size and number of changes — i.e., the headline metric does not predict the outcome it is taken to proxy. Goseva-Popstojanova & Perhinschi (IST 2015) report static-analyzer false-negative rates of 47–80% across 27 projects / 1.15M LOC / 192 vulnerabilities, with multi-tool combination reducing false-negatives only to 30–69% at the cost of additional false positives. Cedrim et al. (ESEC/FSE 2017, 16,566 refactorings across 23 projects) found 33.3% of refactorings *induce* new smells, so smell-direction alone may even invert the signal. Use smells and static-analysis findings as prioritization signals (*where* to look); retain the executed test suite + type-check + (for risky scope) differential test against a known-good baseline as the behavior-preservation gate.

**Incorrect: static analysis green → ship, no executed-test oracle**

```bash
pnpm lint --max-warnings 0 && pnpm tsc --noEmit
# 0 lint warnings; 0 type errors
git push
# but the refactor changed an enum boundary that no static check enforces;
# the regression ships silently and surfaces in a customer report
```

**Correct: smells as priority signal; tests + type-check + differential check as safety gate**

```bash
# (a) priority signal — smells used only to pick the refactor target
pnpm lint --format json | jq '[.[]|select(.filePath|test("src/pricing"))]|length'
# 14   → pricing module is a candidate for refactoring

# (b) safety gate — independent of smell metrics
pnpm test --coverage src/pricing && \
  pnpm tsc --noEmit && \
  pnpm lint --max-warnings 0 && \
  refactoring-detector --against=HEAD~1 --expect "Extract Method"
# 148 passed; 0 type errors; 0 warnings; refactor intent verified

# (c) for high-risk scope: differential check against known-good
scripts/golden-diff.sh src/pricing > /tmp/post.json
diff /tmp/baseline.json /tmp/post.json    # empty → observationally equivalent
```

Reference: [https://www.mn.uio.no/ifi/personer/vit/dagsj/sjoberg.yamashita.anda.mockus.dyba.tse.2013.pdf](https://www.mn.uio.no/ifi/personer/vit/dagsj/sjoberg.yamashita.anda.mockus.dyba.tse.2013.pdf), [https://community.wvu.edu/~kagoseva/Papers/IST-2015.pdf](https://community.wvu.edu/~kagoseva/Papers/IST-2015.pdf), [https://dl.acm.org/doi/10.1145/3106237.3106259](https://dl.acm.org/doi/10.1145/3106237.3106259)

---

## References

1. [https://refactoring.com/](https://refactoring.com/)
2. [https://martinfowler.com/books/refactoring.html](https://martinfowler.com/books/refactoring.html)
3. [https://www.computer.org/csdl/journal/ts/2004/02/e0126/13rRUwInvyM](https://www.computer.org/csdl/journal/ts/2004/02/e0126/13rRUwInvyM)
4. [https://www.microsoft.com/en-us/research/publication/an-empirical-study-of-refactoring-challenges-and-benefits-at-microsoft/](https://www.microsoft.com/en-us/research/publication/an-empirical-study-of-refactoring-challenges-and-benefits-at-microsoft/)
5. [https://refactoringminer.org/](https://refactoringminer.org/)
6. [https://docs.openrewrite.org/](https://docs.openrewrite.org/)
7. [https://github.com/google/error-prone](https://github.com/google/error-prone)
8. [https://coccinelle.gitlabpages.inria.fr/website/](https://coccinelle.gitlabpages.inria.fr/website/)
9. [https://arxiv.org/abs/2411.02320](https://arxiv.org/abs/2411.02320)
