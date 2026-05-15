# Ochams MVP Hardening Todo

This todo list tracks the MVP hardening loop across specification, implementation, verification, and refactoring phases.

## Phase 1: Specification Soundness

- [x] Define relation classes as compiler-derived facts from `vocabulary/relations/<class>/**`.
- [x] Define kind classes as compiler-derived facts from `vocabulary/kinds/<class>/**`.
- [x] Replace informal region phrases such as `structural edge` and `behavioral edge` with precise relation-class checks.
- [x] Define whether multiple files may share the same `module`.
- [x] Define how file basenames contribute to symbol identity, if at all.
- [x] Define the minimal grammar accepted by the first milestone.
- [x] Define canonical graph JSON shape for `ochams graph`.
- [x] Define the first diagnostic set that `ochams check` must produce.
- [x] Define MVP fixture scenarios that prove layout, module, symbol, and relation checks.
- [x] Spawn devil's-advocate review agents to challenge the spec before implementation.

## Phase 2: First Review Fixes

- [x] Split the first Compiler MVP from later Evidence MVP.
- [x] Define exact path-to-module algorithm per region.
- [x] Define exact `space` ownership and workspace file rules.
- [x] Define same-module visibility and local scope.
- [x] Define first-milestone source discovery and filesystem rules.
- [x] Define CLI exit, stdout, stderr, color, and JSON failure behavior.
- [x] Define reference units checked by region reference direction.
- [x] Split declared graph from realization/evidence graph.
- [x] Remove or fully define `view` from first-milestone grammar and JSON.
- [x] Define duplicate edge behavior and JSON edge identity.
- [x] Add deterministic JSON formatting rules.
- [x] Re-run devil's-advocate review agents after Phase 2 fixes.

## Phase 3: Second Review Fixes

- [x] Resolve reserved-region comments-only conflict with mandatory headers.
- [x] Clarify active-region common statements versus region-specific declarations.
- [x] Make `vocabulary/` fixed while lower-level flexibility applies only outside vocabulary.
- [x] Define valid source path segment syntax and PascalCase conversion.
- [x] Disambiguate bare identifiers from dotted paths in the grammar.
- [x] Close kind/relation classes for the Compiler MVP.
- [x] Define symbol category mismatch diagnostics and precedence.
- [x] Define workspace source and module representation in graph JSON.
- [x] Fix edge JSON identity to use fully qualified relation symbols.
- [x] Define exact `ochams query` output contract.
- [x] Define half-open byte spans.
- [x] Require empty stdout on successful `ochams check`.
- [x] Re-run devil's-advocate review agents after Phase 3 fixes.

## Phase 4: Final Review Fixes

- [x] Make `ochams query` input and output fully mechanical.
- [x] Define exact source span coverage.
- [x] Tighten path segment grammar so every segment converts to a valid module segment.
- [x] Define fixture directory and expected-output convention.
- [x] Move `ochams-scan` out of the first workspace shape.
- [x] Run final consistency verification.

## Phase 5: Executable Compiler Slice

- [x] Refactor the scaffold into a Rust workspace with a compiler core crate and CLI crate.
- [x] Implement deterministic architecture source discovery without following symlinks.
- [x] Implement layout classification, path-segment validation, and path-to-module validation.
- [x] Implement the line-oriented `.arch` parser with byte spans.
- [x] Implement workspace, space, module, and declaration-location diagnostics.
- [x] Implement global symbol collection with duplicate detection.
- [x] Implement lexical imports, bare-name resolution, fully qualified resolution, and category checks.
- [x] Implement kind/relation class enforcement and region reference direction checks.
- [x] Implement exact relation endpoint typechecking.
- [x] Implement deterministic graph JSON projection with duplicate edge coalescing.
- [x] Implement `check`, `graph --format json`, and `query` CLI behavior.
- [x] Add implementation fixtures/tests for the required Compiler MVP diagnostics and valid graph flow.
- [x] Update the spec for any implementation-discovered prior changes.
- [x] Run full verification.
- [x] Spawn devil's-advocate review agents to challenge the implementation and updated spec.

