---
title: Smell Reduction and Static-Analysis Clean Are Not Behavior-Preservation Oracles
impact: MEDIUM
tags: measure, smells, static-analysis, safety
---

## Smell Reduction and Static-Analysis Clean Are Not Behavior-Preservation Oracles

Code-smell counts and static-analysis scores are weakly correlated with maintenance outcomes and provide no semantic-safety guarantee. Sjøberg, Yamashita, Anda, Mockus, & Dybå (IEEE TSE 2013, *Quantifying the Effect of Code Smells on Maintenance Effort*) ran a controlled experiment with six professional developers performing 3 maintenance tasks on 4 functionally equivalent Java systems and found *none* of 12 investigated code smells was significantly associated with maintenance effort after adjusting for file size and number of changes — i.e., the headline metric does not predict the outcome it is taken to proxy. Goseva-Popstojanova & Perhinschi (IST 2015) report static-analyzer false-negative rates of 47–80% across 27 projects / 1.15M LOC / 192 vulnerabilities, with multi-tool combination reducing false-negatives only to 30–69% at the cost of additional false positives. Cedrim et al. (ESEC/FSE 2017, 16,566 refactorings across 23 projects) found 33.3% of refactorings *induce* new smells, so smell-direction alone may even invert the signal. Use smells and static-analysis findings as prioritization signals (*where* to look); retain the executed test suite + type-check + (for risky scope) differential test against a known-good baseline as the behavior-preservation gate.

**Incorrect (static analysis green → ship, no executed-test oracle):**

```bash
pnpm lint --max-warnings 0 && pnpm tsc --noEmit
# 0 lint warnings; 0 type errors
git push
# but the refactor changed an enum boundary that no static check enforces;
# the regression ships silently and surfaces in a customer report
```

**Correct (smells as priority signal; tests + type-check + differential check as safety gate):**

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

References:
- [Sjøberg, Yamashita, Anda, Mockus, Dybå (2013) — Quantifying the Effect of Code Smells on Maintenance Effort, IEEE TSE](https://www.mn.uio.no/ifi/personer/vit/dagsj/sjoberg.yamashita.anda.mockus.dyba.tse.2013.pdf)
- [Goseva-Popstojanova & Perhinschi (2015) — On the Capability of Static Code Analysis to Detect Security Vulnerabilities, IST](https://community.wvu.edu/~kagoseva/Papers/IST-2015.pdf)
- [Cedrim et al. (2017) — Understanding the Impact of Refactoring on Smells, ESEC/FSE](https://dl.acm.org/doi/10.1145/3106237.3106259)
