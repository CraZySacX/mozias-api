// Copyright © 2019 mozias-api developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! mussh - SSH Multiplexing
//!
//! ```
//! ```
#![feature(crate_visibility_modifier, decl_macro, proc_macro_hygiene)]
#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    bare_trait_objects,
    box_pointers,
    clippy::all,
    clippy::pedantic,
    dead_code,
    elided_lifetimes_in_paths,
    ellipsis_inclusive_range_patterns,
    keyword_idents,
    macro_use_extern_crate,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    missing_doc_code_examples,
    private_doc_tests,
    question_mark_macro_sep,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#![warn(single_use_lifetimes, unsafe_code)]
#![doc(html_root_url = "https://docs.rs/dataq-api/0.1.0")]

use clap::ErrorKind;
use std::error::Error;
use std::process;

mod cors;
mod db;
mod error;
mod fairings;
mod model;
mod routes;
mod run;

/// mozias-api entry point
fn main() {
    match run::run() {
        Ok(_) => process::exit(0),
        Err(error) => {
            if let Some(cause) = error.source() {
                if let Some(err) = cause.downcast_ref::<clap::Error>() {
                    let kind = err.kind;
                    eprintln!("{}", err.message);
                    match kind {
                        ErrorKind::HelpDisplayed | ErrorKind::VersionDisplayed => process::exit(0),
                        _ => process::exit(1),
                    }
                } else {
                    eprintln!("{}", error.description());

                    if let Some(cause) = error.source() {
                        eprintln!(": {}", cause);
                    }
                    process::exit(1);
                }
            } else {
                eprintln!("{}", error.description());

                if let Some(cause) = error.source() {
                    eprintln!(": {}", cause);
                }
                process::exit(1);
            }
        }
    }
}
