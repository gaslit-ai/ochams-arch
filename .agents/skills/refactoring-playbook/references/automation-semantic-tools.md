---
title: Prefer Semantic-Aware Codemods Over Regex Search-and-Replace
impact: HIGH
tags: automation, codemod, ast, symbol-resolution
---

## Prefer Semantic-Aware Codemods Over Regex Search-and-Replace

Repository-scale refactoring is reliably automatable only via tools operating on a program's semantic representation — AST, symbol-resolution, type information — not surface text. Coccinelle / SmPL (Lawall et al., EuroSys 2008, *Test-of-Time*-awarded) applies semantic patches to C code and achieved 93%+ completion across 5,800+ Linux device-driver files for 62 representative collateral evolutions, contributing 6,000+ landed Linux-kernel commits over a decade (USENIX ATC 2018); ClangMR (Wright, Jasper, Klimek, Carruth, Wan, ICSM 2013) parallelizes Clang's AST traversal across MapReduce to refactor Google's C++ monorepo on API migrations at codebase scale; `jscodeshift` (Facebook OSS) operates on recast-mediated ASTs preserving original whitespace and identifier scope. Regex-based search-and-replace ignores identifier shadowing, string-literal and comment context, and overloaded-symbol resolution — defects propagate silently across the diff and accumulate review cost downstream.

**Incorrect (textual `sed` rename rewrites comments, string literals, and partial-match identifiers):**

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

**Correct (AST-aware codemod with symbol scoping; only the Cart class binding is touched):**

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

References:
- [Lawall, Padioleau, Hansen, Muller (2008) — Documenting and Automating Collateral Evolutions in Linux Device Drivers, EuroSys](https://who.paris.inria.fr/Julia.Lawall/eurosys08.pdf)
- [Wright, Jasper, Klimek, Carruth, Wan (2013) — Large-Scale Automated Refactoring Using ClangMR, ICSM](https://www.hyrumwright.org/papers/icsm2013.pdf)
- [Facebook — jscodeshift (AST-based JavaScript/TypeScript codemod toolkit)](https://github.com/facebook/jscodeshift)
