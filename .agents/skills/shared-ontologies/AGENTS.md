# Shared Ontologies

**Version 0.2.0**  
Duke Engineering  
May 2026

> **Note:**  
> This guide is optimized for agents and AI-assisted engineering workflows.
> It prioritizes deterministic behavior, explicit contracts, and maintainable defaults.

---

## Abstract

Evidence-based playbook for shared ontologies in software systems — semantic artifacts used across services, schemas, APIs, knowledge graphs, validation layers, annotations, data-access mappings, or architecture artifacts. Each reference separates source-backed evidence (synthesized from cited literature) from derived engineering guidance. Per-reference frontmatter records evidenceStrength and corroborationCount in place of subjective confidence levels. Code examples are synthetic implementation sketches, not examples observed in the cited papers. Synthesizes 50+ peer-reviewed or canonical sources spanning ontology engineering (Gruber, Grüninger & Fox, METHONTOLOGY, NeOn, Ontology Requirements Specification), semantic web standards (OWL 2 profiles, OBDA, SHACL/ShEx, R2RML), software-engineering ontology literature, biomedical ontology infrastructure (OBO Foundry, Gene Ontology, MIREOT), upper ontologies (DOLCE, BFO), modularity research, ontology evaluation (OntoClean, OOPS!, RDFUnit), ontology evolution and versioning (PAV, Flouris, Zablith), schema and ontology matching (Rahm & Bernstein; Euzenat & Shvaiko), and collaborative ontology engineering (WebProtégé, NeOn governance) into references covering requirements, architecture, validation, evolution, and governance.

---

## Table of Contents

