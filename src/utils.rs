//! `utils` Module
//!
//! Put utility functions here, or delete it, do you bro.

{% if use_benches %}
{% if is_library %}
pub fn calculate_fib(n: u64) -> u64 {
    {% if use_flamegraph %}
    #[cfg(feature = "flame_on")]
    let _fg = flame::start_guard(format!("{}::calculate_fib({})", module_path!(), n));
    {% endif %}
    match n {
        0 => 0,
        1 => 1,
        n => calculate_fib(n - 1) + calculate_fib(n - 2),
    }
}
{% endif %}
{% endif %}
