# Golden Command Fixtures

The `repo/` directories are authoritative fixture inputs. The checked-in
`expected.*` files are reviewed public command-contract oracles derived from
those inputs and the current CLI contract.

Each fixture has a human-authored `commands.txt` manifest. That manifest chooses
which public commands are part of the fixture contract; expected files provide
the oracle for those commands but do not decide coverage. Expected files and
`query.symbol` for undeclared commands are stale fixture errors.

Use:

```text
cargo verify-fixtures
cargo regenerate-fixtures
```

`cargo verify-fixtures` runs the real `ochams` binary and fails on drift without
writing fixture files. It builds the binary with Cargo's lockfile enforced.
`cargo regenerate-fixtures` rewrites expected exit, stdout, and stderr
projections deterministically; regenerated diffs still need human review before
they become contract updates.