## Phase 6: Review Repair

- [x] Reject non-space graph statements in `workspace.arch` instead of silently discarding them.
- [x] Reject symlinked `architecture/` roots instead of following them.
- [x] Coalesce duplicate identical edge facts in query output.
- [x] Render malformed CLI invocations as diagnostic-coded stderr.
- [x] Narrow the query API prior to immediate Compiler MVP context.
- [x] Narrow the diagnostic prior to the implemented minimal diagnostic model.
- [x] Clarify inline repository fixtures versus golden fixture files.
- [x] Clarify that core responsibility names are conceptual until phase modules need to split.
- [x] Run final verification after review repairs.

## Phase 7: Golden Command Fixtures

- [x] Promote the public command contract from inline assertions to root-level fixture repositories.
- [x] Define query fixture arguments explicitly so query snapshots are reproducible.
- [x] Add a valid minimal fixture covering `check`, `graph --format json`, and `query`.
- [x] Add an invalid fixture covering deterministic diagnostic stderr.
- [x] Refactor CLI tests to consume fixture metadata and exact expected outputs.
- [x] Keep semantic compiler unit fixtures inline where they are smaller than golden files.
- [x] Update the spec to make the adopted fixture convention match the harness.
- [x] Run full verification.
- [x] Spawn devil's-advocate review agents to challenge the fixture harness and updated spec.

## Phase 8: Golden Fixture Review Repair

- [x] Make fixture metadata drive command execution by discovering every `expected.<command>.exit` file.
- [x] Treat only absent expected stdout/stderr files as empty streams; fail on other read errors.
- [x] Add invalid-repository `graph --format json` failure expectations.
- [x] Add invalid-repository `query` failure expectations.
- [x] Pin unsupported graph format, unknown command, extra args, and missing query args.
- [x] Update the spec to require discovered command fixtures and invalid-repository command coverage.
- [x] Run full verification after fixture review repairs.

## Phase 9: Compiler Phase Boundary Refactor

- [x] Extract compiler pipeline data structures into a private model module.
- [x] Extract deterministic source discovery and filesystem traversal into a source module.
- [x] Extract header validation into a headers module.
- [x] Extract declaration collection into a declarations module.
- [x] Extract import and symbol reference resolution into a resolution module.
- [x] Extract semantic checks into a checks module.
- [x] Extract graph assembly into an assembly module.
- [x] Keep the public `compile` API unchanged.
- [x] Update the spec to describe concrete phase modules rather than only conceptual responsibilities.
- [x] Run full verification.
- [x] Spawn devil's-advocate review agents to challenge the phase split and updated spec.

## Phase 10: Phase Boundary Review Repair

- [x] Make compiled graph facts observable but not publicly mutable.
- [x] Make raw graph records and duplicate edge access crate-private.
- [x] Replace derived graph debug output with a stable projection summary.
- [x] Carry resolved edge origins through resolution instead of coupling checks to edge vector position.
- [x] Classify source layout once and carry the result into parsed-source creation.
- [x] Make `classify` the only per-source layout diagnostic authority.
- [x] Remove stale internal module names from the spec.
- [x] Align the documented compiler pipeline with the implemented phase boundaries.
- [x] Clarify the source module's actual responsibility.
- [x] Clarify core-owned deterministic text rendering versus CLI terminal policy.
- [x] Correct fixture and parser claims that drifted from the implementation.
- [x] Run full verification after review repairs.
- [x] Spawn devil's-advocate review agents to challenge the repaired boundary and updated spec.

## Phase 11: MVP Acceptance and Public Projection Closure

