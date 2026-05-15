---
title: Reuse Reference Ontologies When They Match the Competency Questions
impact: HIGH
impactDescription: Redundant local vocabularies increase integration and alignment work.
evidenceStrength: HIGH
corroborationCount: 7
claimType: Source-backed evidence plus derived software guidance
tags: architecture, reuse, reference-ontology, obo, mireot, ontology-design-patterns, upper-ontology
---

## Reuse Reference Ontologies When They Match the Competency Questions

**Source-backed evidence:** The OBO Foundry paper states that ontology proliferation can itself create obstacles to integration and proposes coordinated principles for interoperable biomedical ontologies. The Gene Ontology paper presents a controlled vocabulary for unifying biological annotations; MIREOT proposes guidelines for referencing required external ontology terms because full OWL imports can be impractical, changing, or inference-disruptive.

**Derived engineering rule:** Before minting a local shared term, check whether a reference ontology, upper ontology, domain ontology, or ontology design pattern already satisfies the competency question. Reuse should be selective and justified: the cited MIREOT work supports referencing needed external terms rather than blindly importing entire external ontologies.

**Incorrect — synthetic implementation sketch only:**

```ts
// Synthetic anti-pattern: local synonym created before checking reference terms.
export const CellPart = {
  iri: "https://ontology.example/lab#CellPart",
  definition: "Something that is part of a biological cell."
};
```

**Correct — synthetic implementation sketch only:**

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

References:
- [Smith et al. — The OBO Foundry](https://www.nature.com/articles/nbt1346)
- [Ashburner et al. — Gene Ontology: Tool for the Unification of Biology](https://www.nature.com/articles/ng0500_25)
- [Courtot et al. — MIREOT: The Minimum Information to Reference an External Ontology Term](https://journals.sagepub.com/doi/10.3233/AO-2011-0087)
- [Gangemi — Ontology Design Patterns for Semantic Web Content](https://www.researchgate.net/publication/221466152_Ontology_Design_Patterns_for_Semantic_Web_Content)
- [Gangemi & Presutti — Ontology Design Patterns](https://www.researchgate.net/publication/227215903_Ontology_Design_Patterns)
- [Otte, Beverley & Ruttenberg — BFO: Basic Formal Ontology](https://journals.sagepub.com/doi/10.3233/AO-220262)
- [Borgo et al. — DOLCE: A Descriptive Ontology for Linguistic and Cognitive Engineering](https://journals.sagepub.com/doi/10.3233/AO-210259)
