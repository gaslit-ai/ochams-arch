---
title: Define Ontology Scope With Competency Questions Before Adding Shared Terms
impact: CRITICAL
impactDescription: Unscoped shared vocabularies create ambiguous integration artifacts rather than shared semantics.
evidenceStrength: VERY HIGH
corroborationCount: 8
claimType: Source-backed evidence plus derived software guidance
tags: requirements, competency-questions, scope, ontology-engineering, shared-semantics
---

## Define Ontology Scope With Competency Questions Before Adding Shared Terms

**Source-backed evidence:** Grüninger and Fox characterize an ontology through competency questions and identify axioms needed to represent and answer them; Noy and McGuinness describe competency questions as a way to determine ontology scope and later test whether the ontology contains enough information. Ontology requirements literature similarly requires intended uses, end users, requirements, and motivating scenarios; NeOn and METHONTOLOGY both treat ontology development as a lifecycle with requirements, reuse, specification, conceptualization, implementation, and evaluation activities.

**Derived engineering rule:** Do not add a shared ontology term to a codebase, schema, graph, or generated type unless it is traceable to a competency question, ontology requirement, or accepted integration scenario. The rule is derived from the cited methodology work: if the ontology's expected questions are not stated, downstream software cannot distinguish shared meaning from a shared label.

**Incorrect — synthetic implementation sketch only:**

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

**Correct — synthetic implementation sketch only:**

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

References:
- [Grüninger & Fox — The Role of Competency Questions in Enterprise Engineering](https://www.researchgate.net/publication/2653385_The_Role_of_Competency_Questions_in_Enterprise_Engineering)
- [Noy & McGuinness — Ontology Development 101](https://protege.stanford.edu/publications/ontology_development/ontology101-noy-mcguinness.html)
- [Suárez-Figueroa et al. — How to Write and Use the Ontology Requirements Specification Document](https://www.researchgate.net/publication/220830640_How_to_Write_and_Use_the_Ontology_Requirements_Specification_Document)
- [Suárez-Figueroa et al. — The NeOn Methodology Framework](https://journals.sagepub.com/doi/abs/10.3233/AO-150145)
- [Fernández-López et al. — METHONTOLOGY](https://aaai.org/papers/0005-ss97-06-005-methontology-from-ontological-art-towards-ontological-engineering/)
- [Uschold & Grüninger — Ontologies: Principles, Methods and Applications](https://www.researchgate.net/publication/302937543_Ontologies_Principles_methods_and_applications)
- [Barbosa Fernandes et al. — Using Goal Modeling to Capture Competency Questions](https://periodicos.ufmg.br/index.php/jidm/article/view/124)
- [Gruber — Toward Principles for the Design of Ontologies Used for Knowledge Sharing](https://www.sciencedirect.com/science/article/pii/S1071581985710816)
