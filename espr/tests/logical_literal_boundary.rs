//! An identifier that merely *starts* with a logical keyword is an identifier.
//!
//! Upstream issue #243 reported `TrueNorth` tokenizing as `TRUE` plus a stray
//! `North`, and was fixed by requiring a non-letter after the keyword. That
//! guard was too narrow: an EXPRESS `simple_id` continues with a letter, a digit
//! or an underscore, so `FALSE_POSITIVE`, `TRUE_NORTH`, `TRUE2` and `UNKNOWN_9`
//! still failed to parse -- and those are ordinary names to find in a schema.

use espr::ast::SyntaxTree;

/// Uses the identifier where only an identifier is legal, so a parse failure
/// means the tokenizer split the word.
fn parses(identifier: &str) -> bool {
    let source = format!(
        "SCHEMA s;\n ENTITY e;\n  WHERE\n   w : NOT(EXISTS({identifier}));\n END_ENTITY;\nEND_SCHEMA;"
    );
    SyntaxTree::parse(&source).is_ok()
}

#[test]
fn identifiers_beginning_with_a_logical_keyword_are_identifiers() {
    for identifier in [
        "North",          // no keyword prefix at all
        "TrueNorth",      // the shape reported upstream in #243
        "TRUENorth",      // continues with a letter
        "TRUE_NORTH",     // continues with an underscore
        "TRUE2",          // continues with a digit
        "FALSE_POSITIVE", // an entirely ordinary schema name
        "UNKNOWN_9",
    ] {
        assert!(
            parses(identifier),
            "`{identifier}` must parse as an identifier"
        );
    }
}

#[test]
fn the_bare_keywords_are_still_logical_literals() {
    // The boundary guard must not cost us the literals themselves.
    for source in [
        "SCHEMA s;\n ENTITY e;\n  WHERE\n   w : TRUE;\n END_ENTITY;\nEND_SCHEMA;",
        "SCHEMA s;\n ENTITY e;\n  WHERE\n   w : FALSE;\n END_ENTITY;\nEND_SCHEMA;",
        "SCHEMA s;\n ENTITY e;\n  WHERE\n   w : UNKNOWN;\n END_ENTITY;\nEND_SCHEMA;",
    ] {
        assert!(
            SyntaxTree::parse(source).is_ok(),
            "bare keyword must still parse:\n{source}"
        );
    }
}
