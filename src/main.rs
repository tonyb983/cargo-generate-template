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
// Shut clippy up during tests...
#![cfg_attr(
    test,
    allow(
        clippy::pedantic,
        clippy::all,
        missing_docs,
        nonstandard_style,
        rust_2018_idioms,
        rust_2018_compatibility,
        rust_2021_compatibility,
        rustdoc::all
    )
)]
// This removes some of the more annoying / pointless lints:
#![allow(
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
    no_coverage,
    once_cell,
    pattern,
    round_char_boundary,
    test,
    try_blocks
)]{% else %}
// This enables a coverage attribute to exclude code from coverage numbers, see test function below for usage example.
// This can also be enabled globally using the NIGHTLY feature `no_coverage` (`#![feature(no_coverage)]`), in which case it becomes
// easier to disable coverage for a function by simply adding
//     #[no_coverage]
// to the function, instead of having to use
//     #[cfg_attr(coverage, no_coverage)]
#![cfg_attr(coverage, feature(no_coverage))]
{% endif %}

{% if use_flamegraph %}mod flame_example;{% endif %}
{% if use_puffin %}mod puffin_example;{% endif %}
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello from {{project-name}}!");

    Ok(())
}

fn test_function() -> String {
    "Hello from {{project-name}}::test_function!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    {% if use_nightly %}#[no_coverage]{% else %}#[cfg_attr(coverage, no_coverage)]
    {% endif %}#[test]
    fn good() {
        assert_eq!(test_function(), "Hello from {{project-name}}::test_function!");
    }

    {% if use_nightly %}#[no_coverage]{% else %}#[cfg_attr(coverage, no_coverage)]
    {% endif %}#[test]
    #[should_panic]
    fn bad() {
        assert_ne!(test_function(), "Hello from {{project-name}}::test_function!");
    }{% if use_flamegraph %}

    // This test will run the flamegraph generation code (useful for library projects).
    // It is being ignored so as not to run on every test run, such as in CI environments, but this doesnt have to be the case.
    {% if use_nightly %}#[no_coverage]{% else %}#[cfg_attr(coverage, no_coverage)]
    {% endif %}#[test]
    #[ignore]
    fn run_flamegraph() {
        flame_example::flamegraph_main();
    }{% endif %}
}
