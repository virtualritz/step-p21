# Repository Guidelines

## Blueprint Baseline

This repo adopts the shared [`blueprints`](https://github.com/virtualritz/blueprints)
baseline via the `.blueprints/` git submodule. Those files are the cross-project
default for agent behavior, code quality, and microtypography; the
project-specific rules in this file take precedence wherever they conflict.

Core rules (read first):

- [Agent behavior](.blueprints/base/AGENTS.md)
- [Rust agent rules](.blueprints/lang/rust/AGENTS.md) and [Rust testing](.blueprints/lang/rust/testing.md)
- [Git safety](.blueprints/base/git-safety.md) and [test ownership](.blueprints/base/test-ownership.md)

Reference:

- [Microtypography](.blueprints/base/microtypography.md) -- slashes, dashes, quotes, ellipses, spacing.
- [Documentation standards](.blueprints/base/documentation.md)
- [Commit messages](.blueprints/base/commit-messages.md)
- [API change protocol](.blueprints/base/api-changes.md)

Fresh clones need the submodule: `git submodule update --init --recursive`.

## What This Crate Is, And Is Not

`step-p21` implements **ISO 10303-21**: the clear-text encoding of the exchange
structure, i.e. the syntax of a `.step` file. It tokenizes, parses and serializes
that syntax, and hydrates entity tables through the `Holder` machinery.

It does **not** own the *vocabulary*. What `PLANE`, `ADVANCED_FACE` or
`MANIFOLD_SOLID_BREP` mean is Part 42/43, the AICs and the application protocols
(AP203, AP214, AP242) -- a consumer's concern. Keep that boundary: entity
semantics belong in the consumer, not here.

`step-p11` and `step-p11-derive` are the EXPRESS (Part 11) schema compiler. They are
retained because this workspace's own tests use `step_p11_derive::inline_express!` to
exercise the `Holder`/table machinery, and they are **not published**.

## Build, Test, and Development Commands

```bash
# Run everything (this also builds everything).
cargo test --workspace

# One crate.
cargo test -p step-p21

# Format and lint before committing.
cargo fmt --all
cargo clippy --workspace --all-targets
```

`crates/step-p11/tests/*.rs` use **inline** `insta` snapshots, which live in the test source
rather than in `.snap` files. `INSTA_UPDATE` will not rewrite them; use
`cargo insta` or edit the literal. A rename that changes how `rustfmt` sorts the
generated `use` lines will move those snapshots -- that is expected, but read the
diff to confirm nothing else moved with it.

## Naming

Public names are spelled out. Prefer the full word over an abbreviation a reader
has to decode: `parameter` not `param`, `reference` not `ref`, `identifier` not
`id` where the long form is the domain term. This is deliberately different from
upstream, which inherited terse EXPRESS-flavored names.

Every rename ships a `#[deprecated]` alias so code written against upstream
`ruststep` or an earlier `step-p21` keeps compiling. Removing an alias is a
breaking change and needs a major version.

Standard-mandated spellings are not ours to change: `ISO-10303-21`, entity
keywords (`ADVANCED_FACE`), and EXPRESS keywords stay exactly as the standard
writes them, including case.

## Tests

A test's name states the invariant, not the mechanics. Prefer inputs taken from
real exchange files over synthetic ones, and say in a comment where the shape
came from -- a regression row is only convincing if the reader can see why that
input matters.

Two behaviors are load-bearing for downstream users and must never regress:
`''` as an escaped apostrophe, and `()` as an empty aggregate. See
`crates/step-p21/tests/unreleased_syntax_fixes.rs`.

## Upstream

This is a fork of [`ricosjp/ruststep`](https://github.com/ricosjp/ruststep)
(Apache-2.0). Keep the LICENSE and the upstream copyright, and record divergence
in `CHANGELOG.md` -- Apache-2.0 requires stating changes. Fixes that are not
fork-specific are worth offering upstream.
