# Ochams Architecture Compiler

Ochams is a language-agnostic architecture compiler. It treats architecture as typed source text under a canonical `architecture/` tree, compiles that source into a checked graph, and exposes deterministic diagnostics, graph JSON, and immediate symbol context queries.

The Compiler MVP is intentionally small:

```text
ochams check <root>
ochams graph <root> --format json
ochams query <root> <symbol>
```

The source tree is the authoritative store. Generated JSON, query text, diagrams, documents, and context packs are projections.

## Model

An Ochams repository contains architecture facts, not implementation code:

```text
architecture/
  workspace.arch
  vocabulary/
    kinds/
    relations/
  domain/
  capabilities/
  boundaries/
```

`workspace.arch` declares the root `space`. Every other active `.arch` file repeats that `space`, declares its path-derived `module`, and then declares or references graph facts.

```text
space VetClinic
module Domain.Resources

use VetClinic.Vocabulary.Kinds.Entity

node Pet : Entity
```

Paths define authoring context. Source declarations define graph facts. Moving a file is therefore an architectural refactor because the legal region and path-derived module change together.

## Core API

`ochams-core` exposes the checked compiler boundary:

```rust
use ochams_core::{compile, format_diagnostics, format_query};

let compilation = compile("/path/to/project");
match compilation.graph {
    Some(graph) => {
        let json = graph.to_pretty_json();
        let query = format_query(&graph, "VetClinic.Domain.Resources.Pet")
            .unwrap_or_else(|diagnostic| format_diagnostics(&[diagnostic]));
    }
    None => {
        let diagnostics = format_diagnostics(&compilation.diagnostics);
    }
}
```

The public graph surface is projection-oriented. `Graph::projection()` returns `GraphProjection` and the stable view records `GraphWorkspaceSource`, `GraphSource`, `GraphKind`, `GraphRelation`, `GraphNode`, and `GraphEdge`. Raw compiler records remain internal.

Diagnostics are part of the same public contract. `DiagnosticCode::ALL` is the code-owned catalog in canonical order, and `DiagnosticCode::as_str()` returns the stable `OCH###` text used by `format_diagnostics`.

`ochams-core` denies undocumented public items. Rustdoc therefore describes the exported compiler contract, while generated documentation remains a disposable projection of the source.

The snippet above shows the primary flow. Supporting exported types such as diagnostics, source spans, compilation results, and graph projection records are part of the Rustdoc contract too.

## Verification

```text
cargo fmt --check
cargo check
cargo clippy -- -D warnings
cargo test
cargo verify-fixtures
cargo doc --no-deps
```

`cargo verify-fixtures` checks the committed command fixtures without writing fixture files and enforces the Cargo lockfile while building the CLI. `cargo regenerate-fixtures` rewrites fixture projections from the current `ochams` CLI; review those diffs before keeping them.
