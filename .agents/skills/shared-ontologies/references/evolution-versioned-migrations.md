---
title: Version Ontology Changes as Semantic Migrations
impact: CRITICAL
impactDescription: Ontology changes affect meaning, mappings, annotations, queries, generated types, and downstream integrations.
evidenceStrength: VERY HIGH
corroborationCount: 7
claimType: Source-backed evidence plus derived software guidance
tags: evolution, versioning, ontology-evolution, provenance, migration, deprecation
---

## Version Ontology Changes as Semantic Migrations

**Source-backed evidence:** Ontology-versioning research states that ontologies are not static and that changing domains, tasks, or conceptualizations require explicit relations between ontology revisions. Ontology-management and evolution literature covers comparing, aligning, merging, maintaining versions, translating ontologies, and keeping ontologies updated as requirements or domains change; PAV describes provenance, authoring, and versioning metadata for web resources.

**Derived engineering rule:** Treat every nontrivial ontology change like a semantic migration, not a text edit. A change that alters class meaning, property meaning, disjointness, domain/range, mapping semantics, or competency-question answers should include version identifiers, provenance, deprecation notes, migration tests, affected modules, and affected software consumers.

**Incorrect — synthetic implementation sketch only:**

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

**Correct — synthetic implementation sketch only:**

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

References:
- [Klein & Fensel — Ontology Versioning on the Semantic Web](https://www.researchgate.net/publication/2377424_Ontology_versioning_on_the_Semantic_Web)
- [Noy & Musen — Ontology Versioning in an Ontology Management Framework](https://www.researchgate.net/publication/3454215_Ontology_versioning_in_an_ontology_management_framework)
- [Zablith et al. — Ontology Evolution: A Process-Centric Survey](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/CE4387A954917B7CA3282CE25FAC09FA/S0269888913000349a.pdf/ontology-evolution-a-process-centric-survey.pdf)
- [Maedche, Motik & Stojanovic — Managing Multiple and Distributed Ontologies on the Semantic Web](https://research.manchester.ac.uk/en/publications/managing-multiple-and-distributed-ontologies-on-the-semantic-web)
- [Ciccarese et al. — PAV Ontology: Provenance, Authoring and Versioning](https://jbiomedsem.biomedcentral.com/articles/10.1186/2041-1480-4-37)
- [Noy & Musen — PROMPT: Algorithm and Tool for Automated Ontology Merging and Alignment](https://www.aaai.org/Library/AAAI/2000/aaai00-070.php)
- [Smith et al. — The OBO Foundry](https://www.nature.com/articles/nbt1346)
