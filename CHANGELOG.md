# Fork: step-p21 0.5.0 (2026-08-04)

Forked from [`ricosjp/ruststep`](https://github.com/ricosjp/ruststep) at
`cd89e5a` (upstream `master`, 2025-03-20). The fork exists because
`ruststep-v0.4.0` (2024-09-20) left five commits unreleased, two of which fix
ISO 10303-21 syntax that real CAD exports depend on -- and a `[patch.crates-io]`
entry cannot carry a fix to downstream users.

- `ruststep` renamed to `step-p21`, `ruststep-derive` to `step-p21-derive`. The
  name states the scope: Part 21, the clear-text encoding of the exchange
  structure.
- `step-p21-derive` resolves its runtime path via `crate_name("step-p21")`.
- New `step-p21/tests/unreleased_syntax_fixes.rs` pins the two inherited fixes
  through the public `parse` and `exchange::parameter` entry points: `''` as an
  escaped apostrophe, and `()` as an empty aggregate. Upstream covered each
  sub-parser; these cover the API a caller actually uses.
- `step-p11` / `step-p11-derive` retained unrenamed and unpublished: they are the
  EXPRESS (Part 11) compiler, kept because this workspace's tests use
  `step_p11_derive::inline_express!` to exercise the `Holder`/table machinery.
- Inline `insta` snapshots in `step-p11/tests` were re-ordered: the rename changes
  how rustfmt sorts the generated `use` lines. Generated code is otherwise
  unchanged.

# Changelog

- All notable changes to this project will be documented in this file.
  - The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
  - and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

In addition to original Keep-a-Changelog, we use following rules:

- Use [GitHub Flavored Markdown](https://github.github.com/gfm/)
- Each line in changes SHOULD include a link to Pull Request in GitHub
- Each Pull Request MUST add a line in this file
  - This will be checked by GitHub Actions
- Each Pull Request MAY correspond to one or more lines in this file

## Unreleased

### Fixed

- Fixes to support parsing of ISO13399 database plib. https://github.com/ricosjp/step-p21/pull/251

## 0.4.0 - 2024-09-20

### Added
- Deserialize `LOGICAL` and `BOOLEAN` by `.T.`, `.F.`, and `.U.` notations. https://github.com/ricosjp/step-p21/pull/231

### Changed
- Remove `field` attr from enumerations. https://github.com/ricosjp/step-p21/pull/233
- Recursive `get_owned` for select type without boxed variant. https://github.com/ricosjp/step-p21/pull/234
- Hide `XXHolderVisitor` in document https://github.com/ricosjp/step-p21/pull/247
- Add document to `XXAnyHolder`. https://github.com/ricosjp/step-p21/pull/248

### Fixed
- Plural class names are converted as is. https://github.com/ricosjp/step-p21/pull/245
- Fixed bug in logical_listeral parser. https://github.com/ricosjp/step-p21/pull/244
- Deseialize `Option::Some`. https://github.com/ricosjp/step-p21/pull/232
- Recursive implementation of `step-p21::tables::EntityTable::{get_owned, owned_iter}` for select types. https://github.com/ricosjp/step-p21/pull/230

### Internal
- `cargo upgrade --workspace` https://github.com/ricosjp/step-p21/pull/240
- Upgrading MSRV, updating dependent packages, and supporting the new clippy https://github.com/ricosjp/step-p21/pull/246

## 0.3.0 - 2022-06-14

### Added
- Deserialize Record as a struct https://github.com/ricosjp/step-p21/pull/228
- Add `step-p21::ast::SubSuperRecord` https://github.com/ricosjp/step-p21/pull/225
- Document and tests for serde mapping in step-p21::ast::de https://github.com/ricosjp/step-p21/pull/220
- Document for step-p21::header https://github.com/ricosjp/step-p21/pull/218
- Design document of step-p21 crate https://github.com/ricosjp/step-p21/pull/214
- Term and definitions https://github.com/ricosjp/step-p21/pull/210
- Introduction to STEP, step-p11 design document https://github.com/ricosjp/step-p21/pull/208
- Formal approach for instantiable complex entity https://github.com/ricosjp/step-p21/pull/203
- Partial complex entities described in ISO-10303-11 Annex B https://github.com/ricosjp/step-p21/pull/200

### Changed
- `step-p21::ast::RValue` is renamed to `Name` https://github.com/ricosjp/step-p21/pull/219
- `step-p21::place_holder` is integrated into `step-p21::tables` https://github.com/ricosjp/step-p21/pull/216

### Fixed
- `SUBTYPE_CONSTRAINT` cannot parse supertype-constraint like `ONEOF` https://github.com/ricosjp/step-p21/pull/205

### Internal
- Document for internal mapping https://github.com/ricosjp/step-p21/pull/226
- Reconstruct documents of ast::de https://github.com/ricosjp/step-p21/pull/229
- Do not use RecordDeserializer for Name https://github.com/ricosjp/step-p21/pull/224
- Use Instantiables instead of Constraint https://github.com/ricosjp/step-p21/pull/206
- Return index from ir::Namespace::resolve https://github.com/ricosjp/step-p21/pull/204
- cargo clippy --fix https://github.com/ricosjp/step-p21/pull/202
- Index named AST https://github.com/ricosjp/step-p21/pull/201

## 0.2.0 - 2022-02-17

### Added
- Re-expose serde and itertools from step-p21 https://github.com/ricosjp/step-p21/pull/199
- Add the module created from AP203. https://github.com/ricosjp/step-p21/pull/185
- Implement `AsRef` and `AsMut` for `XXAny`. https://github.com/ricosjp/step-p21/pull/180
- Implement `AsRef` and `AsMut` in the case of multiple inheritance. https://github.com/ricosjp/step-p21/pull/179
- Implements `AsRef`, `AsMut`, `Deref`, `DerefMut` for subtypes. https://github.com/ricosjp/step-p21/pull/174
- Implements `Serialize` to `step-p21::primitive::Logical`. https://github.com/ricosjp/step-p21/pull/170
- Add derive trait `From` and `Into` to the tuple struct generated by simple type declare. https://github.com/ricosjp/step-p21/pull/169
- `ast::Component` trait for converting from String to AST https://github.com/ricosjp/step-p21/pull/168
- Add fields for tuple structs to `Table`. https://github.com/ricosjp/step-p21/pull/151
- Expose the module `derive_more` in `step-p21`. https://github.com/ricosjp/step-p21/pull/144
- step_p11_derive crate, `step_p11_derive::inline_express!` macro https://github.com/ricosjp/step-p21/pull/158
- Expose `step_p21_derive::*` macros in `step-p21::` namespace https://github.com/ricosjp/step-p21/pull/159
- `#[derive(Holder)]` for tuple struct https://github.com/ricosjp/step-p21/pull/146
- Overview diagram written in asciiflow https://github.com/ricosjp/step-p21/pull/137
- `Tables` from `DataSection` https://github.com/ricosjp/step-p21/pull/139
- impl `FromStr` for `Record` and `DataSection` https://github.com/ricosjp/step-p21/pull/140

### Changed
- Generate Holder struct for TYPE declaration with simple type. https://github.com/ricosjp/step-p21/pull/186
- Replace the methods `xxx_iter` of `Tables` with `xxx_holder`.  https://github.com/ricosjp/step-p21/pull/187
- Cut out `IntoOwned` trait from `Holder`. https://github.com/ricosjp/step-p21/pull/183
- Translates `TYPE` declarations in EXPRESS to Rust tuple struct https://github.com/ricosjp/step-p21/pull/144
- Visitor struct and all fields in Holder struct become public https://github.com/ricosjp/step-p21/pull/160
- Drop unused derive_more, and dyn-clone crate dependencies https://github.com/ricosjp/step-p21/pull/159
- Remove `step_p21_derive::as_holder_visitor!` https://github.com/ricosjp/step-p21/pull/147
- Use Rust 2021 edition https://github.com/ricosjp/step-p21/pull/128

### Fixed
- Incorrect `GENERIC` type handling https://github.com/ricosjp/step-p21/pull/198
- Subtype-Supertype dependency graph generation fixed https://github.com/ricosjp/step-p21/pull/161
- Supertype field is not included in subtypes type https://github.com/ricosjp/step-p21/pull/166
- Generate `Into<XxxAny>` correctly https://github.com/ricosjp/step-p21/pull/176
- Use raw identifier `r#` for reserved words. https://github.com/ricosjp/step-p21/pull/172

### Internal
- impl `SeqDeserializer::size_hint` https://github.com/ricosjp/step-p21/pull/197
- Replace `SeqDeserializer` https://github.com/ricosjp/step-p21/pull/194
- Use `Record` struct in `Parameter::Typed` https://github.com/ricosjp/step-p21/pull/192
- Drop type parameter `T` in `SingleMapDeserializer` https://github.com/ricosjp/step-p21/pull/191
- Use `syn::Type` and other explicit types instead of `proc_macro2::TokenStream` in step-p11/codegen https://github.com/ricosjp/step-p21/pull/184
- Legalize the type declare of `SET` and `LIST`. https://github.com/ricosjp/step-p21/pull/171
- Snapshot testing for step-p21-derive https://github.com/ricosjp/step-p21/pull/175
- Refactoring `step-p11::codegen` https://github.com/ricosjp/step-p21/pull/165
- ignore pending snapshot https://github.com/ricosjp/step-p21/pull/164
- Snapshot testing for step-p11 https://github.com/ricosjp/step-p21/pull/163
- step-p21/tests uses `inline_express!` macro https://github.com/ricosjp/step-p21/pull/160
- Add flag to switch step-p21 internal/external codegen in IR::to_token_stream https://github.com/ricosjp/step-p21/pull/158
- Use rust-cache for faster CI https://github.com/ricosjp/step-p21/pull/156
- Comprehensive tests for step_p21_derive https://github.com/ricosjp/step-p21/pull/147
- Check CHANGELOG is updated in each pull request https://github.com/ricosjp/step-p21/pull/155
- Test for `EntityTables` https://github.com/ricosjp/step-p21/pull/136

## 0.1.0 - 2021-09-28

See https://github.com/ricosjp/step-p21/releases/tag/step-p21-0.1.0