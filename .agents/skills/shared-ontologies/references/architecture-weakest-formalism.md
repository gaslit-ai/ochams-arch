---
title: Choose Ontology Expressivity From the Required Inference and Validation Workload
impact: HIGH
impactDescription: Excessive expressivity raises implementation and reasoning cost; insufficient expressivity fails to encode required semantics.
evidenceStrength: VERY HIGH
corroborationCount: 8
claimType: Source-backed evidence plus derived software guidance
tags: architecture, expressivity, owl-profiles, reasoning, shacl, obda, dl-lite
---

## Choose Ontology Expressivity From the Required Inference and Validation Workload

**Source-backed evidence:** OWL 2 defines a language with formal semantics for classes, properties, individuals, and data values; OWL 2 Profiles explicitly trade expressive power for reasoning efficiency and describe EL, QL, and RL profiles for different reasoning workloads. SHACL is a W3C language for validating RDF graphs against shape constraints, while DL-Lite and OBDA literature target tractable query answering and access to data through conceptual schemas and mappings.

**Derived engineering rule:** Select the weakest formalism that satisfies the software task: SKOS-like vocabularies for lightweight classification, OWL profiles for required open-world inference, SHACL for closed-world data validation, R2RML/OBDA mappings for relational data access, and full OWL DL only when the required reasoning justifies it. This is derived from the cited profile, validation, and OBDA literature, not from a claim that one formalism dominates all others.

**Incorrect — synthetic implementation sketch only:**

```ts
// Synthetic anti-pattern: maximum formalism selected without workload analysis.
export const ontologyRuntime = {
  language: "OWL-DL",
  reasoner: "global",
  reason: "formal ontologies should always use the strongest logic"
};
```

**Correct — synthetic implementation sketch only:**

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

References:
- [OWL 2 Web Ontology Language Document Overview](https://www.w3.org/TR/owl2-overview/)
- [OWL 2 Web Ontology Language Profiles](https://www.w3.org/TR/owl2-profiles/)
- [Horrocks, Patel-Schneider & van Harmelen — From SHIQ and RDF to OWL](https://papers.ssrn.com/sol3/papers.cfm?abstract_id=3199003)
- [Horrocks & Patel-Schneider — Reducing OWL Entailment to Description Logic Satisfiability](https://research.manchester.ac.uk/en/publications/reducing-owl-entailment-to-description-logic-satisfiability)
- [SHACL W3C Recommendation](https://www.w3.org/TR/shacl/)
- [Calvanese et al. — DL-Lite](https://www.diag.uniroma1.it/rosati/publications/CDLLPRR08.htm)
- [Xiao et al. — Ontology-Based Data Access: A Survey](https://www.ijcai.org/proceedings/2018/777)
