---
title: Commit Messages Encode Refactor Intent, Not Just "Cleanup"
impact: HIGH
tags: review, commits, self-affirmed-refactoring, conventional-commits
---

## Commit Messages Encode Refactor Intent, Not Just "Cleanup"

Commit messages function as the atomic-grain documentation unit for refactoring activity. AlOmar et al.'s "On the Documentation of Refactoring Types" (2021, 5,004 commits) demonstrate via supervised classification that Rename Method and Extract Method are the best-textually-documented operations while Pull-Up / Push-Down Method are empirically unrecoverable from natural language alone; the same authors' Self-Affirmed Refactoring (SAR) study (Journal of Systems and Software, 2020, 2,867 commits, 40+ recovered SAR lexical patterns) shows that when developers do encode intent they reliably emit operation + target + motivation tokens classifiable at F-measure ≈ 0.90; the industry Conventional Commits 1.0.0 specification operationalizes this structure as `<type>(<scope>): <description>` with optional body and footer, supporting deterministic downstream tooling (release notes, semantic versioning, change classification). Generic "cleanup" subjects degrade `git blame`, bisect annotation, and release-automation; structured intent compounds in value with every future reader of the commit graph.

**Incorrect (generic subjects — refactor type, scope, and motivation are unrecoverable):**

```text
git log --oneline -3 -- src/cart/
a1b2c3d  refactor
b2c3d4e  cleanup
c3d4e5f  small refactor
```

**Correct (Conventional Commits form: operation + scope + motivation, SAR-classifiable at ~0.90 F1):**

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

References:
- [AlOmar, Liu, Mkaouer, Newman, Ouni (2021) — On the Documentation of Refactoring Types](https://arxiv.org/abs/2112.01581)
- [AlOmar, Mkaouer, Ouni (2020) — Toward the Automatic Classification of Self-Affirmed Refactoring, JSS](https://mkaouer.net/publication/alomar-2021-toward/alomar-2021-toward.pdf)
- [Conventional Commits 1.0.0 — specification](https://www.conventionalcommits.org/en/v1.0.0/)