- [x] Rename public graph projection records away from JSON-specific implementation names.
- [x] Re-export every public graph projection record type from `ochams-core`.
- [x] Add an integration test proving external callers can name the public projection surface.
- [x] Replace the placeholder README with a concise Compiler MVP entrypoint.
- [x] Update the spec to define the core public API boundary.
- [x] Make graph JSON rendering return deterministic text without exporting `serde_json::Error`.
- [x] Make projection construction fail loudly if checked graph invariants drift.
- [x] Pin positive fully qualified reference resolution.
- [x] Pin duplicate edge span preservation exactly across files.
- [x] Pin kind and relation query output.
- [x] Clarify README error handling for graph JSON versus query diagnostics.
- [x] Align resolved graph shape terminology with public projection types.
- [x] Run full verification, including rustdoc.
- [x] Spawn devil's-advocate review agents to challenge MVP acceptance and public API closure.

## Phase 12: Declarative Statement Parser Boundary

- [x] Install the current `winnow` parser combinator crate in `ochams-core`.
- [x] Replace whitespace-vector statement parsing with a line-oriented declarative parser.
- [x] Preserve existing statement spans and diagnostic codes.
- [x] Add focused parser tests for blank lines, comments, EOF, invalid identifiers, and trailing tokens.
- [x] Update the spec to record the parser dependency prior and retained line-oriented grammar boundary.
- [x] Make statement shape parsing materially combinator-backed.
- [x] Pin ASCII whitespace and punctuation-adjacency parser behavior.
- [x] Update crate strategy priors for the adopted `winnow` dependency.
- [x] Run full verification, including rustdoc.
- [x] Spawn devil's-advocate review agents to challenge parser dependency choice and behavior preservation.

## Phase 13: Public API Documentation Contract

- [x] Research Rust public API documentation and package metadata expectations.
- [x] Define the exported `ochams-core` surface as a documented compiler contract.
- [x] Add lint enforcement so new public items cannot appear without semantic documentation.
- [x] Document public diagnostic, graph, compilation, and query APIs with durable intent.
- [x] Add crate manifest metadata that describes the packages without introducing future-export claims.
- [x] Update the README and spec to record the public API documentation prior.
- [x] Run full verification, including rustdoc.
- [x] Spawn devil's-advocate review agents to challenge API documentation, manifest metadata, and spec alignment.

## Phase 14: Resolved Symbol Invariant Boundary

- [x] Research Rust enum/newtype guidance for making invalid states unrepresentable.
- [x] Split compiler declarations from resolved graph records.
- [x] Replace optional final symbol fields with category-specific resolved record variants.
- [x] Preserve existing diagnostics, query output, graph JSON, and public API behavior.
- [x] Update the spec to record the resolved-symbol invariant prior.
- [x] Run full verification, including rustdoc.
- [x] Spawn devil's-advocate review agents to challenge invariant closure and behavioral preservation.

## Phase 15: Checked Source Unit Boundary

- [x] Research Rust phase-state patterns for parse-then-check compiler data.
- [x] Split parsed source units from header-checked active source units.
- [x] Remove optional header fields from downstream compiler phases.
- [x] Preserve existing diagnostics, graph JSON, query output, fixture behavior, and public API behavior.
- [x] Update the spec to record the parsed-source versus checked-source prior.
- [x] Run full verification, including rustdoc.
- [x] Spawn devil's-advocate review agents to challenge source-unit invariant closure and behavioral preservation.

## Phase 16: Finite Compiler Policy Authority

- [x] Research current Rust/Cargo macro, build-script, and fixture-tooling guidance against the attached rule bundle.
- [x] Identify duplicated finite semantic policy in layout classification, source discovery, declaration placement, and reference direction.
- [x] Create a single typed policy table plus explicit evaluator functions for closed top-level regions, vocabulary children, kind classes, relation classes, declaration placement, and reference direction.
- [x] Refactor compiler phases to consume the policy evaluators without changing public diagnostics, graph JSON, query output, or CLI behavior.
- [x] Add executable policy-closure tests that catch duplicate, missing, or drifted policy rows before broader semantic changes.
- [x] Update the spec so finite policy is documented as compiler-owned semantic authority, not generated code or macro expansion.
- [x] Run full verification, including rustdoc.
- [x] Spawn devil's-advocate review agents to challenge finite policy authority and behavior preservation.

