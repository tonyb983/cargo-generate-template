//! `utils` Module
//!
//! Put utility functions here, or delete it, do you bro.

{% if use_benches %}
{% if is_library %}
/// Simple function calculating the fibonacci value of a given number `n` recursively.
/// Used only for the criterion benchmarking example and can be safely removed.
#[must_use]
pub fn calculate_fib(n: u64) -> u64 {
    {% if use_flamegraph %}
    #[cfg(feature = "flame")]
    let _fg = flame::start_guard(format!("{}::calculate_fib({})", module_path!(), n));
    {% endif %}
    {% if use_puffin %}
    #[cfg(feature = "puffin")]
    let _fg = flame::start_guard(format!("{}::calculate_fib({})", module_path!(), n));
    {% endif %}
    match n {
        0 => 0,
        1 => 1,
        n => calculate_fib(n - 1) + calculate_fib(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    {% if use_nightly %}#[no_coverage]{% else %}#[cfg_attr(coverage, no_coverage)]
    {% endif %}#[test]
    fn it_works() {
        assert_eq!(calculate_fib(0), 0);
        assert_eq!(calculate_fib(1), 1);
        assert_eq!(calculate_fib(2), 1);
        assert_eq!(calculate_fib(3), 2);
        assert_eq!(calculate_fib(4), 3);
        assert_eq!(calculate_fib(5), 5);
        assert_eq!(calculate_fib(10), 55);
    }
}
{% endif %}
{% endif %}
