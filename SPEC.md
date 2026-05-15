# Ochams Architecture Compiler Specification

## Overview

Ochams is a compiler for architecture graphs.

Its source files are the architecture store. They are not documentation, configuration, diagrams, or generated artifacts. They are the durable record of a system's architectural facts: the things that exist, the kinds of those things, the relations allowed between them, and the edges that connect them.

Ochams also treats the architecture source tree as meaningful. Every project uses the same canonical layout. The folder structure does not create hidden business facts, but it does constrain which kinds of architectural facts may be written in each region. This gives the language a stable cognitive shape:

```text
source statements create graph facts
folder position constrains which facts are legal
the compiler validates both
```

The language is intentionally independent of implementation languages. It must not assume packages, classes, functions, services, controllers, repositories, tables, routes, SDKs, or any other implementation-shaped concept. Those may appear in a user's architecture only if the user explicitly models them as graph nodes and relations. Ochams itself only knows sources, layout regions, symbols, kinds, relations, nodes, edges, spans, evidence, views, and checks.

The first complete product is the Compiler MVP:

```text
canonical architecture tree
  -> parse .arch files
  -> validate source location
  -> resolve symbols
  -> build typed graph
  -> check graph
  -> query graph
```

The later Evidence MVP compares the declared graph with realization claims and observed evidence. It is deliberately separated from the first compiler milestone so the architecture language can become precise before implementation scanning exists.

No generated implementation code is required. The strongest form of the tool is not code generation; it is making architectural relationships explicit, typed, navigable, consistently placed, and difficult to drift from unnoticed.

## Table Of Contents

