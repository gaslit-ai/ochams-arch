---
title: Make Schema-to-Ontology and Ontology-to-Ontology Mappings First-Class Artifacts
impact: HIGH
impactDescription: Shared ontologies do not eliminate semantic heterogeneity; they expose where mappings must be specified and tested.
evidenceStrength: VERY HIGH
corroborationCount: 9
claimType: Source-backed evidence plus derived software guidance
tags: evolution, mapping, alignment, schema-matching, obda, r2rml, data-integration
---

## Make Schema-to-Ontology and Ontology-to-Ontology Mappings First-Class Artifacts

**Source-backed evidence:** Schema-matching research identifies matching as a basic problem in data integration, semantic query processing, and related systems; ontology-matching surveys treat alignment as a distinct research problem with continuing challenges. Ontology-based data access literature uses mappings between local data sources and global conceptual schemas, while R2RML specifies customized mappings from relational databases to RDF datasets.

**Derived engineering rule:** Do not bury ontology mappings inside adapter code, SQL string manipulation, field-normalization functions, or undocumented ETL conventions. Store mappings as versioned, testable, reviewable artifacts with source schema, target ontology term, transformation rule, cardinality assumption, provenance, and affected competency questions.

**Incorrect — synthetic implementation sketch only:**

```ts
// Synthetic anti-pattern: semantic mapping hidden in procedural normalization.
export function normalizeOwner(row: Record<string, unknown>) {
  return {
    ownerId: row.client_id ?? row.party_ref ?? row.account_holder_id
  };
}
```

**Correct — synthetic implementation sketch only:**

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

References:
- [Rahm & Bernstein — A Survey of Approaches to Automatic Schema Matching](https://vldb.org/vldb_journal/index.php/component/article_manager/article/839)
- [Shvaiko & Euzenat — Ontology Matching: State of the Art and Future Challenges](https://www.researchgate.net/publication/260324688_Ontology_Matching_State_of_the_Art_and_Future_Challenges)
- [Aumueller et al. — Schema and Ontology Matching with COMA++](https://www.researchgate.net/publication/221212846_Schema_and_ontology_matching_with_COMA)
- [Wache et al. — Ontology-Based Integration of Information](https://www.researchgate.net/publication/200122923_Ontology-based_integration_of_information_---_a_survey_of_existing_approaches)
- [Xiao et al. — Ontology-Based Data Access: A Survey](https://www.ijcai.org/proceedings/2018/777)
- [R2RML: RDB to RDF Mapping Language](https://www.w3.org/TR/r2rml/)
- [Calvanese et al. — Data Integration through DL-LiteA Ontologies](https://www.diag.uniroma1.it/rosati/publications/CDLLPRR08.htm)
- [Calvanese et al. — Mastro: Ontology-Based Data Access at Work](https://link.springer.com/article/10.1007/s10844-011-0184-1)
- [Kharlamov et al. — Ontology Based Data Access in Statoil](https://www.sciencedirect.com/science/article/pii/S1570826817300276)
