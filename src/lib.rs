//! # {{project-name}}
//!
//! Crate documentation should go here.

// This makes clippy as loud and annoying as possible, you may want to comment some of them out while developing.
#![warn(
    clippy::pedantic,
    clippy::all,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    rust_2018_compatibility,
    rust_2021_compatibility,
    rustdoc::all
)]
// This removes some of the more annoying / pointless lints:
#![allow(
    // This is a library so there's going to be a lot of unused
    unused,
    // I will remove this later on, but for now it's less pointlessly annoying
    dead_code,
    // I hate this lint
    clippy::module_inception,
    // I also hate this lint
    clippy::module_name_repetitions,
    // I am undecided on this lint
    clippy::unnecessary_wraps,
    // I use them sparingly and only when appropriate
    clippy::wildcard_imports,
)]

{% if use_nightly %}// Here are a bunch of cool features that can be used with the nightly toolchain. You should remove
// any that you don't plan on actually using.
#![feature(
    associated_type_defaults,
    backtrace,
    box_patterns,
    box_syntax,
    inline_const,
    // This feature is "incomplete" and therefore more experimental than the others.
    // inline_const_pat,
    const_trait_impl,
    control_flow_enum,
    concat_idents,
    crate_visibility_modifier,
    default_free_fn,
    exclusive_range_pattern,
    explicit_generic_args_with_impl_trait,
    half_open_range_patterns,
    let_chains,
    let_else,
    lint_reasons,
    once_cell,
    pattern,
    round_char_boundary,
    test,
    try_blocks
)]

// This enables a coverage attribute to exclude code from coverage numbers, see test function below for usage example.
// This can also be enabled using the above feature list (with feature(no_coverage)`), in which case it becomes
// easier to disable coverage for a function by simply adding 
//     #[no_coverage] 
// to the function, instead of having to use
//     #[cfg_attr(coverage, no_coverage)]
#![cfg_attr(coverage, feature(no_coverage))]{% endif %}

mod utils;

fn test_function() -> String {
    "Hello from {{project-name}}::test_function!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    {% if use_nightly %}#[cfg_attr(coverage, no_coverage)]
    {% endif %}#[test]
    fn good() {
        assert_eq!(test_function(), "Hello from {{project-name}}::test_function!");
    }

    {% if use_nightly %}#[cfg_attr(coverage, no_coverage)]
    {% endif %}#[test]
    #[should_panic]
    fn bad() {
        assert_ne!(test_function(), "Hello from {{project-name}}::test_function!");
    }
}