## Phase 17: Diagnostic Catalog Mechanical Macro Boundary

- [x] Research current Rust declarative macro guidance against the attached macro and semantic-authority rules.
- [x] Identify diagnostic-code duplication across enum variants, string rendering, tests, and spec prose.
- [x] Convert `DiagnosticCode` into a local declarative mechanical catalog whose invocation exposes every public diagnostic identity fact.
- [x] Keep diagnostic semantics, source spans, rendering order, and phase ownership in ordinary functions, not macro expansion.
- [x] Add catalog-closure and public rendering tests that fail when a diagnostic row is omitted, reordered, renamed, or assigned the wrong string.
- [x] Trim the spec so diagnostic catalog authority lives in code while the spec records the boundary and invariants.
- [x] Run full verification, including rustdoc.
- [x] Spawn gpt-5.4 high devil's-advocate review agents to challenge the macro boundary, diagnostic contract, and spec alignment.

## Phase 18: Explicit Golden Fixture Tooling

- [x] Research current Cargo external-tool and build-script guidance against the attached projection-regeneration rules.
- [x] Identify checked-in fixture projections that currently rely on test execution rather than an explicit maintainer command.
- [x] Add a workspace `xtask` binary with `verify-fixtures` and `regenerate-fixtures` commands.
- [x] Make fixture verification run the real `ochams` CLI and fail on drift without mutating checked-in artifacts.
- [x] Make fixture regeneration deterministic, explicit, and separate from ordinary compilation and tests.
- [x] Add discoverable Cargo aliases and documentation for the fixture commands.
- [x] Update the spec so checked-in golden fixtures remain reviewed contract oracles while regeneration is a maintainer aid.
- [x] Run full verification, including the fixture verification command and rustdoc.
- [x] Spawn gpt-5.4 high devil's-advocate review agents to challenge fixture tooling, projection authority, and spec alignment.

## Phase 19: Fixture Contract Helper Regression Boundary

- [x] Research Cargo integration-test and workspace guidance against the fixture helper boundary.
- [x] Identify fixture contract rules that are now shared by CLI tests and maintainer tooling but not directly regression-tested.
- [x] Add focused tests for explicit command manifests, fixture discovery, expected exit parsing, empty stream defaults, and stdout naming rules.
- [x] Keep helper tests local to fixture contract behavior without generating semantic implementation or fixture projections.
- [x] Update the spec only where fixture helper invariants need high-level guidance.
- [x] Run full verification, including fixture verification and rustdoc.
- [x] Spawn gpt-5.4 high devil's-advocate review agents to challenge fixture helper coverage, contract authority, and spec alignment.
- [x] Fix review finding by deriving the built CLI path from Cargo `compiler-artifact` output instead of target-directory guesses.
- [x] Fix review finding by rejecting orphaned expected files and `query.symbol` files for undeclared commands.
- [x] Fix review finding by covering malformed expected streams and regeneration file mutation in focused tests.

## Phase 20: Repository-Wide Fixture Seed Reuse

- [x] Read the repository as one testing system and identify where checked-in fixture repos and inline semantic test repos duplicate the same architecture shapes.
- [x] Reuse checked-in semantic seed repos as seed inputs for compiler tests wherever the seed already expresses the needed base shape.
- [x] Keep inline repository synthesis only for tests that genuinely need bespoke shapes or targeted mutations not represented by reviewed fixtures.
- [x] Update the lean spec only if the testing prior changes in a way that matters beyond one file.
- [x] Run full verification, including fixture verification and rustdoc.
- [x] Spawn gpt-5.4 high devil's-advocate review agents to challenge the repo-wide test-topology simplification and spec alignment.
- [x] Fix review finding by separating semantic seed repos from public golden command fixtures.
- [x] Fix review finding by tightening seeded negative tests from “contains code” to exact diagnostic-code sets where the scenario is single-failure by construction.
- [x] Fix review finding by reusing the checked-in `missing-space` seed and documenting the broader checked-in fixture corpus.
