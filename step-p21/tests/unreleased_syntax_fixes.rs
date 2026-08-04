//! The two ISO 10303-21 syntax defects this fork exists to ship.
//!
//! Both were fixed upstream in `ricosjp/ruststep` after the `ruststep-v0.4.0`
//! tag (2024-09-20) and never released: `4197eee` (escaped strings) and
//! `90fdc93` (empty parameter lists). Upstream covered each with a unit test on
//! its own sub-parser; these rows pin them through the PUBLIC entry points a
//! consumer actually calls, which is the contract a published crate owes.
//!
//! Both inputs are real, not synthetic:
//!
//! * `''` inside a string is how Part 21 escapes a literal apostrophe, which
//!   imperial CAD emits constantly as an inch mark in thread callouts. Getting
//!   it wrong ends the string early and derails the rest of the file.
//! * `()` is a legal EMPTY aggregate, e.g. `ADVANCED_FACE('',(),#57075,.T.)`
//!   where a face carries no bounds list. Rejecting it fails the whole parse.
//!
//! Each defect made a whole real assembly unreadable, so a regression here is
//! not cosmetic: it silently returns "cannot read this file".

use step_p21::{
    ast::{EntityInstance, Parameter},
    parser::{exchange, parse},
};

/// A minimal but complete exchange structure carrying both constructs.
const STEP: &str = r#"ISO-10303-21;
HEADER;
FILE_DESCRIPTION((''),'2;1');
FILE_NAME('1/2'' NPT coupling','2026-08-04T00:00:00',(''),(''),'','','');
FILE_SCHEMA(('AUTOMOTIVE_DESIGN'));
ENDSEC;
DATA;
#1 = ADVANCED_FACE('',(),#2,.T.);
ENDSEC;
END-ISO-10303-21;
"#;

/// `''` collapses to one apostrophe, and the string does not end early.
#[test]
fn doubled_apostrophe_is_one_apostrophe() {
    let (residual, parsed) = exchange::parameter("'1/2'' NPT coupling'").unwrap();
    assert_eq!(residual, "", "the whole literal must be consumed");
    assert_eq!(parsed, Parameter::String("1/2' NPT coupling".to_string()));
}

/// An empty aggregate is an empty list, not a parse error and not `NotProvided`.
#[test]
fn empty_aggregate_is_an_empty_list() {
    let (residual, parsed) = exchange::parameter("()").unwrap();
    assert_eq!(residual, "");
    assert_eq!(parsed, Parameter::List(vec![]));

    // Distinct from the omitted/unset markers, which have their own spellings.
    assert_ne!(parsed, Parameter::NotProvided);
    assert_ne!(parsed, Parameter::Omitted);
}

/// Both constructs inside one real exchange structure, through `parse`.
///
/// The unit rows above exercise the sub-parsers; this one proves the file as a
/// whole survives, which is what a caller sees.
#[test]
fn a_file_using_both_parses_whole() {
    let exchange = parse(STEP).expect("a file with an escaped string and an empty aggregate");

    // The header kept the inch mark as a single apostrophe.
    let file_name = exchange
        .header
        .iter()
        .find(|record| record.name == "FILE_NAME")
        .expect("FILE_NAME is present");
    let Parameter::List(args) = &file_name.parameter else {
        panic!(
            "FILE_NAME's parameter is an argument list, got {:?}",
            file_name.parameter
        );
    };
    assert_eq!(
        args.first(),
        Some(&Parameter::String("1/2' NPT coupling".to_string())),
        "the doubled apostrophe must survive a full-file parse, not just the sub-parser",
    );

    // The face's empty bounds list survived as an empty list.
    let data = exchange.data.first().expect("one DATA section");
    let entity = data.entities.first().expect("one entity");
    let EntityInstance::Simple { id, record } = entity else {
        panic!("expected a simple entity instance, got {entity:?}");
    };
    assert_eq!(*id, 1);
    assert_eq!(record.name, "ADVANCED_FACE");
    let Parameter::List(args) = &record.parameter else {
        panic!(
            "ADVANCED_FACE's parameter is an argument list, got {:?}",
            record.parameter
        );
    };
    assert_eq!(
        args.get(1),
        Some(&Parameter::List(vec![])),
        "the empty aggregate must survive as an empty list",
    );
}