1. [Requirements — Purpose, Scope, and Contract](#1-requirements-—-purpose,-scope,-and-contract) — **CRITICAL**
   - 1.1 [Define Ontology Scope With Competency Questions Before Adding Shared Terms](#11-define-ontology-scope-with-competency-questions-before-adding-shared-terms)
   - 1.2 [Treat Shared Ontology Terms as Semantic Contracts, Not Naming Conventions](#12-treat-shared-ontology-terms-as-semantic-contracts-not-naming-conventions)
2. [Architecture — Reuse, Modularity, and Expressivity](#2-architecture-—-reuse,-modularity,-and-expressivity) — **HIGH**
   - 2.1 [Choose Ontology Expressivity From the Required Inference and Validation Workload](#21-choose-ontology-expressivity-from-the-required-inference-and-validation-workload)
   - 2.2 [Modularize Shared Ontologies Around Stable Software and Data Boundaries](#22-modularize-shared-ontologies-around-stable-software-and-data-boundaries)
   - 2.3 [Reuse Reference Ontologies When They Match the Competency Questions](#23-reuse-reference-ontologies-when-they-match-the-competency-questions)
3. [Validation — Release-Gate Checks](#3-validation-—-release-gate-checks) — **CRITICAL**
   - 3.1 [Validate Ontology Releases With Competency Tests, Reasoners, Shapes, and Pitfall Checks](#31-validate-ontology-releases-with-competency-tests-reasoners-shapes-and-pitfall-checks)
4. [Evolution — Versioning and Mappings](#4-evolution-—-versioning-and-mappings) — **CRITICAL**
   - 4.1 [Make Schema-to-Ontology and Ontology-to-Ontology Mappings First-Class Artifacts](#41-make-schema-to-ontology-and-ontology-to-ontology-mappings-first-class-artifacts)
   - 4.2 [Version Ontology Changes as Semantic Migrations](#42-version-ontology-changes-as-semantic-migrations)
5. [Governance — Editorial Workflow and Measurement](#5-governance-—-editorial-workflow-and-measurement) — **HIGH**
   - 5.1 [Govern Shared Ontologies With Collaborative Editorial Workflow](#51-govern-shared-ontologies-with-collaborative-editorial-workflow)
   - 5.2 [Measure Ontology Value by Operational Interoperability, Not Term Count](#52-measure-ontology-value-by-operational-interoperability-not-term-count)

---

## 1. Requirements — Purpose, Scope, and Contract

**Impact: CRITICAL**

A shared ontology becomes unbounded the moment it is treated as a list of preferred names. The ontology-engineering literature converges on two prerequisites before any term is minted: explicit competency questions and motivating scenarios that define the verification target, and explicit definitions, relations, and constraints that turn each term into a commitment rather than a label. Skip either and the ontology cannot be evaluated, scoped, or relied on at integration boundaries.

### 1.1 Define Ontology Scope With Competency Questions Before Adding Shared Terms

**Impact: CRITICAL (Unscoped shared vocabularies create ambiguous integration artifacts rather than shared semantics.)**

**Source-backed evidence:** Grüninger and Fox characterize an ontology through competency questions and identify axioms needed to represent and answer them; Noy and McGuinness describe competency questions as a way to determine ontology scope and later test whether the ontology contains enough information. Ontology requirements literature similarly requires intended uses, end users, requirements, and motivating scenarios; NeOn and METHONTOLOGY both treat ontology development as a lifecycle with requirements, reuse, specification, conceptualization, implementation, and evaluation activities.

**Derived engineering rule:** Do not add a shared ontology term to a codebase, schema, graph, or generated type unless it is traceable to a competency question, ontology requirement, or accepted integration scenario. The rule is derived from the cited methodology work: if the ontology's expected questions are not stated, downstream software cannot distinguish shared meaning from a shared label.

**Incorrect — synthetic implementation sketch only**

```ts
// Synthetic anti-pattern: a shared term list with no scope, user, query, or test.
export const enterpriseTerms = [
  "Customer",
  "Account",
  "Party",
  "Owner",
  "Identity"
] as const;
```

**Correct — synthetic implementation sketch only**

```ts
type CompetencyQuestion = {
  id: string;
  question: string;
  requiredTerms: readonly string[];
  acceptanceTest: string;
};

export const customerAccountOntologyScope = {
  intendedUse: "Resolve account ownership for billing and compliance queries.",
  intendedUsers: ["billing-service", "compliance-reporting", "identity-service"],
  excludedUses: ["marketing segmentation", "credit-risk scoring"],
  competencyQuestions: [
    {
      id: "CQ-001",
      question: "Which legal party owned an account at a given effective time?",
      requiredTerms: ["LegalParty", "Account", "ownsAccount", "effectiveTime"],
      acceptanceTest: "query(CQ-001) returns exactly one owner for audited examples"
    }
  ]
} satisfies {
  intendedUse: string;
  intendedUsers: readonly string[];
  excludedUses: readonly string[];
  competencyQuestions: readonly CompetencyQuestion[];
};
```

Reference: [https://www.researchgate.net/publication/2653385_The_Role_of_Competency_Questions_in_Enterprise_Engineering](https://www.researchgate.net/publication/2653385_The_Role_of_Competency_Questions_in_Enterprise_Engineering), [https://protege.stanford.edu/publications/ontology_development/ontology101-noy-mcguinness.html](https://protege.stanford.edu/publications/ontology_development/ontology101-noy-mcguinness.html), [https://www.researchgate.net/publication/220830640_How_to_Write_and_Use_the_Ontology_Requirements_Specification_Document](https://www.researchgate.net/publication/220830640_How_to_Write_and_Use_the_Ontology_Requirements_Specification_Document), [https://journals.sagepub.com/doi/abs/10.3233/AO-150145](https://journals.sagepub.com/doi/abs/10.3233/AO-150145), [https://aaai.org/papers/0005-ss97-06-005-methontology-from-ontological-art-towards-ontological-engineering/](https://aaai.org/papers/0005-ss97-06-005-methontology-from-ontological-art-towards-ontological-engineering/), [https://www.researchgate.net/publication/302937543_Ontologies_Principles_methods_and_applications](https://www.researchgate.net/publication/302937543_Ontologies_Principles_methods_and_applications), [https://periodicos.ufmg.br/index.php/jidm/article/view/124](https://periodicos.ufmg.br/index.php/jidm/article/view/124), [https://www.sciencedirect.com/science/article/pii/S1071581985710816](https://www.sciencedirect.com/science/article/pii/S1071581985710816)

### 1.2 Treat Shared Ontology Terms as Semantic Contracts, Not Naming Conventions

**Impact: CRITICAL (Shared labels without formal definitions preserve ambiguity across services and schemas.)**

**Source-backed evidence:** Gruber defines an ontology as an explicit specification of a representational vocabulary, including definitions of classes, relations, functions, and other objects. OWL 2 specifies ontologies as formally defined vocabularies containing classes, properties, individuals, and data values; software-engineering ontology literature argues that ontologies help reduce terminology mismatch among people, organizations, tools, and software artifacts.

**Derived engineering rule:** A shared software ontology term should carry at least a stable identifier, definition, relation semantics, provenance, and intended use. A TypeScript type alias, enum value, database column, or JSON field name alone is not a semantic contract because the cited ontology literature treats formalized meaning — not spelling — as the object of sharing.

**Incorrect — synthetic implementation sketch only**

```ts
// Synthetic anti-pattern: identical label, unresolved semantics.
type CustomerId = string;

// In one service: payer.
// In another service: login subject.
// In another service: legal account owner.
export function loadCustomer(id: CustomerId) {
  return fetch(`/customers/${id}`);
}
```

**Correct — synthetic implementation sketch only**

```ts
type OntologyTerm = {
  iri: string;
  preferredLabel: string;
  definition: string;
  broader?: string;
  relations: readonly string[];
  provenance: string;
};

export const LegalCustomer: OntologyTerm = {
  iri: "https://ontology.example/customer/v2#LegalCustomer",
  preferredLabel: "Legal customer",
  definition:
    "A legal party that has an accountable commercial relationship with the organization.",
  broader: "https://ontology.example/party/v1#LegalParty",
  relations: ["ownsAccount", "hasBillingResponsibility"],
  provenance: "ADR-ONT-042; accepted by billing, compliance, and identity owners"
};
```

Reference: [https://tomgruber.org/writing/ontolingua-kaj-1993/](https://tomgruber.org/writing/ontolingua-kaj-1993/), [https://www.sciencedirect.com/science/article/pii/S1071581985710816](https://www.sciencedirect.com/science/article/pii/S1071581985710816), [https://www.w3.org/TR/owl2-overview/](https://www.w3.org/TR/owl2-overview/), [https://www.researchgate.net/publication/302937543_Ontologies_Principles_methods_and_applications](https://www.researchgate.net/publication/302937543_Ontologies_Principles_methods_and_applications), [https://link.springer.com/book/10.1007/3-540-34518-3](https://link.springer.com/book/10.1007/3-540-34518-3), [https://www.researchgate.net/publication/228386661_Applications_of_ontologies_in_software_engineering](https://www.researchgate.net/publication/228386661_Applications_of_ontologies_in_software_engineering), [https://www.researchgate.net/publication/224101625_Using_ontology_to_support_development_of_software_architectures](https://www.researchgate.net/publication/224101625_Using_ontology_to_support_development_of_software_architectures)

---

## 2. Architecture — Reuse, Modularity, and Expressivity

**Impact: HIGH**

Architectural decisions — what to reuse from existing reference and upper ontologies, how to decompose into modules with stable interfaces, and which formalism to commit to — set the lifetime cost of the ontology. Monolithic vocabularies, locally-minted synonyms of well-known terms, and choosing a logic stronger than the required inference each create durable maintenance and reasoning overhead.

### 2.1 Choose Ontology Expressivity From the Required Inference and Validation Workload

**Impact: HIGH (Excessive expressivity raises implementation and reasoning cost; insufficient expressivity fails to encode required semantics.)**

**Source-backed evidence:** OWL 2 defines a language with formal semantics for classes, properties, individuals, and data values; OWL 2 Profiles explicitly trade expressive power for reasoning efficiency and describe EL, QL, and RL profiles for different reasoning workloads. SHACL is a W3C language for validating RDF graphs against shape constraints, while DL-Lite and OBDA literature target tractable query answering and access to data through conceptual schemas and mappings.

**Derived engineering rule:** Select the weakest formalism that satisfies the software task: SKOS-like vocabularies for lightweight classification, OWL profiles for required open-world inference, SHACL for closed-world data validation, R2RML/OBDA mappings for relational data access, and full OWL DL only when the required reasoning justifies it. This is derived from the cited profile, validation, and OBDA literature, not from a claim that one formalism dominates all others.

**Incorrect — synthetic implementation sketch only**

```ts
// Synthetic anti-pattern: maximum formalism selected without workload analysis.
export const ontologyRuntime = {
  language: "OWL-DL",
  reasoner: "global",
  reason: "formal ontologies should always use the strongest logic"
};
```

**Correct — synthetic implementation sketch only**

```ts
type OntologyFormalismDecision = {
  requirement: string;
  selectedFormalism: "SKOS" | "OWL2-EL" | "OWL2-QL" | "OWL2-RL" | "SHACL" | "R2RML";
  reason: string;
};

export const formalismDecisions: readonly OntologyFormalismDecision[] = [
  {
    requirement: "Validate required fields and cardinalities in incoming RDF data.",
    selectedFormalism: "SHACL",
    reason: "Closed-world constraint validation against data graphs."
  },
  {
    requirement: "Expose relational source tables through ontology-level queries.",
    selectedFormalism: "R2RML",
    reason: "Mapping relational data to RDF terms for ontology-based access."
  },
  {
    requirement: "Classify a large biomedical-style class hierarchy.",
    selectedFormalism: "OWL2-EL",
    reason: "OWL 2 EL is designed for large ontologies with polynomial-time reasoning."
  }
];
```

Reference: [https://www.w3.org/TR/owl2-overview/](https://www.w3.org/TR/owl2-overview/), [https://www.w3.org/TR/owl2-profiles/](https://www.w3.org/TR/owl2-profiles/), [https://papers.ssrn.com/sol3/papers.cfm?abstract_id=3199003](https://papers.ssrn.com/sol3/papers.cfm?abstract_id=3199003), [https://research.manchester.ac.uk/en/publications/reducing-owl-entailment-to-description-logic-satisfiability](https://research.manchester.ac.uk/en/publications/reducing-owl-entailment-to-description-logic-satisfiability), [https://www.w3.org/TR/shacl/](https://www.w3.org/TR/shacl/), [https://www.diag.uniroma1.it/rosati/publications/CDLLPRR08.htm](https://www.diag.uniroma1.it/rosati/publications/CDLLPRR08.htm), [https://www.ijcai.org/proceedings/2018/777](https://www.ijcai.org/proceedings/2018/777)

### 2.2 Modularize Shared Ontologies Around Stable Software and Data Boundaries

**Impact: HIGH (Monolithic ontologies increase reasoning, import, review, and downstream change cost.)**

**Source-backed evidence:** Modular ontology research reports that ontologies become larger and more complex to manage as their number and size grow, and that modularization supports extracting or managing parts relevant to particular scenarios. Rector et al. classify modular OWL development use cases including organization, stable software-interface bindings, localization, extension, and encapsulation; Cuenca Grau et al. formalize modular reuse using concepts such as safety, conservative extension, and module extraction.

**Derived engineering rule:** A software system should import the smallest ontology module that satisfies its competency questions and integration contract. Treat an ontology module like a semantic API: version it, publish its imports, test its closure properties, and avoid making every service depend on a whole-enterprise ontology.

**Incorrect — synthetic implementation sketch only**

```ts
// Synthetic anti-pattern: every service imports the entire enterprise ontology.
import { EnterpriseOntology } from "@example/ontology/all";

export const ownerClass = EnterpriseOntology.Customer.Account.Owner;
```

**Correct — synthetic implementation sketch only**

```ts
type OntologyModuleDependency = {
  moduleIri: string;
  version: string;
  requiredFor: readonly string[];
};

export const billingOntologyDependencies: readonly OntologyModuleDependency[] = [
  {
    moduleIri: "https://ontology.example/customer-core",
    version: "2.1.0",
    requiredFor: ["CQ-001", "CQ-004"]
  },
  {
    moduleIri: "https://ontology.example/account-relations",
    version: "1.8.0",
    requiredFor: ["CQ-001"]
  }
];
```

Reference: [https://journals.sagepub.com/doi/pdf/10.3233/ICA-2009-0315](https://journals.sagepub.com/doi/pdf/10.3233/ICA-2009-0315), [https://www.researchgate.net/publication/262175610_Engineering_use_cases_for_modular_development_of_ontologies_in_OWL](https://www.researchgate.net/publication/262175610_Engineering_use_cases_for_modular_development_of_ontologies_in_OWL), [https://www.researchgate.net/publication/220543139_Modular_Reuse_of_Ontologies_Theory_and_Practice](https://www.researchgate.net/publication/220543139_Modular_Reuse_of_Ontologies_Theory_and_Practice), [https://link.springer.com/book/10.1007/978-3-642-01907-4](https://link.springer.com/book/10.1007/978-3-642-01907-4), [https://journals.sagepub.com/doi/10.3233/AO-2011-0087](https://journals.sagepub.com/doi/10.3233/AO-2011-0087), [https://research.manchester.ac.uk/en/publications/managing-multiple-and-distributed-ontologies-on-the-semantic-web](https://research.manchester.ac.uk/en/publications/managing-multiple-and-distributed-ontologies-on-the-semantic-web)

### 2.3 Reuse Reference Ontologies When They Match the Competency Questions

**Impact: HIGH (Redundant local vocabularies increase integration and alignment work.)**

**Source-backed evidence:** The OBO Foundry paper states that ontology proliferation can itself create obstacles to integration and proposes coordinated principles for interoperable biomedical ontologies. The Gene Ontology paper presents a controlled vocabulary for unifying biological annotations; MIREOT proposes guidelines for referencing required external ontology terms because full OWL imports can be impractical, changing, or inference-disruptive.

**Derived engineering rule:** Before minting a local shared term, check whether a reference ontology, upper ontology, domain ontology, or ontology design pattern already satisfies the competency question. Reuse should be selective and justified: the cited MIREOT work supports referencing needed external terms rather than blindly importing entire external ontologies.

**Incorrect — synthetic implementation sketch only**

```ts
// Synthetic anti-pattern: local synonym created before checking reference terms.
export const CellPart = {
  iri: "https://ontology.example/lab#CellPart",
  definition: "Something that is part of a biological cell."
};
```

**Correct — synthetic implementation sketch only**

```ts
type ExternalOntologyReference = {
  localAlias: string;
  externalIri: string;
  sourceOntology: string;
  reuseReason: string;
};

export const CellularComponentRef: ExternalOntologyReference = {
  localAlias: "CellularComponent",
  externalIri: "http://purl.obolibrary.org/obo/GO_0005575",
  sourceOntology: "Gene Ontology",
  reuseReason:
    "Matches the competency question about annotating assay observations by cellular component."
};
```

Reference: [https://www.nature.com/articles/nbt1346](https://www.nature.com/articles/nbt1346), [https://www.nature.com/articles/ng0500_25](https://www.nature.com/articles/ng0500_25), [https://journals.sagepub.com/doi/10.3233/AO-2011-0087](https://journals.sagepub.com/doi/10.3233/AO-2011-0087), [https://www.researchgate.net/publication/221466152_Ontology_Design_Patterns_for_Semantic_Web_Content](https://www.researchgate.net/publication/221466152_Ontology_Design_Patterns_for_Semantic_Web_Content), [https://www.researchgate.net/publication/227215903_Ontology_Design_Patterns](https://www.researchgate.net/publication/227215903_Ontology_Design_Patterns), [https://journals.sagepub.com/doi/10.3233/AO-220262](https://journals.sagepub.com/doi/10.3233/AO-220262), [https://journals.sagepub.com/doi/10.3233/AO-210259](https://journals.sagepub.com/doi/10.3233/AO-210259)

---

## 3. Validation — Release-Gate Checks

**Impact: CRITICAL**

A shared ontology is executable semantic infrastructure. Releases that ship without reasoner consistency checks, competency-question entailment, SHACL/ShEx data-shape validation, and empirically-grounded pitfall scans deliver distributed defects directly into downstream consumers — annotations, queries, generated code, and integration mappings inherit every undetected error.

### 3.1 Validate Ontology Releases With Competency Tests, Reasoners, Shapes, and Pitfall Checks

**Impact: CRITICAL (Shared ontology defects propagate into schemas, mappings, generated code, queries, and data validation.)**

**Source-backed evidence:** Ontology-evaluation surveys define evaluation as judging whether an ontology fits an application criterion; OntoClean targets taxonomic modeling errors using formal metaproperties; OOPS catalogs ontology pitfalls from empirical analysis of hundreds of ontologies. SHACL defines machine-checkable validation reports for RDF graphs, and RDFUnit proposes test-driven quality assessment for vocabularies, ontologies, and knowledge bases.

**Derived engineering rule:** A shared ontology release should not be accepted solely because files parse or terms look reasonable. Require executable checks tied to competency questions, logical consistency, shape conformance, known ontology pitfalls, and data-quality metrics relevant to the intended software application.

**Incorrect — synthetic implementation sketch only**

```ts
// Synthetic anti-pattern: release gate verifies only that files exist.
export const ontologyReleaseGate = {
  parses: true,
  filesPresent: ["ontology.ttl"],
  semanticTests: []
};
```

**Correct — synthetic implementation sketch only**

```ts
type OntologyReleaseCheck =
  | "competency-questions-pass"
  | "reasoner-consistency-pass"
  | "no-unsatisfiable-classes"
  | "shacl-production-samples-pass"
  | "critical-pitfalls-zero"
  | "mapping-tests-pass";

export const ontologyReleaseGate: readonly OntologyReleaseCheck[] = [
  "competency-questions-pass",
  "reasoner-consistency-pass",
  "no-unsatisfiable-classes",
  "shacl-production-samples-pass",
  "critical-pitfalls-zero",
  "mapping-tests-pass"
];
```

Reference: [https://www.researchgate.net/publication/228857266_A_survey_of_ontology_evaluation_techniques](https://www.researchgate.net/publication/228857266_A_survey_of_ontology_evaluation_techniques), [https://cacm.acm.org/research/evaluating-ontological-decisions-with-ontoclean/](https://cacm.acm.org/research/evaluating-ontological-decisions-with-ontoclean/), [https://www.semanticscholar.org/paper/OOPS%21-%28OntOlogy-Pitfall-Scanner%21%29%3A-An-On-line-Tool-Poveda-Villal%C3%B3n-G%C3%B3mez-P%C3%A9rez/28f692a5b6e61ab48bece1221f4e17e05a9a8139](https://www.semanticscholar.org/paper/OOPS%21-%28OntOlogy-Pitfall-Scanner%21%29%3A-An-On-line-Tool-Poveda-Villal%C3%B3n-G%C3%B3mez-P%C3%A9rez/28f692a5b6e61ab48bece1221f4e17e05a9a8139), [https://www.w3.org/TR/shacl/](https://www.w3.org/TR/shacl/), [https://www.researchgate.net/publication/259828947_Test-driven_Evaluation_of_Linked_Data_Quality](https://www.researchgate.net/publication/259828947_Test-driven_Evaluation_of_Linked_Data_Quality), [https://content.iospress.com/articles/semantic-web/sw175](https://content.iospress.com/articles/semantic-web/sw175), [https://link.springer.com/book/10.1007/978-3-031-79478-0](https://link.springer.com/book/10.1007/978-3-031-79478-0)

---

## 4. Evolution — Versioning and Mappings

**Impact: CRITICAL**

Ontology changes affect data, APIs, queries, generated code, annotations, reasoning results, and downstream integrations. The two artefacts that keep change tractable are explicit versioning with provenance, deprecation, and migration rules, and first-class cross-ontology mappings that make heterogeneity visible, reviewable, and testable instead of hidden inside service code.

### 4.1 Make Schema-to-Ontology and Ontology-to-Ontology Mappings First-Class Artifacts

**Impact: HIGH (Shared ontologies do not eliminate semantic heterogeneity; they expose where mappings must be specified and tested.)**

**Source-backed evidence:** Schema-matching research identifies matching as a basic problem in data integration, semantic query processing, and related systems; ontology-matching surveys treat alignment as a distinct research problem with continuing challenges. Ontology-based data access literature uses mappings between local data sources and global conceptual schemas, while R2RML specifies customized mappings from relational databases to RDF datasets.

**Derived engineering rule:** Do not bury ontology mappings inside adapter code, SQL string manipulation, field-normalization functions, or undocumented ETL conventions. Store mappings as versioned, testable, reviewable artifacts with source schema, target ontology term, transformation rule, cardinality assumption, provenance, and affected competency questions.

**Incorrect — synthetic implementation sketch only**

```ts
// Synthetic anti-pattern: semantic mapping hidden in procedural normalization.
export function normalizeOwner(row: Record<string, unknown>) {
  return {
    ownerId: row.client_id ?? row.party_ref ?? row.account_holder_id
  };
}
```

**Correct — synthetic implementation sketch only**

```ts
type SourceToOntologyMapping = {
  sourceField: string;
  targetTermIri: string;
  relation: string;
  cardinality: "one-to-one" | "many-to-one" | "one-to-many";
  transformation: string;
  tests: readonly string[];
};

export const billingOwnerMapping: SourceToOntologyMapping = {
  sourceField: "billing.client_id",
  targetTermIri: "https://ontology.example/customer/v2#LegalCustomer",
  relation: "identifies",
  cardinality: "many-to-one",
  transformation: "Resolve billing.client_id through billing_party_registry.",
  tests: ["client-id-resolves-to-legal-customer", "cq-001-owner-query-uses-mapping"]
};
```

Reference: [https://vldb.org/vldb_journal/index.php/component/article_manager/article/839](https://vldb.org/vldb_journal/index.php/component/article_manager/article/839), [https://www.researchgate.net/publication/260324688_Ontology_Matching_State_of_the_Art_and_Future_Challenges](https://www.researchgate.net/publication/260324688_Ontology_Matching_State_of_the_Art_and_Future_Challenges), [https://www.researchgate.net/publication/221212846_Schema_and_ontology_matching_with_COMA](https://www.researchgate.net/publication/221212846_Schema_and_ontology_matching_with_COMA), [https://www.researchgate.net/publication/200122923_Ontology-based_integration_of_information_---_a_survey_of_existing_approaches](https://www.researchgate.net/publication/200122923_Ontology-based_integration_of_information_---_a_survey_of_existing_approaches), [https://www.ijcai.org/proceedings/2018/777](https://www.ijcai.org/proceedings/2018/777), [https://www.w3.org/TR/r2rml/](https://www.w3.org/TR/r2rml/), [https://www.diag.uniroma1.it/rosati/publications/CDLLPRR08.htm](https://www.diag.uniroma1.it/rosati/publications/CDLLPRR08.htm), [https://link.springer.com/article/10.1007/s10844-011-0184-1](https://link.springer.com/article/10.1007/s10844-011-0184-1), [https://www.sciencedirect.com/science/article/pii/S1570826817300276](https://www.sciencedirect.com/science/article/pii/S1570826817300276)

### 4.2 Version Ontology Changes as Semantic Migrations

**Impact: CRITICAL (Ontology changes affect meaning, mappings, annotations, queries, generated types, and downstream integrations.)**

**Source-backed evidence:** Ontology-versioning research states that ontologies are not static and that changing domains, tasks, or conceptualizations require explicit relations between ontology revisions. Ontology-management and evolution literature covers comparing, aligning, merging, maintaining versions, translating ontologies, and keeping ontologies updated as requirements or domains change; PAV describes provenance, authoring, and versioning metadata for web resources.

**Derived engineering rule:** Treat every nontrivial ontology change like a semantic migration, not a text edit. A change that alters class meaning, property meaning, disjointness, domain/range, mapping semantics, or competency-question answers should include version identifiers, provenance, deprecation notes, migration tests, affected modules, and affected software consumers.

**Incorrect — synthetic implementation sketch only**

```ts
// Synthetic anti-pattern: same IRI, changed meaning, no migration metadata.
export const Customer = {
  iri: "https://ontology.example/customer#Customer",
  definition: "A person who can log in to the product."
};

// Later, silently changed:
export const CustomerChanged = {
  iri: "https://ontology.example/customer#Customer",
  definition: "A legal party with billable account responsibility."
};
```

**Correct — synthetic implementation sketch only**

```ts
type OntologyMigration = {
  fromIri: string;
  toIri: string;
  changeType: "rename" | "split" | "merge" | "semantic-narrowing" | "semantic-broadening";
  affectedCompetencyQuestions: readonly string[];
  migrationTests: readonly string[];
  provenance: string;
};

export const CustomerMigrationV1ToV2: OntologyMigration = {
  fromIri: "https://ontology.example/customer/v1#Customer",
  toIri: "https://ontology.example/customer/v2#LegalCustomer",
  changeType: "semantic-narrowing",
  affectedCompetencyQuestions: ["CQ-001", "CQ-009"],
  migrationTests: ["legacy-customer-to-legal-customer-mapping", "cq-001-still-answers"],
  provenance: "PAV-compatible release note; approved in ONT-1042"
};
```

Reference: [https://www.researchgate.net/publication/2377424_Ontology_versioning_on_the_Semantic_Web](https://www.researchgate.net/publication/2377424_Ontology_versioning_on_the_Semantic_Web), [https://www.researchgate.net/publication/3454215_Ontology_versioning_in_an_ontology_management_framework](https://www.researchgate.net/publication/3454215_Ontology_versioning_in_an_ontology_management_framework), [https://www.cambridge.org/core/services/aop-cambridge-core/content/view/CE4387A954917B7CA3282CE25FAC09FA/S0269888913000349a.pdf/ontology-evolution-a-process-centric-survey.pdf](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/CE4387A954917B7CA3282CE25FAC09FA/S0269888913000349a.pdf/ontology-evolution-a-process-centric-survey.pdf), [https://research.manchester.ac.uk/en/publications/managing-multiple-and-distributed-ontologies-on-the-semantic-web](https://research.manchester.ac.uk/en/publications/managing-multiple-and-distributed-ontologies-on-the-semantic-web), [https://jbiomedsem.biomedcentral.com/articles/10.1186/2041-1480-4-37](https://jbiomedsem.biomedcentral.com/articles/10.1186/2041-1480-4-37), [https://www.aaai.org/Library/AAAI/2000/aaai00-070.php](https://www.aaai.org/Library/AAAI/2000/aaai00-070.php), [https://www.nature.com/articles/nbt1346](https://www.nature.com/articles/nbt1346)

---

## 5. Governance — Editorial Workflow and Measurement

**Impact: HIGH**

A shared ontology encodes social agreement among stakeholders, so the editorial workflow and the value-measurement story are part of the engineering surface. Without explicit roles, proposal flow, domain-expert review, and operational measures of use (CQ pass rate, mapping reuse, adapter logic removed), the ontology drifts toward whichever local interpretation wins the last merge.

### 5.1 Govern Shared Ontologies With Collaborative Editorial Workflow

**Impact: HIGH (Shared ontologies encode cross-team commitments; single-actor edits create local semantics with global blast radius.)**

**Source-backed evidence:** Collaborative ontology-engineering surveys describe community-driven ontology engineering as a central paradigm and argue that useful ontologies require environments supporting collaboration and user participation. WebProtégé is described as a web-based ontology editor and knowledge-acquisition tool with collaboration support; other work proposes collaborative ontology design, explicit editorial workflows, change representation, and version management.

**Derived engineering rule:** A shared ontology change should pass through an editorial process involving affected domain experts, data owners, software owners, and ontology maintainers. Review should include affected competency questions, mappings, downstream consumers, validation results, and migration requirements.

**Incorrect — synthetic implementation sketch only**

```ts
// Synthetic anti-pattern: semantic change merged as an ordinary local refactor.
await ontologyRepository.merge({
  change: "Rename Party to Customer because Customer sounds clearer.",
  proposer: "one-service-owner",
  reviewers: [],
  affectedMappings: [],
  affectedCompetencyQuestions: []
});
```

**Correct — synthetic implementation sketch only**

```ts
type OntologyChangeProposal = {
  id: string;
  affectedTerms: readonly string[];
  affectedCompetencyQuestions: readonly string[];
  affectedMappings: readonly string[];
  requiredReviewers: readonly string[];
  validationEvidence: readonly string[];
  decisionRecord: string;
};

export const partyCustomerBoundaryProposal: OntologyChangeProposal = {
  id: "ONT-742",
  affectedTerms: ["Party", "LegalCustomer", "NaturalPerson"],
  affectedCompetencyQuestions: ["CQ-001", "CQ-017"],
  affectedMappings: ["billing.client_id -> LegalCustomer"],
  requiredReviewers: ["domain-owner", "data-architecture", "billing-service-owner"],
  validationEvidence: ["cq-tests-pass", "mapping-tests-pass", "shacl-tests-pass"],
  decisionRecord: "ADR-ONT-742"
};
```

Reference: [https://www.cambridge.org/core/services/aop-cambridge-core/content/view/S0269888913000192](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/S0269888913000192), [https://journals.sagepub.com/doi/10.3233/SW-2012-0057](https://journals.sagepub.com/doi/10.3233/SW-2012-0057), [https://cacm.acm.org/research/a-collaborative-approach-to-ontology-design/](https://cacm.acm.org/research/a-collaborative-approach-to-ontology-design/), [https://www.sciencedirect.com/science/article/abs/pii/S1570826811000503](https://www.sciencedirect.com/science/article/abs/pii/S1570826811000503), [https://journals.sagepub.com/doi/abs/10.3233/AO-150145](https://journals.sagepub.com/doi/abs/10.3233/AO-150145), [https://www.researchgate.net/publication/220830640_How_to_Write_and_Use_the_Ontology_Requirements_Specification_Document](https://www.researchgate.net/publication/220830640_How_to_Write_and_Use_the_Ontology_Requirements_Specification_Document), [https://www.nature.com/articles/nbt1346](https://www.nature.com/articles/nbt1346)

### 5.2 Measure Ontology Value by Operational Interoperability, Not Term Count

**Impact: MEDIUM (A larger ontology is not necessarily more useful; value depends on whether software and data tasks improve.)**

**Source-backed evidence:** Ontology-evaluation literature frames quality relative to application criteria; linked-data quality literature defines quality using dimensions and metrics related to fitness for use. The Gene Ontology and OBO Foundry papers motivate ontologies through annotation, unification, and integration; OBDA case studies and surveys evaluate ontology use through conceptual data access, mappings, and query translation rather than taxonomy size alone.

**Derived engineering rule:** Track operational measures: competency-question pass rate, mapping reuse, validation failures, query success, annotation coverage, number of adapters removed, number of downstream consumers, and migration defect rate. Term count can be an inventory metric, but the cited evaluation, data-quality, and integration literature does not make term count itself a sufficient measure of ontology value.

**Incorrect — synthetic implementation sketch only**

```ts
// Synthetic anti-pattern: ontology success reduced to vocabulary size.
export const ontologyHealth = {
  termCount: 12_000,
  status: "healthy"
};
```

**Correct — synthetic implementation sketch only**

```ts
type OntologyOperationalMetrics = {
  competencyQuestionPassRate: number;
  shaclValidationPassRate: number;
  mappingTestPassRate: number;
  downstreamConsumerCount: number;
  reusedMappingCount: number;
  adapterBranchesRemoved: number;
};

export const ontologyHealth: OntologyOperationalMetrics = {
  competencyQuestionPassRate: 0.98,
  shaclValidationPassRate: 0.99,
  mappingTestPassRate: 0.97,
  downstreamConsumerCount: 14,
  reusedMappingCount: 38,
  adapterBranchesRemoved: 22
};
```

Reference: [https://www.researchgate.net/publication/228857266_A_survey_of_ontology_evaluation_techniques](https://www.researchgate.net/publication/228857266_A_survey_of_ontology_evaluation_techniques), [https://content.iospress.com/articles/semantic-web/sw175](https://content.iospress.com/articles/semantic-web/sw175), [https://www.nature.com/articles/ng0500_25](https://www.nature.com/articles/ng0500_25), [https://www.nature.com/articles/nbt1346](https://www.nature.com/articles/nbt1346), [https://www.ijcai.org/proceedings/2018/777](https://www.ijcai.org/proceedings/2018/777), [https://www.sciencedirect.com/science/article/pii/S1570826817300276](https://www.sciencedirect.com/science/article/pii/S1570826817300276), [https://www.researchgate.net/publication/259828947_Test-driven_Evaluation_of_Linked_Data_Quality](https://www.researchgate.net/publication/259828947_Test-driven_Evaluation_of_Linked_Data_Quality)

---

## References

1. [https://tomgruber.org/writing/ontolingua-kaj-1993/](https://tomgruber.org/writing/ontolingua-kaj-1993/)
2. [https://protege.stanford.edu/publications/ontology_development/ontology101-noy-mcguinness.html](https://protege.stanford.edu/publications/ontology_development/ontology101-noy-mcguinness.html)
3. [https://www.w3.org/TR/owl2-overview/](https://www.w3.org/TR/owl2-overview/)
4. [https://www.w3.org/TR/owl2-profiles/](https://www.w3.org/TR/owl2-profiles/)
5. [https://www.w3.org/TR/shacl/](https://www.w3.org/TR/shacl/)
6. [https://www.w3.org/TR/r2rml/](https://www.w3.org/TR/r2rml/)
7. [https://www.nature.com/articles/nbt1346](https://www.nature.com/articles/nbt1346)
8. [https://www.nature.com/articles/ng0500_25](https://www.nature.com/articles/ng0500_25)
9. [https://link.springer.com/book/10.1007/978-3-642-38721-0](https://link.springer.com/book/10.1007/978-3-642-38721-0)
10. [https://www.ijcai.org/proceedings/2018/777](https://www.ijcai.org/proceedings/2018/777)
11. [https://www.cambridge.org/core/services/aop-cambridge-core/content/view/S0269888913000192](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/S0269888913000192)
12. [https://link.springer.com/book/10.1007/978-3-031-79478-0](https://link.springer.com/book/10.1007/978-3-031-79478-0)
