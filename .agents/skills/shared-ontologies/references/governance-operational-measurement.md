---
title: Measure Ontology Value by Operational Interoperability, Not Term Count
impact: MEDIUM
impactDescription: A larger ontology is not necessarily more useful; value depends on whether software and data tasks improve.
evidenceStrength: HIGH
corroborationCount: 7
claimType: Source-backed evidence plus derived software guidance
tags: governance, measurement, quality, interoperability, adoption, linked-data-quality
---

## Measure Ontology Value by Operational Interoperability, Not Term Count

**Source-backed evidence:** Ontology-evaluation literature frames quality relative to application criteria; linked-data quality literature defines quality using dimensions and metrics related to fitness for use. The Gene Ontology and OBO Foundry papers motivate ontologies through annotation, unification, and integration; OBDA case studies and surveys evaluate ontology use through conceptual data access, mappings, and query translation rather than taxonomy size alone.

**Derived engineering rule:** Track operational measures: competency-question pass rate, mapping reuse, validation failures, query success, annotation coverage, number of adapters removed, number of downstream consumers, and migration defect rate. Term count can be an inventory metric, but the cited evaluation, data-quality, and integration literature does not make term count itself a sufficient measure of ontology value.

**Incorrect — synthetic implementation sketch only:**

```ts
// Synthetic anti-pattern: ontology success reduced to vocabulary size.
export const ontologyHealth = {
  termCount: 12_000,
  status: "healthy"
};
```

**Correct — synthetic implementation sketch only:**

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

References:
- [Brank, Grobelnik & Mladenić — A Survey of Ontology Evaluation Techniques](https://www.researchgate.net/publication/228857266_A_survey_of_ontology_evaluation_techniques)
- [Zaveri et al. — Quality Assessment for Linked Data: A Survey](https://content.iospress.com/articles/semantic-web/sw175)
- [Ashburner et al. — Gene Ontology: Tool for the Unification of Biology](https://www.nature.com/articles/ng0500_25)
- [Smith et al. — The OBO Foundry](https://www.nature.com/articles/nbt1346)
- [Xiao et al. — Ontology-Based Data Access: A Survey](https://www.ijcai.org/proceedings/2018/777)
- [Kharlamov et al. — Ontology Based Data Access in Statoil](https://www.sciencedirect.com/science/article/pii/S1570826817300276)
- [Kontokostas et al. — Test-Driven Evaluation of Linked Data Quality](https://www.researchgate.net/publication/259828947_Test-driven_Evaluation_of_Linked_Data_Quality)
