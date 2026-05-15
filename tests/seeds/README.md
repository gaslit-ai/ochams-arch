# Semantic Seed Repositories

These directories are reviewed base repositories for semantic compiler tests in
`crates/ochams-core/tests/compiler.rs`.

They are intentionally separate from `tests/fixtures/`, which is the reviewed
public CLI command corpus. A seed repo may share architectural content with a
golden command fixture, but it is a separate authority so command-fixture
updates do not silently rewrite semantic test baselines.
