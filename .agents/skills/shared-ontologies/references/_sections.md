# Sections

This file defines section ordering, impact, and reference filename prefixes.

---

## 1. Requirements — Purpose, Scope, and Contract (requirements)

**Impact:** CRITICAL
**Description:** A shared ontology becomes unbounded the moment it is treated as a list of preferred names. The ontology-engineering literature converges on two prerequisites before any term is minted: explicit competency questions and motivating scenarios that define the verification target, and explicit definitions, relations, and constraints that turn each term into a commitment rather than a label. Skip either and the ontology cannot be evaluated, scoped, or relied on at integration boundaries.

## 2. Architecture — Reuse, Modularity, and Expressivity (architecture)

**Impact:** HIGH
**Description:** Architectural decisions — what to reuse from existing reference and upper ontologies, how to decompose into modules with stable interfaces, and which formalism to commit to — set the lifetime cost of the ontology. Monolithic vocabularies, locally-minted synonyms of well-known terms, and choosing a logic stronger than the required inference each create durable maintenance and reasoning overhead.

## 3. Validation — Release-Gate Checks (validation)

**Impact:** CRITICAL
**Description:** A shared ontology is executable semantic infrastructure. Releases that ship without reasoner consistency checks, competency-question entailment, SHACL/ShEx data-shape validation, and empirically-grounded pitfall scans deliver distributed defects directly into downstream consumers — annotations, queries, generated code, and integration mappings inherit every undetected error.

## 4. Evolution — Versioning and Mappings (evolution)

**Impact:** CRITICAL
**Description:** Ontology changes affect data, APIs, queries, generated code, annotations, reasoning results, and downstream integrations. The two artefacts that keep change tractable are explicit versioning with provenance, deprecation, and migration rules, and first-class cross-ontology mappings that make heterogeneity visible, reviewable, and testable instead of hidden inside service code.

## 5. Governance — Editorial Workflow and Measurement (governance)

**Impact:** HIGH
**Description:** A shared ontology encodes social agreement among stakeholders, so the editorial workflow and the value-measurement story are part of the engineering surface. Without explicit roles, proposal flow, domain-expert review, and operational measures of use (CQ pass rate, mapping reuse, adapter logic removed), the ontology drifts toward whichever local interpretation wins the last merge.
