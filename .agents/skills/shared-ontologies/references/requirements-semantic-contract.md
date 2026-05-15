---
title: Treat Shared Ontology Terms as Semantic Contracts, Not Naming Conventions
impact: CRITICAL
impactDescription: Shared labels without formal definitions preserve ambiguity across services and schemas.
evidenceStrength: VERY HIGH
corroborationCount: 7
claimType: Source-backed evidence plus derived software guidance
tags: requirements, semantic-contract, definitions, interoperability, owl, software-engineering
---

## Treat Shared Ontology Terms as Semantic Contracts, Not Naming Conventions

**Source-backed evidence:** Gruber defines an ontology as an explicit specification of a representational vocabulary, including definitions of classes, relations, functions, and other objects. OWL 2 specifies ontologies as formally defined vocabularies containing classes, properties, individuals, and data values; software-engineering ontology literature argues that ontologies help reduce terminology mismatch among people, organizations, tools, and software artifacts.

**Derived engineering rule:** A shared software ontology term should carry at least a stable identifier, definition, relation semantics, provenance, and intended use. A TypeScript type alias, enum value, database column, or JSON field name alone is not a semantic contract because the cited ontology literature treats formalized meaning — not spelling — as the object of sharing.

**Incorrect — synthetic implementation sketch only:**

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

**Correct — synthetic implementation sketch only:**

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

References:
- [Gruber — A Translation Approach to Portable Ontology Specifications](https://tomgruber.org/writing/ontolingua-kaj-1993/)
- [Gruber — Toward Principles for the Design of Ontologies Used for Knowledge Sharing](https://www.sciencedirect.com/science/article/pii/S1071581985710816)
- [OWL 2 Web Ontology Language Document Overview](https://www.w3.org/TR/owl2-overview/)
- [Uschold & Grüninger — Ontologies: Principles, Methods and Applications](https://www.researchgate.net/publication/302937543_Ontologies_Principles_methods_and_applications)
- [Calero, Ruiz & Piattini — Ontologies for Software Engineering and Software Technology](https://link.springer.com/book/10.1007/3-540-34518-3)
- [Happel & Seedorf — Applications of Ontologies in Software Engineering](https://www.researchgate.net/publication/228386661_Applications_of_ontologies_in_software_engineering)
- [Akerman & Tyree — Using Ontology to Support Development of Software Architectures](https://www.researchgate.net/publication/224101625_Using_ontology_to_support_development_of_software_architectures)
