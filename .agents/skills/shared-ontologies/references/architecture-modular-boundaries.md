---
title: Modularize Shared Ontologies Around Stable Software and Data Boundaries
impact: HIGH
impactDescription: Monolithic ontologies increase reasoning, import, review, and downstream change cost.
evidenceStrength: HIGH
corroborationCount: 6
claimType: Source-backed evidence plus derived software guidance
tags: architecture, modularity, imports, bounded-contexts, ontology-modules, maintenance
---

## Modularize Shared Ontologies Around Stable Software and Data Boundaries

**Source-backed evidence:** Modular ontology research reports that ontologies become larger and more complex to manage as their number and size grow, and that modularization supports extracting or managing parts relevant to particular scenarios. Rector et al. classify modular OWL development use cases including organization, stable software-interface bindings, localization, extension, and encapsulation; Cuenca Grau et al. formalize modular reuse using concepts such as safety, conservative extension, and module extraction.

**Derived engineering rule:** A software system should import the smallest ontology module that satisfies its competency questions and integration contract. Treat an ontology module like a semantic API: version it, publish its imports, test its closure properties, and avoid making every service depend on a whole-enterprise ontology.

**Incorrect — synthetic implementation sketch only:**

```ts
// Synthetic anti-pattern: every service imports the entire enterprise ontology.
import { EnterpriseOntology } from "@example/ontology/all";

export const ownerClass = EnterpriseOntology.Customer.Account.Owner;
```

**Correct — synthetic implementation sketch only:**

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

References:
- [Pathak, Johnson & Chute — Survey of Modular Ontology Techniques and Their Applications in the Biomedical Domain](https://journals.sagepub.com/doi/pdf/10.3233/ICA-2009-0315)
- [Rector et al. — Engineering Use Cases for Modular Development of Ontologies in OWL](https://www.researchgate.net/publication/262175610_Engineering_use_cases_for_modular_development_of_ontologies_in_OWL)
- [Cuenca Grau et al. — Modular Reuse of Ontologies: Theory and Practice](https://www.researchgate.net/publication/220543139_Modular_Reuse_of_Ontologies_Theory_and_Practice)
- [Stuckenschmidt, Parent & Spaccapietra — Modular Ontologies](https://link.springer.com/book/10.1007/978-3-642-01907-4)
- [Courtot et al. — MIREOT](https://journals.sagepub.com/doi/10.3233/AO-2011-0087)
- [Maedche, Motik & Stojanovic — Managing Multiple and Distributed Ontologies on the Semantic Web](https://research.manchester.ac.uk/en/publications/managing-multiple-and-distributed-ontologies-on-the-semantic-web)
