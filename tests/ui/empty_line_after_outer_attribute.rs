
#![warn(empty_line_after_outer_attr)]

// This should produce a warning
#[crate_type = "lib"]

/// some comment
fn with_one_newline_and_comment() { assert!(true) }

// This should not produce a warning
#[crate_type = "lib"]
/// some comment
fn with_no_newline_and_comment() { assert!(true) }


// This should produce a warning
#[crate_type = "lib"]

fn with_one_newline() { assert!(true) }

// This should produce a warning, too
#[crate_type = "lib"]


fn with_two_newlines() { assert!(true) }


// This should produce a warning
#[crate_type = "lib"]

enum Baz {
    One,
    Two
}

// This should produce a warning
#[crate_type = "lib"]

struct Foo {
    one: isize,
    two: isize
}

// This should produce a warning
#[crate_type = "lib"]

mod foo {
}

// This should not produce a warning
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[allow(missing_docs)]
fn three_attributes() { assert!(true) }

fn main() { }