1. [Product Thesis](#product-thesis)
2. [Design Principles](#design-principles)
3. [Canonical Project Layout](#canonical-project-layout)
4. [Layout Semantics](#layout-semantics)
5. [Kind And Relation Classes](#kind-and-relation-classes)
6. [Canonical Source Model](#canonical-source-model)
7. [Architecture Language](#architecture-language)
8. [Minimal Grammar](#minimal-grammar)
9. [Imports And Symbol Resolution](#imports-and-symbol-resolution)
10. [Graph Semantics](#graph-semantics)
11. [Graph JSON Projection](#graph-json-projection)
12. [Compiler Pipeline](#compiler-pipeline)
13. [Compiler MVP Commands](#compiler-mvp-commands)
14. [Evidence MVP](#evidence-mvp)
15. [Diagnostics](#diagnostics)
16. [MVP Fixtures](#mvp-fixtures)
17. [Determinism](#determinism)
18. [Internal Architecture](#internal-architecture)
19. [Crate Strategy](#crate-strategy)
20. [Deferred Capabilities](#deferred-capabilities)
21. [Non-Goals](#non-goals)
22. [Acceptance Criteria](#acceptance-criteria)

## Product Thesis

Architecture should be edited as a typed graph, stored as source text, arranged in a universal source topology, and compiled like any other serious semantic artifact.

Ochams is based on five claims:

1. The architecture source tree is the graph store.
2. The canonical layout is part of the compiler contract.
3. The resolved graph is a rebuildable compiler product.
4. Implementation links are evidence, not authority.
5. Refactoring becomes tractable when side effects are presented as graph edges.

The key user workflow is:

```text
change architecture source
  -> compiler shows graph consequences
query architecture graph
  -> user sees declared impact and dependencies
later compare implementation-adjacent evidence
  -> user sees drift and missing links
```

The tool should be simple enough to understand in one sitting and strict enough to trust in large systems.

## Design Principles

### The Language Is The Store

There is no separate graph database as the source of truth. The repository's `.arch` files are authoritative. Indexes, JSON, diagrams, caches, and editor data are disposable.

### The Layout Is A Compiler Primitive

Every project uses the same architecture source layout. Path position has semantic meaning because it constrains what may be declared there.

The path does not silently create business facts. The source must still declare the fact.

### Symbols Are Not Strings

Every named architectural object is a symbol. A reference either resolves to a declared symbol or produces a diagnostic. Editor completion should be a view of the symbol table, not free-text guessing.

### Relations Are Typed

A relation declares the kind of source node it accepts and the kind of target node it accepts. An edge is valid only when its endpoints satisfy the relation.

### Imports Are Lexical Only

An import makes symbols easier to reference. It does not create an architecture edge. Graph facts are created only by declarations and edges.

### Stable Identity Is Semantic

Fully qualified symbols are stable identities. Dense compiler IDs are implementation details of one resolved graph revision.

### Authority And Evidence Are Different Regions

Authoritative architecture lives in `vocabulary/`, `domain/`, `capabilities/`, and `boundaries/`. Realization and evidence regions may reference authority, but they must not silently create it.

### Start With The Weakest Useful Formalism

The MVP needs closed-world symbol resolution, layout validation, and relation typechecking. It does not need ontology inference, graph database persistence, theorem proving, generated implementation code, or a universal static analyzer.

## Canonical Project Layout

Every Ochams project uses the same source topology:

```text
architecture/
├── workspace.arch
├── vocabulary/
│   ├── kinds/
│   │   ├── primitive.arch
│   │   ├── domain.arch
│   │   ├── capability.arch
│   │   ├── boundary.arch
│   │   ├── realization.arch
│   │   └── evidence.arch
│   ├── relations/
│   │   ├── structural.arch
│   │   ├── behavioral.arch
│   │   ├── boundary.arch
│   │   ├── realization.arch
│   │   └── evidential.arch
│   └── rules/
│       └── reserved for constraints
├── domain/
│   ├── actors/
│   ├── concepts/
│   ├── resources/
│   ├── states/
│   └── events/
├── capabilities/
│   ├── commands/
│   ├── queries/
│   ├── policies/
│   ├── guarantees/
│   └── effects/
├── boundaries/
│   ├── inbound/
│   │   ├── principals/
│   │   ├── surfaces/
│   │   ├── contracts/
│   │   └── permissions/
│   └── outbound/
│       ├── collaborators/
│       ├── stores/
│       ├── channels/
│       └── obligations/
├── realization/
│   ├── source-sets/
│   ├── anchors/
│   └── ownership/
├── evidence/
│   ├── anchors/
│   ├── static/
│   ├── runtime/
│   └── imported/
└── views/
    ├── contexts/
    ├── slices/
    ├── impact/
    └── reviews/
```

The compiler enforces the top-level spine:

```text
workspace.arch
vocabulary/
domain/
capabilities/
boundaries/
realization/
evidence/
views/
```

The second-level folders shown above are canonical facets. They guide authoring and may be used by tools and documentation, but they are not required compiler regions in the first milestone. The compiler must reject unknown top-level regions.

`vocabulary/` is stricter than the authoring regions. During the Compiler MVP, `vocabulary/` may contain only `kinds/`, `relations/`, and `rules/`. Any other child under `vocabulary/` is a malformed layout position.

Second-level flexibility applies only under:

```text
domain/
capabilities/
boundaries/
realization/
evidence/
views/
```

During the Compiler MVP, `.arch` files under `vocabulary/rules/`, `realization/`, `evidence/`, and `views/` are rejected rather than parsed as comments-only files. These directories may exist, but they must not contain `.arch` source until their milestones define syntax.

The Compiler MVP does not need every file shown above to exist. It does need to understand every top-level region and reject facts written in the wrong top-level region.

### Region Questions

The canonical layout gives users a fixed sequence of architectural questions:

```text
vocabulary/     What words are valid?
domain/         What durable things exist?
capabilities/   What meaningful actions, guarantees, and effects exist?
boundaries/      What crosses the system edge?
realization/     What source locations realize architecture?
evidence/        What has been observed?
views/           How should the graph be inspected?
```

## Layout Semantics

Layout semantics are constraints over source location.

The compiler maps each `.arch` file to a layout region and validates that the file contains only statements allowed in that region.

### Region Permissions

```text
architecture/workspace.arch        permits workspace metadata and root space
vocabulary/kinds/**                permits kind declarations
vocabulary/relations/**            permits relation declarations
vocabulary/rules/**                reserved for constraints; Compiler MVP rejects `.arch` files
domain/**                          permits domain-class nodes and structural-class edges
capabilities/**                    permits capability-class nodes and behavioral-class edges
boundaries/**                      permits boundary-class nodes and boundary-class edges
realization/**                     reserved for Evidence MVP; Compiler MVP rejects `.arch` files
evidence/**                        reserved for Evidence MVP; Compiler MVP rejects `.arch` files
views/**                           reserved for View MVP; Compiler MVP rejects `.arch` files
```

The MVP should implement these as built-in rules, not user configuration.

All active non-workspace regions permit required `space`, required `module`, `use`, blank lines, and comments. Region-specific permissions govern only declarations that create graph facts: `kind`, `relation`, `node`, and `edge`.

### Region Reference Direction

The canonical layout also defines default reference direction:

```text
vocabulary/       may reference vocabulary/
domain/           may reference vocabulary/ and domain/
capabilities/     may reference vocabulary/, domain/, and capabilities/
boundaries/        may reference vocabulary/, domain/, capabilities/, and boundaries/
realization/       reserved for Evidence MVP
evidence/          reserved for Evidence MVP
views/             reserved for View MVP
```

This rule prevents lower-authority regions from smuggling new source-of-truth concepts into the graph.

Checked references:

```text
use path
node kind reference
relation source kind reference
relation target kind reference
edge source reference
edge relation reference
edge target reference
realization claim references, in the Evidence MVP
observed evidence references, in the Evidence MVP
view seed reference, in the View MVP
```

Every checked reference is mapped to the layout region of the referenced symbol's declaration. The source file's region and target symbol's region are then checked against the matrix above.

### Path And Module Alignment

Path and module must align.

```text
architecture/domain/resources/pet.arch
```

contains:

```text
space VetClinic
module Domain.Resources

node Pet : Entity
```

The compiler verifies:

```text
domain/resources/ -> module Domain.Resources
```

Moving a file is therefore an architectural refactor. The path changes the legal authoring context, and the module declaration must move with it.

### Path-To-Module Algorithm

`architecture/workspace.arch` declares the project `space` and must not declare a `module`.

Every other `.arch` file must declare a module derived from its path. File basenames do not contribute to module identity. Directory segments below `architecture/` become module segments converted to PascalCase.

Compiler MVP source path segments must match:

```text
[a-z][a-z0-9]*(?:-[a-z][a-z0-9]*)*
```

PascalCase conversion splits on `-`, capitalizes each component, and concatenates the components.

```text
source-sets -> SourceSets
inbound -> Inbound
resources -> Resources
```

Path segments that cannot be converted by this rule are malformed layout positions.

Examples:

```text
architecture/domain/resources/pet.arch
  -> module Domain.Resources

architecture/capabilities/commands/scheduling.arch
  -> module Capabilities.Commands

architecture/boundaries/inbound/surfaces/http.arch
  -> module Boundaries.Inbound.Surfaces
```

Vocabulary class files are a special case. A class file directly under `vocabulary/kinds/` or `vocabulary/relations/` does not add the basename to the module.

```text
architecture/vocabulary/kinds/domain.arch
  -> module Vocabulary.Kinds

architecture/vocabulary/relations/structural.arch
  -> module Vocabulary.Relations
```

If a vocabulary class uses a directory, that class directory is part of the module.

```text
architecture/vocabulary/kinds/domain/entity.arch
  -> module Vocabulary.Kinds.Domain

architecture/vocabulary/relations/structural/ownership.arch
  -> module Vocabulary.Relations.Structural
```

The compiler must reject any file whose declared module differs from this derived module.

### Explicit Facts Only

The path constrains facts; it does not create them.

This path:

```text
architecture/domain/resources/pet.arch
```

does not mean `Pet` exists.

This statement creates the graph fact:

```text
node Pet : Entity
```

## Kind And Relation Classes

Layout permissions depend on classes. Classes are compiler-derived from declaration location; they are not extra syntax in the MVP.

### Kind Classes

A kind's class is derived from the file or first directory segment under `architecture/vocabulary/kinds/`.

```text
architecture/vocabulary/kinds/domain.arch
architecture/vocabulary/kinds/domain/entity.arch
```

Both paths declare kinds in the `domain` class.

Built-in kind classes:

```text
primitive
domain
capability
boundary
realization
evidence
```

Kind classes are closed in the Compiler MVP. A kind declared under any other class file or class directory is rejected as a malformed layout position.

Example:

```text
# architecture/vocabulary/kinds/domain.arch
kind Entity
kind State
kind Event
```

`Entity`, `State`, and `Event` are domain-class kinds.

If a kind is declared under a class file such as `kinds/domain.arch`, the class is the basename `domain`. If it is declared under a class directory such as `kinds/domain/entity.arch`, the class is the first directory segment `domain`.

### Relation Classes

A relation's class is derived from the file or first directory segment under `architecture/vocabulary/relations/`.

```text
architecture/vocabulary/relations/structural.arch
architecture/vocabulary/relations/structural/ownership.arch
```

Both paths declare relations in the `structural` class.

Built-in relation classes:

```text
structural
behavioral
boundary
realization
evidential
```

Relation classes are closed in the Compiler MVP. A relation declared under any other class file or class directory is rejected as a malformed layout position.

Example:

```text
# architecture/vocabulary/relations/structural.arch
relation has Entity -> Entity
relation owns Entity -> Entity
```

`has` and `owns` are structural-class relations.

If a relation is declared under a class file such as `relations/structural.arch`, the class is the basename `structural`. If it is declared under a class directory such as `relations/structural/ownership.arch`, the class is the first directory segment `structural`.

### Class-Based Declaration Rules

Node declarations are legal when the node's kind class is permitted by the file's layout region.

```text
domain/**          permits domain-class node declarations
capabilities/**    permits capability-class node declarations
boundaries/**      permits boundary-class node declarations
```

Edge declarations are legal when the edge's relation class is permitted by the file's layout region.

```text
domain/**          permits structural-class edge declarations
capabilities/**    permits behavioral-class edge declarations
boundaries/**      permits boundary-class edge declarations
```

This means the compiler can enforce layout without knowing business terminology. The meaning comes from declarations; the allowed declaration placement comes from classes derived by canonical path.

Realization-class and evidence-class kinds and relations may be declared in `vocabulary/**` during the Compiler MVP because vocabulary is authoritative. Their use in `realization/**` and `evidence/**` is reserved for later milestones.

## Canonical Source Model

A project is a canonical `architecture/` tree containing `.arch` source files. Each file contributes declarations and edges to one workspace graph.

The Compiler MVP source model has seven concepts:

```text
space       root naming scope
module      path-aligned naming scope
use         lexical visibility
kind        node category
relation    typed edge vocabulary
node        graph vertex
edge        graph relationship
```

The Compiler MVP requires all seven concepts. `view` is reserved for a later milestone; `.arch` files under `views/**` are rejected until view semantics are defined.

### Modules And Files

Multiple files may declare the same `module`. This lets a region be split by topic without forcing file names into symbol identity.

```text
architecture/domain/resources/pet.arch
architecture/domain/resources/appointment.arch
```

Both may declare:

```text
module Domain.Resources
```

File basenames do not contribute to symbol identity. The stable identity of a symbol is:

```text
<space>.<module>.<local symbol>
```

For example:

```text
space VetClinic
module Domain.Resources
node Pet : Entity
```

declares:

```text
VetClinic.Domain.Resources.Pet
```

The compiler may later add optional style diagnostics when a file's basename is surprising, but the MVP must not make file names part of semantic identity.

### Workspace And Space

`architecture/workspace.arch` is mandatory. It must contain exactly one `space` declaration and no `module`.

Every other `.arch` file must begin with the same `space` declaration and must then declare its path-derived `module`.

```text
space VetClinic
module Domain.Resources
```

Missing `workspace.arch`, missing `space`, incompatible `space`, missing `module`, and module/path mismatch are distinct diagnostics.

### Local Scope

Local declarations are file-local. A symbol declared in another file with the same module is not automatically visible by short name.

Cross-file references must use either:

```text
use VetClinic.Domain.Resources.Pet
```

or a fully qualified symbol:

```text
VetClinic.Domain.Resources.Pet
```

### Example Source Tree

```text
architecture/
├── workspace.arch
├── vocabulary/
│   ├── kinds/domain.arch
│   └── relations/structural.arch
└── domain/
    ├── resources/pet.arch
    └── resources/appointment.arch
```

`architecture/workspace.arch`:

```text
space VetClinic
```

`architecture/vocabulary/kinds/domain.arch`:

```text
space VetClinic
module Vocabulary.Kinds

kind Entity
```

`architecture/vocabulary/relations/structural.arch`:

```text
space VetClinic
module Vocabulary.Relations

use VetClinic.Vocabulary.Kinds.Entity

relation has Entity -> Entity
```

`architecture/domain/resources/pet.arch`:

```text
space VetClinic
module Domain.Resources

use VetClinic.Vocabulary.Kinds.Entity

node Pet : Entity
```

`architecture/domain/resources/appointment.arch`:

```text
space VetClinic
module Domain.Resources

use VetClinic.Vocabulary.Kinds.Entity
use VetClinic.Vocabulary.Relations.has
use VetClinic.Domain.Resources.Pet

node Appointment : Entity

edge Pet has Appointment
```

This example describes architecture only. It does not imply any implementation language, runtime framework, protocol, persistence engine, or repository implementation shape.

## Architecture Language

The language should remain sparse. New syntax must earn its place by making graph facts clearer, not by making examples prettier.

### `space`

Declares the root scope for a source set.

```text
space VetClinic
```

The compiler must reject source sets that define incompatible root spaces.

### `module`

Declares the file's path-aligned scope.

```text
module Domain.Resources
```

The module is checked against the file's canonical layout path.

`architecture/workspace.arch` must not declare a module. Every other `.arch` file must declare one.

### `use`

Makes a symbol visible by short name within the current file.

```text
use VetClinic.Domain.Resources.Pet
```

The fully qualified symbol must always remain valid.

### `kind`

Declares a category of node.

```text
kind Entity
kind Operation
kind Boundary
```

Kinds are themselves symbols and belong in `vocabulary/kinds/**`.

### `relation`

Declares a typed edge vocabulary term.

```text
relation reads Operation -> Entity
```

Relations are themselves symbols and belong in `vocabulary/relations/**`.

This permits:

```text
edge ScheduleAppointment reads Pet
```

It rejects:

```text
edge Pet reads ScheduleAppointment
```

### `node`

Declares a graph vertex with a kind.

```text
node Appointment : Entity
```

The node's region must permit nodes of that kind.

### `edge`

Declares a graph relationship.

```text
edge Pet has Appointment
```

Edges are facts. The compiler must preserve their source spans so diagnostics and graph views can navigate back to source.

## Minimal Grammar

The first milestone should accept a deliberately small line-oriented grammar. This grammar is large enough to build a typed graph and small enough to make diagnostics precise.

```text
file              = blank_or_comment* header statement*
header            = space_decl blank_or_comment* module_decl?

space_decl        = "space" ident newline
module_decl       = "module" relative_path newline

statement         = use_decl
                  | kind_decl
                  | relation_decl
                  | node_decl
                  | edge_decl
                  | blank_or_comment

use_decl          = "use" absolute_path newline
kind_decl         = "kind" ident newline
relation_decl     = "relation" ident kind_ref "->" kind_ref newline
node_decl         = "node" ident ":" kind_ref newline
edge_decl         = "edge" symbol_ref relation_ref symbol_ref newline

kind_ref          = symbol_ref
relation_ref      = symbol_ref
symbol_ref        = absolute_path | ident
absolute_path     = ident "." ident ("." ident)*
relative_path     = ident ("." ident)*
ident             = ASCII_ALPHA (ASCII_ALNUM | "_")*
newline           = "\n" | EOF
whitespace        = ASCII whitespace
blank_or_comment  = whitespace* newline | whitespace* "#" text newline
```

A no-dot `symbol_ref` is a bare identifier and a dotted `symbol_ref` is an absolute path. An absolute path used as a symbol reference must begin with the current `space`.

The MVP excludes:

```text
wildcard imports
blocks
attributes
quoted identifiers
multi-line statements
inline metadata
user-defined layout profiles
```

The exclusion is intentional. Every additional syntax form must prove that it carries graph meaning the core grammar cannot express clearly.

### Parser Implementation Prior

The Compiler MVP uses `winnow` for statement-line parsing. This is an implementation dependency, not language authority. The authoritative grammar is still the line-oriented grammar above.

The parser boundary is intentionally narrow:

```text
source text
  -> physical lines with byte spans
  -> blank/comment filtering
  -> one statement parser per remaining line
```

`winnow` should make statement shapes, token consumption, punctuation separators, and end-of-line checks explicit without introducing blocks, recovery grammar, implicit metadata, Unicode whitespace normalization, or a public token stream. Source spans remain owned by the compiler, and parse diagnostics remain `OCH001` with the current minimal diagnostic model.

## Imports And Symbol Resolution

Symbol resolution must be strict, boring, and predictable.

Resolution order:

```text
fully qualified path
file-local declarations
explicit use declarations
diagnostic on ambiguity
diagnostic on missing symbol
```

Wildcard imports are excluded from the MVP. They make early examples shorter and large systems less clear.

Fully qualified paths must begin with the current `space`.

```text
VetClinic.Domain.Resources.Pet
```

Dotted paths that do not begin with the current `space` are rejected in the MVP.

```text
Domain.Resources.Pet
```

Bare identifiers resolve only against file-local declarations and explicit `use` declarations. If two explicit uses expose the same bare identifier, the bare identifier is ambiguous and the source must use a fully qualified path.

Imports do not create edges.

```text
use VetClinic.Domain.Resources.Pet
```

This creates no graph relationship. A relationship exists only when a fact uses the symbol:

```text
edge ScheduleAppointment reads Pet
```

## Graph Semantics

The resolved graph contains:

```text
Workspace
LayoutRegion
Symbol
KindClass
RelationClass
Kind
Relation
Node
Edge
SourceSpan
Diagnostic
```

Stable public identity:

```text
VetClinic.Domain.Resources.Appointment
VetClinic.Capabilities.Commands.ScheduleAppointment
```

Disposable compiler identity:

```text
SymbolId
KindId
RelationId
NodeId
EdgeId
```

Dense IDs may change on every compilation. Public outputs must prefer stable symbols.

Ochams uses one global symbol namespace per `space`. A kind, relation, and node may not share the same fully qualified symbol. `OCH009 duplicate symbol` fires whenever two declarations resolve to the same stable symbol, regardless of declaration category.

For the Compiler MVP, relation endpoint checking uses exact kind equality. If a relation declares:

```text
relation has Entity -> Entity
```

then an edge using `has` must have a source node whose kind is exactly `Entity` and a target node whose kind is exactly `Entity`. Kind classes do not imply subtyping.

Subtype or class-wide relation matching is deferred until the language has explicit syntax for it.

### Resolved Graph Shape

```text
Graph
  projection() -> GraphProjection
  to_pretty_json() -> text

GraphProjection
  format
  space
  workspace_source: GraphWorkspaceSource
  sources: GraphSource[]
  kinds: GraphKind[]
  relations: GraphRelation[]
  nodes: GraphNode[]
  edges: GraphEdge[]
```

The exact checked in-memory representation is not public contract. The public contract is the deterministic graph projection, deterministic graph JSON, deterministic query text, and diagnostics.

### Compiler MVP Query Surface

The Compiler MVP query surface is intentionally narrow. `ochams query` must render:

```text
symbol identity
symbol category
node kind, when the symbol is a node
layout region
declaration span
coalesced incoming edge facts
coalesced outgoing edge facts
immediate dependents
```

This is sufficient to prove that the graph is usable, not merely serializable.

Broader graph operations are deferred until a caller needs them:

```text
neighbors(symbol, direction, relation?)
reachable(symbol, direction, relation?)
edges_between(source, target)
subgraph(seed, depth, relation?)
cycles(relation?)
symbols_in_region(region)
```

Those future operations should be implemented over adjacency and source indexes before introducing a general graph query language.

## Graph JSON Projection

`ochams graph <root> --format json` emits a deterministic projection of the resolved graph. This projection is rebuildable and must not be edited as source.

The top-level JSON shape is:

```json
{
  "format": "ochams.graph.v1",
  "space": "VetClinic",
  "workspaceSource": {
    "path": "architecture/workspace.arch"
  },
  "sources": [],
  "kinds": [],
  "relations": [],
  "nodes": [],
  "edges": []
}
```

### Sources

`sources` contains non-workspace `.arch` files compiled into the declared graph. `workspace.arch` is represented only by `workspaceSource`.

```json
{
  "path": "architecture/domain/resources/pet.arch",
  "region": "domain/resources",
  "module": "VetClinic.Domain.Resources"
}
```

`sources[].module` is fully qualified with the current `space`, even though source files declare relative modules.

### Kinds

```json
{
  "symbol": "VetClinic.Vocabulary.Kinds.Entity",
  "name": "Entity",
  "class": "domain",
  "declaredAt": {
    "path": "architecture/vocabulary/kinds/domain.arch",
    "start": 42,
    "end": 53
  }
}
```

### Relations

```json
{
  "symbol": "VetClinic.Vocabulary.Relations.has",
  "name": "has",
  "class": "structural",
  "sourceKind": "VetClinic.Vocabulary.Kinds.Entity",
  "targetKind": "VetClinic.Vocabulary.Kinds.Entity",
  "declaredAt": {
    "path": "architecture/vocabulary/relations/structural.arch",
    "start": 78,
    "end": 103
  }
}
```

### Nodes

```json
{
  "symbol": "VetClinic.Domain.Resources.Pet",
  "name": "Pet",
  "kind": "VetClinic.Vocabulary.Kinds.Entity",
  "kindClass": "domain",
  "declaredAt": {
    "path": "architecture/domain/resources/pet.arch",
    "start": 64,
    "end": 81
  }
}
```

### Edges

```json
{
  "key": "VetClinic.Domain.Resources.Pet|VetClinic.Vocabulary.Relations.has|VetClinic.Domain.Resources.Appointment",
  "source": "VetClinic.Domain.Resources.Pet",
  "relation": "VetClinic.Vocabulary.Relations.has",
  "relationClass": "structural",
  "target": "VetClinic.Domain.Resources.Appointment",
  "declaredAt": [
    {
      "path": "architecture/domain/resources/appointment.arch",
      "start": 136,
      "end": 160
    }
  ]
}
```

Public JSON must use stable symbols and root-relative paths. Dense compiler IDs must not appear in public JSON.

`start` and `end` are UTF-8 byte offsets into the source file. Line and column rendering belongs to diagnostics and editor adapters.

Spans are half-open ranges:

```text
[start, end)
```

For declarations and edges, `declaredAt` covers the complete logical statement line: from the first byte of the keyword through the final byte of the last token, excluding the line terminator. For `workspaceSource`, the path identifies the file and no byte span is required.

Duplicate identical edges are coalesced in public JSON. The edge identity is:

```text
source symbol + relation symbol + target symbol
```

All source declarations for that edge are preserved in `declaredAt`, sorted by path and byte span.

JSON output rules:

```text
UTF-8
LF newlines
two-space indentation
trailing newline
stable array ordering
no dense compiler IDs
no JSON emitted when parsing, layout, resolution, or semantic errors exist
```

## Compiler Pipeline

The compiler runs the same semantic pipeline for the CLI and tests. Future editor adapters should call the same core pipeline and translate its outputs without adding compiler rules.

```text
discover architecture root
  -> validate canonical layout
  -> classify source region
  -> read source text
  -> parse source lines
  -> validate workspace, space, and module headers
  -> collect declarations
  -> derive kind and relation classes
  -> validate declaration location
  -> resolve imports, scopes, references, categories, and region direction
  -> check node classes, edge classes, and relation endpoints
  -> construct graph
```

Each phase must produce diagnostics without requiring later phases to guess what happened earlier.

### Phase Responsibilities

`discover architecture root` expects `<root>` to be a project root with a direct child named `architecture/`. Passing the `architecture/` directory itself is an error in the Compiler MVP.

`validate canonical layout` rejects missing `architecture/workspace.arch`, unknown top-level regions, and malformed layout positions.

`read source text` loads active `.arch` files from discovered sources produced by layout classification. Non-`.arch` files under `architecture/` are ignored. Symlinks are not followed. Project-local ignore files may be respected for non-architecture paths, but files under `architecture/` are always considered unless they are not `.arch`.

`parse source lines` produces parsed source units with an AST and recoverable syntax diagnostics. The Compiler MVP parser is line-oriented: it records complete statement byte spans and treats intra-line whitespace-separated tokens as grammar fields rather than as a public token stream.

`classify source region` maps each file path to a layout region exactly once and carries that result into parsing and later compiler phases.

`validate workspace, space, and module headers` proves that the root workspace and every active non-workspace source share one declared space and a module matching its canonical path-derived module. Successful validation converts parsed source units into checked active source units. Downstream phases receive a root space and module-qualified units, not optional header fields.

`collect declarations` gathers symbol declarations for kinds, relations, and nodes, plus unresolved relation, node-kind, and edge reference records. Workspace, space, and module facts are owned by header validation, and edges become graph facts only after reference resolution succeeds.

`derive kind and relation classes` derives classes from canonical vocabulary paths.

`validate declaration location` checks that each statement is legal in its layout region.

`resolve imports, scopes, references, categories, and region direction` records lexical visibility from local declarations and `use` statements, converts names to symbols, checks expected symbol categories, and rejects references that violate the canonical authoring order between regions.

`check node classes, edge classes, and relation endpoints` validates node placement, relation class placement, and every edge against its relation's endpoint kinds.

`construct graph` creates the immutable graph revision. Public callers may project and query the graph, but they must not mutate compiler-derived facts.

After compilation, callers may render diagnostics, project graph JSON, or run query formatting. Those reporting operations must not change the compiled graph.

### Finite Policy Authority

Closed compiler policy is represented as typed data plus explicit evaluator functions inside the compiler core. This includes:

```text
canonical top-level regions
reserved versus active source regions
vocabulary child positions
built-in kind classes
built-in relation classes
region declaration permissions
region reference direction
```

These finite tables are semantic authority. Parser behavior, source discovery, layout classification, declaration collection, resolution, and semantic checks must consume the same evaluator functions rather than cloning the same matrix in separate control flow.

The Compiler MVP does not use procedural macros, generated Rust, build scripts, or generated projections to decide accepted source, source spans, relation compatibility, or public output. A local declarative macro may be used only for a mechanical catalog when the invocation visibly contains every public fact and the expansion is a bijective transposition into ordinary Rust items or match arms. If generation is introduced later, it must be an explicit maintainer command with a verification mode, and public contract tests must exercise the generated projection before generated implementation can become trusted.

### Reporting Responsibilities

The core crate owns deterministic rendering that is part of the compiler contract: diagnostic text, graph JSON, and query text. The CLI owns command parsing, exit codes, stream selection, and terminal policy. Editor and future LSP adapters should translate core diagnostics and graph facts into editor protocol data without adding compiler rules.

### Core Public API Boundary

The Compiler MVP primary entrypoints are deliberately projection-oriented:

```text
compile(root) -> Compilation
format_diagnostics(diagnostics) -> text
Graph::projection() -> GraphProjection
Graph::to_pretty_json() -> text
format_query(graph, symbol) -> Result<text, Diagnostic>
```

`Graph` is an immutable compiled revision from the caller's perspective. It does not expose raw symbol records, raw edge records, mutable fields, or duplicate edge declarations. Public callers observe graph facts through `GraphProjection` and its stable view records:

```text
GraphWorkspaceSource
GraphSource
GraphKind
GraphRelation
GraphNode
GraphEdge
```

Those view records are nameable public types. They are the supported in-process graph projection surface. Their serialized field names define the JSON projection shape.

The primary entrypoint list is not the full exported support surface. Diagnostics, spans, compilation results, graph projection records, and their constructors or renderers are also public where callers need to inspect or report compiler output. Rustdoc is the exhaustive source-level contract for those exported Rust items.

Every exported `ochams-core` item must have Rust API documentation. The crate enforces this with the Rust `missing_docs` lint because an undocumented public item is an undocumented compiler contract. Public documentation should state semantic intent, invariants, ownership, and deterministic rendering behavior. It should not describe future adapters, generated implementation code, or storage systems that are outside the current public API.

This documentation prior is intentionally narrow. Rustdoc is the source-level public API contract; generated HTML is a disposable projection. The language source tree remains the architecture store, and compiled graph JSON remains the public graph projection. The docs must encode deterministic behavior callers rely on, including diagnostic text ordering, query section ordering, graph projection format, edge coalescing, and serialized field names.

## Compiler MVP Commands

### `ochams check`

```text
ochams check <root>
```

Finds `<root>/architecture`, parses all `.arch` files, validates layout, resolves the graph, runs semantic checks, and reports diagnostics.

Command contract:

```text
success: exit 0, stdout empty
failure: nonzero exit, diagnostics on stderr
color: disabled when NO_COLOR is set or stdout/stderr is not a terminal
```

### `ochams graph`

```text
ochams graph <root> --format json
```

Emits deterministic graph JSON. This output is a projection, not a source file.

Command contract:

```text
success: exit 0, graph JSON on stdout, diagnostics absent from stdout
failure: nonzero exit, diagnostics on stderr, no JSON on stdout
format: UTF-8, LF, two-space pretty JSON, trailing newline
invalid format or malformed invocation: nonzero exit, OCH020 diagnostic on stderr
```

### `ochams query`

```text
ochams query <root> <symbol>
```

Shows graph context for a fully qualified symbol. The `<symbol>` argument must be fully qualified and must begin with the current `space`.

Command contract:

```text
success: exit 0, deterministic text output on stdout
failure: nonzero exit, diagnostics on stderr
```

`query` is part of the Compiler MVP because it proves the graph is usable, not merely serializable.

Compiler MVP query output is line-oriented text:

```text
symbol: <fully-qualified-symbol>
category: <kind|relation|node>
kind: <fully-qualified-kind-symbol|none>
layout-region: <layout-region>
declared-at: <root-relative-path>:<start>..<end>
incoming:
  <source> --<relation>--> <target>
outgoing:
  <source> --<relation>--> <target>
dependents:
  <fully-qualified-symbol>
```

`kind` is `none` for kind and relation symbols. `incoming`, `outgoing`, and `dependents` sections are always present. Empty sections contain no item lines. Section items are sorted by source symbol, relation symbol, target symbol, path, and span where those fields exist. Dependents are sorted by fully qualified symbol.

For the Compiler MVP, `incoming` and `outgoing` are edge-context sections for node symbols. Kind and relation queries keep those sections present but empty. `dependents` lists fully qualified symbols that would be directly affected by removing or renaming the queried symbol:

```text
kind query       nodes using the kind, and relations whose endpoints use the kind
relation query   node symbols participating in edges that use the relation
node query       adjacent node symbols connected by incoming or outgoing edges
```

Reachability, subgraph, and cycle query functions may become internal API requirements after the initial command is stable.

## Evidence MVP

The Evidence MVP starts only after the Compiler MVP is complete.

The declared architecture graph and evidence graph are separate compiler products:

```text
declared graph       from vocabulary/domain/capabilities/boundaries
realization graph    from realization/
evidence graph       from evidence/ and external scanners
```

`realization/` contains human-declared claims that source locations realize architecture symbols.

`evidence/` contains observed facts from anchors, static analysis, runtime traces, or imported analyzer output.

Neither `realization/` nor `evidence/` may declare domain, capability, boundary, kind, or relation authority.

Future commands:

```text
ochams scan <root> --code <path>
ochams diff <root> --code <path>
```

These commands are not part of the Compiler MVP.

### Source Anchors

Example anchor text:

```text
@realizes VetClinic.Capabilities.Commands.ScheduleAppointment
@edge VetClinic.Capabilities.Commands.ScheduleAppointment reads VetClinic.Domain.Resources.Pet
@edge VetClinic.Capabilities.Commands.ScheduleAppointment writes VetClinic.Domain.Resources.Appointment
```

The scanner must not care whether this appears in a comment, docstring, Markdown file, or another text format. Byte-level anchor extraction belongs to the Evidence MVP.

### Evidence Model

```text
SourceAnchor
  symbol
  relation
  target
  file
  span
  extractor
  confidence

ObservedEdge
  source
  relation
  target
  provenance
  confidence
  supporting_anchors
```

Evidence can support the declared graph, contradict it, or reveal unexplained implementation behavior. It cannot change the declared graph.

### Evidence Region

`evidence/**` may store imported or observed evidence as source-controlled `.arch` files when a project chooses to preserve observations.

Evidence files may reference authority. They may not declare new authoritative domain, capability, or boundary nodes.

Evidence syntax is not part of the Compiler MVP grammar. During the Compiler MVP, `.arch` files under `evidence/**` are rejected.

## Diagnostics

Diagnostics are part of the compiler contract.

The Compiler MVP diagnostic model is deliberately small. Each diagnostic includes:

```text
code
message
optional primary span
```

Severity, labels, notes, line/column rendering, and richer editor payloads are deferred until the language needs multi-span diagnostics. CLI rendering is diagnostic-code-first and deterministic.

The authoritative diagnostic identity catalog is the `DiagnosticCode` catalog in `ochams-core`. `DiagnosticCode::ALL` exposes the canonical order; `DiagnosticCode::as_str` exposes the stable `OCH###` text. A local declarative macro may maintain this catalog only while its invocation includes every public variant name, every stable code string, and the Rustdoc for each public variant. The macro must not decide diagnostic messages, source spans, phase ordering, or semantic acceptance.

The compiler core owns diagnostic data and deterministic plain-text diagnostic rendering. The CLI owns command routing, exit codes, stream selection, and terminal policy such as color. Some repository-level diagnostics, such as a missing `architecture/workspace.arch`, may not have a source span. Core rendering sorts diagnostics by optional path, optional start offset, optional end offset, diagnostic code, and message before rendering; unspanned diagnostics sort before spanned diagnostics.

Diagnostic precedence:

```text
parse and layout diagnostics precede resolution diagnostics
missing symbols precede category checks
category mismatch precedes relation endpoint kind mismatch
declaration-location diagnostics precede region-reference diagnostics
```

Expected symbol categories:

```text
node kind reference              must resolve to kind
relation source kind reference   must resolve to kind
relation target kind reference   must resolve to kind
edge source reference            must resolve to node
edge relation reference          must resolve to relation
edge target reference            must resolve to node
```

## MVP Fixtures

The first implementation is driven by repository-shaped fixtures, not isolated parser strings. The public command contract uses root-level golden fixtures under `tests/fixtures/`. Reusable semantic seed repos live separately under `tests/seeds/` so command-fixture review and semantic-test baselines do not become the same physical authority. Narrow semantic checks still construct small repository trees inline when a targeted delta is clearer than a checked-in seed directory.

Golden command fixture scenarios are authoritative by whatever `tests/fixtures/*/commands.txt` declares. The current checked-in cases include:

```text
valid-minimal/
  proves workspace, vocabulary, two domain nodes, one structural edge, and graph JSON

duplicate-edge/
  proves duplicate edge declarations coalesce in the public graph and query views

missing-space/
  proves public commands fail deterministically when an active source omits `space`

multiple-diagnostics/
  proves public commands preserve deterministic multi-diagnostic stderr ordering
```

Semantic seed and inline coverage scenarios:

```text

invalid-root-space/
  proves incompatible `space` declarations produce OCH004

missing-workspace/
  proves missing `architecture/workspace.arch` produces OCH002

missing-space/
  proves a non-workspace file without `space` produces OCH003

missing-module/
  proves a non-workspace file without `module` produces OCH005

invalid-module-path/
  proves `architecture/domain/resources/pet.arch` cannot declare `module Domain.Actors`

invalid-layout-region/
  proves unknown top-level architecture folders are rejected

invalid-kind-location/
  proves `kind` outside `vocabulary/kinds/**` is rejected

invalid-relation-location/
  proves `relation` outside `vocabulary/relations/**` is rejected

invalid-node-kind-class/
  proves a boundary-class node cannot be declared under `domain/**`

invalid-edge-relation-class/
  proves a behavioral-class relation cannot be used for an edge under `domain/**`

invalid-dotted-reference/
  proves `Domain.Resources.Pet` is rejected because it is neither bare nor fully qualified

missing-symbol/
  proves unresolved symbol references are rejected

ambiguous-symbol/
  proves two imported short names with the same local name require qualification

reserved-region-source/
  proves `.arch` source in a reserved Compiler MVP region is rejected

malformed-path-segment/
  proves invalid path segments are rejected

unknown-kind-class/
  proves `vocabulary/kinds/other.arch` is rejected

unknown-relation-class/
  proves `vocabulary/relations/other.arch` is rejected

symbol-category-mismatch/
  proves references resolving to the wrong symbol category are rejected before endpoint kind checks
```

Every fixture should assert expected diagnostics and, where applicable, graph JSON and query behavior. Fixture outputs must be deterministic.

Golden command fixture convention:

```text
tests/fixtures/<case>/
├── repo/
│   └── architecture/
├── commands.txt
├── expected.check.exit
├── expected.check.stdout
├── expected.check.stderr
├── expected.graph.exit
├── expected.graph.stdout.json
├── expected.graph.stderr
├── query.symbol
├── expected.query.exit
├── expected.query.stdout
└── expected.query.stderr
```

`commands.txt` is the human-authored fixture manifest. It lists one command name per line from `check`, `graph`, and `query`; blank lines and `#` comments are ignored. The fixture harness discovers every fixture directory and runs only the commands declared by `commands.txt`. Expected files are reviewed command-output oracles and must not decide coverage. Expected files or `query.symbol` for undeclared commands are stale fixture-authority errors.

For each declared command, `expected.<command>.exit` is required. Missing stdout or stderr files for an exercised command mean the stream is expected to be empty. Existing expected stream files must be readable UTF-8; unreadable or malformed files are fixture errors, not empty streams. `graph` stdout must use `expected.graph.stdout.json`; plain `expected.graph.stdout` is a fixture error. `check` and `query` stdout must use `expected.<command>.stdout`; `.stdout.json` is a fixture error for non-JSON commands. `query.symbol` supplies the fully qualified query argument for `expected.query.*` files.

The shared fixture helper owns only fixture contract mechanics: manifest parsing, fixture discovery, expected stream lookup, and stdout naming validation. Tests for that helper must exercise those mechanics directly. Semantic compiler behavior is still proved by running the public core API, CLI fixtures, and explicit maintainer fixture verification commands against reviewed expected outputs.

Checked-in semantic seed repos under `tests/seeds/` are the preferred seed corpus for semantic integration tests when they already express the needed base architecture shape. Golden command fixtures under `tests/fixtures/` remain the reviewed public command corpus. Inline test-only repository synthesis should remain for targeted deltas and bespoke failure shapes, not as a second hand-maintained copy of the same reviewed seed tree.

Invalid-repository fixtures should pin failure behavior for every public command they can exercise. At minimum, one invalid fixture must assert that `check`, `graph --format json`, and `query` fail without stdout leakage and with deterministic diagnostic stderr.

Checked-in `expected.*` files are reviewed contract oracles, not compiler authority. Maintainers verify them with:

```text
cargo verify-fixtures
```

That command runs the real `ochams` CLI against every exercised fixture and fails on drift without writing fixture projections or source-controlled files.
It builds the CLI with Cargo's lockfile enforced so fixture verification does not update dependency resolution as a side effect. It locates the built CLI from Cargo's reported `compiler-artifact` executable path rather than reconstructing a target-directory convention.

Maintainers may update fixture projections with:

```text
cargo regenerate-fixtures
```

Regeneration is explicit maintainer tooling. It rewrites expected exit, stdout, and stderr files from current CLI output, removes empty stream files, never runs as part of ordinary compilation, and still requires human review of the resulting diff before the projection becomes a contract update.

## Determinism

The tool must produce stable output from stable input.

Rules:

```text
sort emitted symbols by fully qualified name
sort emitted edges by source symbol, relation symbol, target symbol, then source span
sort source files by canonical root-relative path
render paths relative to the chosen root
render paths with forward slashes
disable color in snapshot tests unless explicitly testing color
avoid user-global ignore files for deterministic check output
preserve source spans for every declaration and edge
avoid ordinary hash-map iteration in public output
```

The repository's `.arch` files are the only authoritative architecture store. Derived outputs may be regenerated explicitly, but checked-in fixture projections become public command-contract oracles only after review.

## Internal Architecture

The first implementation should become a Rust workspace after the root scaffold proves the baseline.

```text
crates/
  ochams-core/
  ochams-cli/
  ochams-fixtures/
  xtask/
```

### `ochams-core`

Owns the compiler.

```text
layout
syntax
graph
query
diagnostic
compiler/
  model
  source
  headers
  declarations
  resolution
  checks
  assembly
```

`compiler.rs` owns the public `compile` entrypoint and pipeline orchestration. The private `compiler/` modules own phase-specific behavior:

```text
model          internal compilation data structures
source         source discovery, symlink policy, directory layout validation, source loading, parsed-source creation
headers        workspace, space, and module validation
declarations   declared-symbol collection and declaration-location checks
resolution     imports, references, category checks, region references, resolved-symbol construction
checks         node class, edge class, and relation endpoint checks
assembly       checked graph construction
```

The split is a review boundary, not an invitation for each phase to invent its own architecture. Phase modules share the diagnostic model, but source and symbol data both have deliberate phase states. Source data moves from parsed source units to checked active source units after header validation. Symbol data moves from declarations to resolved graph records after reference resolution.

Parsed source units contain layout classification and parsed statements only. Checked active source units contain layout classification, body statements with headers removed, a fully qualified module, and per-file lexical visibility maps that later phases populate. They exclude the workspace source and do not carry optional `space` or `module` fields into declaration, resolution, checking, or assembly phases.

Declarations are collected before references can be resolved. They contain identity, category, layout authority, and any class derived directly from declaration location. Kind and relation declarations carry classes because vocabulary paths define those classes; node declarations carry identity only until their kind reference resolves. Resolution consumes those declarations and produces category-specific resolved records for the graph. A resolved kind always has a class, a resolved relation always has source and target kinds, and a resolved node always has a kind and kind class. The checked graph must not encode those facts as unrelated optional fields on a single record shape.

`syntax` owns the line-oriented parser and may depend on parser-combinator tooling when that dependency makes the grammar boundary clearer. Parser dependencies must not leak into public graph, diagnostic, or CLI APIs.

No CLI, LSP, filesystem watcher, editor URL, terminal color policy, or external analyzer type belongs in `ochams-core`. Deterministic diagnostic and query text renderers may live in core when they are part of the stable compiler contract rather than terminal presentation.

Evidence data structures may be introduced after the Compiler MVP, but they are not part of the first core crate boundary. The first `ochams-core` should only compile declared architecture into a checked graph.

### `ochams-cli`

Owns command parsing, exit codes, stream selection, and terminal policy. It delegates deterministic diagnostic, graph JSON, and query text rendering to `ochams-core`.

### `ochams-fixtures`

Owns shared repository fixture and seed support for workspace tests and maintainer tooling. It finds the workspace root, reads fixture manifests, validates expected-output file naming, loads expected streams, materializes semantic seed repositories into temporary working trees, and builds command argv for public CLI fixture execution. It must not run the compiler or decide command behavior.

### `xtask`

Owns maintainer-only repository tasks. The Compiler MVP uses it for explicit golden fixture verification and regeneration. It may invoke the built `ochams` CLI, but it must not contain compiler semantics and must not mutate checked-in projections except through an explicit regeneration command.

### Deferred Crates

`ochams-lsp` is deferred until `ochams-core` has stable symbol, diagnostic, source-span, and query contracts. It will own editor protocol translation and convert core data into completions, go-to-definition, diagnostics, document symbols, and workspace symbols. It must not contain compiler rules.

`ochams-scan` is deferred until the Evidence MVP. It will own source traversal and evidence extraction, normalizing anchors and external analyzer output into `SourceAnchor` and `ObservedEdge`. It must not define architecture.

## Crate Strategy

Rust is the implementation language. It is not part of the architecture language.

### Adopted Compiler Slice Dependencies

```text
serde                 serialization
serde_json            graph JSON
winnow                line-oriented statement parser
```

The executable slice intentionally uses Rust's standard library for traversal, diagnostics, CLI argument handling, stable maps, and graph indexes. Parsing uses `winnow` only at the statement-line boundary. This is not a rejection of compiler tooling; it is the weakest useful implementation for the current line-oriented grammar.

Package manifests carry only durable crate metadata: edition, version, license, repository, rust-version, and a concise description. Manifest metadata must describe what each crate is now, not future export targets or imagined integrations. The workspace currently enforces Rust 2024 with a minimum compiler version compatible with that edition.

The public API documentation contract uses Rust's built-in `missing_docs` lint. No documentation generator, README synchronizer, or API diff crate is adopted until the exported surface becomes large enough to make that automation cheaper than direct rustdoc review.

### Adoption Candidates

These crates become appropriate when a measured implementation pressure appears:

```text
logos                 lexer when token-level recovery outgrows line parsing
chumsky               parser with recovery when grammar gains nested forms
ariadne               CLI diagnostic rendering when line/column labels are required
lasso                 string interning when symbol storage becomes measurable overhead
indexmap              deterministic insertion-order maps when BTreeMap ordering is insufficient
la-arena              dense typed arenas when graph revision identity needs compact handles
salsa                 incremental compiler queries when watch/LSP latency matters
clap                  CLI once command shape grows beyond three stable commands
toml                  optional project config
camino                UTF-8 paths
ignore                deterministic repository traversal
globset               path filtering
thiserror             typed library errors
anyhow                binary boundary errors
miette                diagnostic protocol and rich reports
tracing               internal events
tracing-subscriber    trace rendering
```

### Dev Dependency Candidates

The executable slice currently uses standard Rust tests and process-level CLI tests without external test harness crates. These become useful when fixture snapshots become too large or too repetitive for direct assertions.

```text
insta                 snapshots
assert_cmd            CLI integration tests
snapbox               golden command and filesystem tests
predicates            assertion helpers
camino-tempfile       UTF-8 temp fixtures
pretty_assertions     readable equality failures
```

### Explicitly Deferred

```text
petgraph              optional algorithms after custom IR is stable
tree-sitter           editor-grade parsing and structural scanning later
rowan                 lossless syntax tree for refactors later
datafrog              recursive relation checks later
similar               human-readable evidence and graph diffs later
CodeQL integration    high-cost evidence adapter later
SCIP import           precise symbol evidence later
```

## Deferred Capabilities

The MVP should leave room for:

```text
LSP completion and go-to-definition
watch mode
view execution
constraints
source formatting
graph slicing
layout-aware refactors
SCIP import
tree-sitter-backed structural anchors
external analyzer adapters
formal invariant checks
derived diagrams
model-context projections
```

None of these should be allowed to complicate the core language before the typed graph compiler and canonical layout checks are complete.

## Non-Goals

Ochams is not:

```text
a code generator
a graph database
a diagramming language
an ontology reasoner
a universal static analyzer
a replacement for implementation language tooling
a runtime framework
a documentation generator whose output becomes authority
a configurable architecture-layout framework
```

Ochams may emit diagrams, docs, JSON, and context packs. Those are projections.

Ochams may scan implementation-adjacent files. Those scans are evidence.

Ochams may later integrate with external analyzers. Those analyzers are adapters.

## Acceptance Criteria

The MVP is complete when a repository can:

1. Store architecture facts in the canonical `architecture/` tree.
2. Reject unknown or malformed layout regions.
3. Reject declarations written in the wrong layout region.
4. Verify that file paths and `module` declarations align.
5. Enforce mandatory `workspace.arch`, repeated `space`, and non-workspace `module` declarations.
6. Resolve symbols across files with explicit imports or fully qualified references.
7. Reject missing, duplicate, ambiguous, and invalid dotted references.
8. Reject edges whose endpoint kinds do not exactly satisfy the relation.
9. Reject references that violate region reference direction.
10. Coalesce duplicate identical edges while preserving all declaration spans.
11. Emit deterministic graph JSON.
12. Query a symbol's immediate graph context.
13. Run all checks in CI without requiring an editor, graph database, implementation language toolchain, generated implementation code, source scanner, or external analyzer.

The Compiler MVP milestone is:

```text
ochams check <root>
ochams graph <root> --format json
ochams query <root> <symbol>
```

Those commands must validate the canonical layout before evidence scanning and diffing become meaningful.
