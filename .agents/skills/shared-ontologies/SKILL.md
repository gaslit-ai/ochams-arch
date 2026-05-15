---
name: shared-ontologies
description: Evidence-based playbook for designing, evolving, and governing shared ontologies across systems and teams. Use when introducing a shared vocabulary, integrating data across services, picking ontology expressivity, validating ontology releases, versioning semantic changes, or aligning a new schema with an existing reference ontology. Triggers on tasks framed as "shared ontology", "domain model", "controlled vocabulary", "semantic schema", "data integration", "ontology mapping", or "ontology alignment".
license: MIT
metadata:
  author: duke
  version: "0.2.0"
---

# Shared Ontologies

Evidence-based guidance for shared ontologies. Each reference separates source-backed evidence from derived engineering rules; per-reference frontmatter records evidence strength and corroboration count.

Each reference below is a separate file in `references/`; the agent loads only the references it needs for the current task. The fully expanded long-form guide lives in `AGENTS.md` for tools that follow the [AGENTS.md convention](https://agents.md) rather than the [Agent Skills spec](https://agentskills.io/specification).

## References

### 1. Requirements — Purpose, Scope, and Contract — **CRITICAL**

A shared ontology becomes unbounded the moment it is treated as a list of preferred names. The ontology-engineering literature converges on two prerequisites before any term is minted: explicit competency questions and motivating scenarios that define the verification target, and explicit definitions, relations, and constraints that turn each term into a commitment rather than a label. Skip either and the ontology cannot be evaluated, scoped, or relied on at integration boundaries.

- [`requirements-competency-questions`](./references/requirements-competency-questions.md) — Define Ontology Scope With Competency Questions Before Adding Shared Terms
- [`requirements-semantic-contract`](./references/requirements-semantic-contract.md) — Treat Shared Ontology Terms as Semantic Contracts, Not Naming Conventions

### 2. Architecture — Reuse, Modularity, and Expressivity — **HIGH**

Architectural decisions — what to reuse from existing reference and upper ontologies, how to decompose into modules with stable interfaces, and which formalism to commit to — set the lifetime cost of the ontology. Monolithic vocabularies, locally-minted synonyms of well-known terms, and choosing a logic stronger than the required inference each create durable maintenance and reasoning overhead.

- [`architecture-weakest-formalism`](./references/architecture-weakest-formalism.md) — Choose Ontology Expressivity From the Required Inference and Validation Workload
- [`architecture-modular-boundaries`](./references/architecture-modular-boundaries.md) — Modularize Shared Ontologies Around Stable Software and Data Boundaries
- [`architecture-reuse-reference`](./references/architecture-reuse-reference.md) — Reuse Reference Ontologies When They Match the Competency Questions

### 3. Validation — Release-Gate Checks — **CRITICAL**

A shared ontology is executable semantic infrastructure. Releases that ship without reasoner consistency checks, competency-question entailment, SHACL/ShEx data-shape validation, and empirically-grounded pitfall scans deliver distributed defects directly into downstream consumers — annotations, queries, generated code, and integration mappings inherit every undetected error.

- [`validation-tests-reasoners-shapes`](./references/validation-tests-reasoners-shapes.md) — Validate Ontology Releases With Competency Tests, Reasoners, Shapes, and Pitfall Checks

### 4. Evolution — Versioning and Mappings — **CRITICAL**

Ontology changes affect data, APIs, queries, generated code, annotations, reasoning results, and downstream integrations. The two artefacts that keep change tractable are explicit versioning with provenance, deprecation, and migration rules, and first-class cross-ontology mappings that make heterogeneity visible, reviewable, and testable instead of hidden inside service code.

- [`evolution-mappings-first-class`](./references/evolution-mappings-first-class.md) — Make Schema-to-Ontology and Ontology-to-Ontology Mappings First-Class Artifacts
- [`evolution-versioned-migrations`](./references/evolution-versioned-migrations.md) — Version Ontology Changes as Semantic Migrations

### 5. Governance — Editorial Workflow and Measurement — **HIGH**

A shared ontology encodes social agreement among stakeholders, so the editorial workflow and the value-measurement story are part of the engineering surface. Without explicit roles, proposal flow, domain-expert review, and operational measures of use (CQ pass rate, mapping reuse, adapter logic removed), the ontology drifts toward whichever local interpretation wins the last merge.

- [`governance-editorial-workflow`](./references/governance-editorial-workflow.md) — Govern Shared Ontologies With Collaborative Editorial Workflow
- [`governance-operational-measurement`](./references/governance-operational-measurement.md) — Measure Ontology Value by Operational Interoperability, Not Term Count

## Full Compiled Document

For the complete guide with every reference expanded inline (bad/good examples, citations, prerequisites), see [`AGENTS.md`](./AGENTS.md). It is the [AGENTS.md-convention](https://agents.md) fallback for tools that do not load individual references on demand.
