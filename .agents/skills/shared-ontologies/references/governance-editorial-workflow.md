---
title: Govern Shared Ontologies With Collaborative Editorial Workflow
impact: HIGH
impactDescription: Shared ontologies encode cross-team commitments; single-actor edits create local semantics with global blast radius.
evidenceStrength: HIGH
corroborationCount: 7
claimType: Source-backed evidence plus derived software guidance
tags: governance, collaboration, editorial-workflow, webprotege, change-management
---

## Govern Shared Ontologies With Collaborative Editorial Workflow

**Source-backed evidence:** Collaborative ontology-engineering surveys describe community-driven ontology engineering as a central paradigm and argue that useful ontologies require environments supporting collaboration and user participation. WebProtégé is described as a web-based ontology editor and knowledge-acquisition tool with collaboration support; other work proposes collaborative ontology design, explicit editorial workflows, change representation, and version management.

**Derived engineering rule:** A shared ontology change should pass through an editorial process involving affected domain experts, data owners, software owners, and ontology maintainers. Review should include affected competency questions, mappings, downstream consumers, validation results, and migration requirements.

**Incorrect — synthetic implementation sketch only:**

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

**Correct — synthetic implementation sketch only:**

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

References:
- [Simperl & Luczak-Rösch — Collaborative Ontology Engineering: A Survey](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/S0269888913000192)
- [Tudorache et al. — WebProtégé: A Collaborative Ontology Editor and Knowledge Acquisition Tool](https://journals.sagepub.com/doi/10.3233/SW-2012-0057)
- [Holsapple & Joshi — A Collaborative Approach to Ontology Design](https://cacm.acm.org/research/a-collaborative-approach-to-ontology-design/)
- [Palma et al. — A Holistic Approach to Collaborative Ontology Development Based on Change Management](https://www.sciencedirect.com/science/article/abs/pii/S1570826811000503)
- [Suárez-Figueroa et al. — The NeOn Methodology Framework](https://journals.sagepub.com/doi/abs/10.3233/AO-150145)
- [Suárez-Figueroa et al. — Ontology Requirements Specification Document](https://www.researchgate.net/publication/220830640_How_to_Write_and_Use_the_Ontology_Requirements_Specification_Document)
- [Smith et al. — The OBO Foundry](https://www.nature.com/articles/nbt1346)
