# `step-p21`

## Overview

`step-p21` reads and writes **ISO 10303-21**, the clear-text encoding of the
STEP exchange structure -- the syntax of a `.step` file. It is a fork of
[`ruststep`](https://github.com/ricosjp/ruststep).

The name states the scope. Part 21 is the *encoding*; what `PLANE` or
`ADVANCED_FACE` mean lives in Part 42, the AICs and the application protocols
(AP203, AP214, AP242), and that vocabulary is a consumer's concern. This crate
gives you the syntax tree and the table machinery to hydrate typed entities from
it.

## Why Was This Forked?

**Upstream stopped publishing.** `ruststep-v0.4.0` was tagged **2024-09-20**, and
the last commit on `master` is **2025-03-20**, which left five commits stranded:
fixed in the repository, absent from every released version.

Two of those five fix Part 21 syntax that real CAD exports depend on:

- `''` inside a string is Part 21's escape for a literal apostrophe. Imperial CAD
  emits it constantly as an inch mark in thread callouts. Mis-parsing it ends the
  string early and derails the rest of the file.
- `()` is a legal **empty aggregate**, e.g. `ADVANCED_FACE('',(),#57075,.T.)`.
  Rejecting it fails the whole parse.

Each defect made entire real assemblies unreadable, and a `[patch.crates-io]`
entry cannot help: it fixes the patcher's own build and never its dependents. The
only way those fixes reach downstream users is a published crate. Hence this one.

Verified rather than assumed: on released `ruststep` 0.4.0 a file containing
`ANNOTATION_PLANE('name',(#1),#1,())` fails to parse; on `step-p21` it parses.
That is upstream issue
[#256](https://github.com/ricosjp/ruststep/issues/256) outright, and it is the
demonstrated cause of the symptom reported in
[#252](https://github.com/ricosjp/ruststep/issues/252).

The second goal is **ergonomics**. The upstream codebase carries terse
EXPRESS-flavored names and occasionally confusing phrasing. Public names here are
spelled out, documentation is written to be read by someone who is not already
fluent in ISO 10303, and the language is kept inclusive and plain -- whether you
are a non-native English speaker or a seasoned CAD engineer. Every rename ships a
`#[deprecated]` alias, so code written against `ruststep` keeps compiling.

Fixes that are not fork-specific are worth offering upstream. Nothing here is
intended as a hostile fork.

### Changes Since the Fork

See [`CHANGELOG.md`](CHANGELOG.md); Apache-2.0 requires stating modifications, so
divergence is recorded there rather than summarised loosely.


| name | crates.io | docs.rs | description |
|:-----|:----------|:--------|-------------|
| `step-p11` | _not published_ | | | [EXPRESS Language (ISO 10303-11)][EXPRESS] compiler, used only by this workspace's tests and the `expressc` binary |
| `step-p11-derive` | _not published_ | | | proc-macro that runs the `step-p11` compiler |
| step-p21 | [![Crate](https://img.shields.io/crates/v/step-p21.svg)](https://crates.io/crates/step-p21) | [![docs.rs](https://docs.rs/step-p21/badge.svg)](https://docs.rs/step-p21)|Serialize/Deserialize STEP files|
| step-p21-derive | [![Crate](https://img.shields.io/crates/v/step-p21-derive.svg)](https://crates.io/crates/step-p21-derive) | [![docs.rs](https://docs.rs/step-p21-derive/badge.svg)](https://docs.rs/step-p21-derive)|proc-macro helper crate|

[EXPRESS]: https://www.iso.org/standard/38047.html

## What is STEP?

- STEP is a set of data serialize formats, schema language, and common schemas.
- Data serialize format is called **exchange structure** in ISO document, but usually called **STEP file**,
  They are serialized as ASCII text (ISO-10303-21, usually with extension `*.step`, `*.stp` or `*.p21`) or XML (ISO-10303-28).
- Schema language is called **EXPRESS**. EXPRESS file is usually named with extension `*.exp`.
- Many common schemas are defined in ISO-10303 by EXPRESS language.
  - [`schemas`](https://github.com/virtualritz/step-p21/tree/master/schemas) contains copies
  - Application Protocol (AP) is a class of defined schemas, and the main target of this project.
  - AP203 is most famous one in CAD (computer-aided design) applications.

### Rosetta Stone for web developers

|                 | Protocol Buffers                                           | STEP (ISO 10303)                                         |
|:----------------|:-----------------------------------------------------------|:---------------------------------------------------------|
| Schema Language | [Protocol Buffers Version 3 Language Specification][pbspec]| EXPRESS Language (ISO 10303-11)                          |
| Schema file     | `*.proto` file                                             | `*.exp` file                                             |
| Data            | [Encoded Binary data][pbencoding]                          | "Exchange structure", `*.step` file (ASCII, ISO 10303-21)|
| Compiler        | protoc                                                     | expressc                                                    |

[pbspec]: https://developers.google.com/protocol-buffers/docs/reference/proto3-spec
[pbencoding]: https://developers.google.com/protocol-buffers/docs/encoding

## Why `step-p21`?

- STEP is not only for CAD
  - EXPRESS is a general data schema like [Protocol Buffers][pbspec]
  - Later ISOs also uses EXPRESS
    - [ISO 13584 "Industrial automation systems and integration - Parts library"](https://www.iso.org/standard/43423.html)
    - [ISO 13399 "Cutting tool data representation and exchange"](https://www.iso.org/standard/36757.html)
  - For computer-aided technologies (CAx) including:
    - computer-aided manufacturing (CAM)
    - computer-aided engineering (CAE)
    - product data management (PDM/EDM)
    - manufacturing resource planning (MRP)
    - enterprise resource planning (ERP)
- We have to generate many codes from EXPRESS schemas
  - Tables for SQL or NoSQL database, Object-Record Mapper (ORM)
  - on-wire, on-memory format
    - ASCII and XML are less efficient than a binary format such as protocol buffers
- Extensible EXPRESS compiler is required
  - Like as protoc generates gRPC binding using gRPC-plugin

## License

Copyright 2021 RICOS Co. Ltd. (upstream `ruststep`)
Copyright 2026 Moritz Moeller (this fork)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

### Exception

The following directories are included for development purpose.
**They are not parts of this project**.

- [`schemas`](https://github.com/virtualritz/step-p21/tree/master/schemas)
- [`crates/step-p21/tests/steps`](https://github.com/virtualritz/step-p21/tree/master/crates/step-p21/tests/steps)

## Contributing

Contributions are accepted under the same Apache-2.0 licence as the rest of the
crate; there is no separate contributor agreement and no CLA bot on this fork.

Fixes that are not fork-specific are worth offering to
[upstream](https://github.com/ricosjp/ruststep) as well.
