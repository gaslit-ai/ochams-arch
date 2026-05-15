---
title: LLM-Assisted Refactoring Is Strongest on Local, Mechanically Scoped Transformations
impact: HIGH
tags: automation, llm, scope, hallucination
---

## LLM-Assisted Refactoring Is Strongest on Local, Mechanically Scoped Transformations

LLM refactoring efficacy is non-uniform across the refactoring taxonomy and degrades sharply with cross-file design context. Cordeiro, Noei, & Zou (TOSEM 2025 / arXiv 2411.02320, 30-project Java corpus) report StarCoder2 outperforming human developers on 10 of 16 syntactic/pattern-based code smells (Long Statement, Long Parameter List, Magic Number, Empty Catch Clause, …) with a ~20.1% greater smell-density reduction while developers retain the edge on context-dependent design issues; Liu et al. (arXiv 2411.04444, 180 ground-truth refactorings, 20 projects) measured ChatGPT identifying only 15.6% of opportunities at default prompting, rising to 86.7% when prompts narrow the search space, with 63.6% of recommended solutions judged comparable-to-superior to human experts and a non-trivial fraction unsafe (functionality change or syntax error); Pomian, Bellur, & Tsantalis (EM-Assist, FSE 2024 Demonstrations, 1,752-refactoring corpus) document a 76.3% hallucination rate that necessitates IDE static-analysis filtering, after which recall against ground-truth Extract Method reaches 53.4% versus 39.4% for prior state-of-the-art. Scope LLM use to local, type-checked, individually verifiable transformations; delegate cross-file architecture and invariant-bearing redesign to humans plus IDE static analysis.

**Incorrect (cross-file architectural rewrite from a thin prompt — high hallucination rate, no oracle):**

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

**Correct (local, type-checked, single-method Extract Method with explicit pre/postconditions):**

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

References:
- [Cordeiro, Noei, Zou (2024/2025) — An Empirical Study on the Code Refactoring Capability of Large Language Models, TOSEM](https://arxiv.org/abs/2411.02320)
- [Liu et al. (2024) — An Empirical Study on the Potential of LLMs in Automated Software Refactoring](https://arxiv.org/abs/2411.04444)
- [Pomian, Bellur, Tsantalis (2024) — EM-Assist: Safe Automated Extract Method Refactoring with LLMs, FSE Demonstrations](https://arxiv.org/abs/2405.20551)
