//! Example file showing how to use the `flame` and `flamer` libraries to generate flamegraph information.
//!
//! - `flamer` attribute examples: `attribute_flame`, `flamer_with_locals`
//! - `flame` manual usage examples: `manual_flame`, `layered_scopes`, `using_module_path`
//! - flamegraph output generation: `flamegraph_main`
//!
//! The `flamer` crate is completely optional, but makes it much easier to intersperse `flame` scope guards
//! into your codebase. Keep in mind that flamer does not generate any sort of output when used by itself,
//! the functions from the `flame` crate must be used to dump the output.
//! See: `flame::dump_html`, `flame::dump_json`, `flame::dump_stdout`.
//!
//! Gating the flamegraph code behind a feature is also technically optional but should always be done to ensure that
//! the function call overhead is only added when the performance is actually being measured.

/// This attribute will ensure that the flame portion of the code is only applied when the
/// `flame` feature is enabled. This feature can be renamed to whatever you like, but
/// make sure you read [the rules regarding features](https://doc.rust-lang.org/cargo/reference/features.html).
/// Due to the attribute, this code will automatically have the flame guard code inserted.
/// It will be registered under the name `flame_example::attribute_flame` due to the string
/// inside of `flamer::flame(..)`, without this it would only show up as `attribute_flame`
/// which can be problematic for bigger projects.
#[cfg_attr(feature = "flame", flamer::flame("flame_example"))]
pub fn attribute_flame() {
    println!("Hello from attribute_flame");
}

/// This example only demonstrates that even local functions will be separately flamed
/// when using the `flamer` library. The top function will show up as
/// `flame_example::flamer_with_locals` and the local functions will show up as
/// `flame_example::flamer_with_locals::local_functions_too`.
///
/// The second `cfg_attr` is required since the `flamer::flame` attribute adds generated code
/// before any of your own code, so clippy will complain about items after statements.
/// This can also be fixed project-wide by adding the following to your `lib.rs` or `main.rs`:
/// ```rust
/// #![cfg_attr(feature = "flame", allow(clippy::items_after_statements))]
/// ```
#[cfg_attr(feature = "flame", flamer::flame("flame_example"))]
#[cfg_attr(feature = "flame", allow(clippy::items_after_statements))]
pub fn flamer_with_locals() {
    fn local_functions_too() {
        println!("Hello from local_functions_too");
    }
    println!("Hello from flamer_with_locals");
    local_functions_too();
    local_functions_too();
    println!("Hello from flamer_with_locals");
}

/// This function shows how to manually add a `flame` scope guard to your function, and is
/// largely equivalent to the code that is inserted by `flamer::flame`.
///
/// # IMPORTANT
/// It is important to note that the variable which `flame::start_guard` is assigned to ***MUST BE NAMED***,
/// it cannot be `_`. When you name a variable `_`, the drop code for that variable **will not be run** when it
/// goes out of scope. This is a very important distinction to keep in mind, and luckily clippy / rustc should
/// warn you whenever you assign a type that has a `Drop` implementation to a variable named `_`. With that
/// in mind, your variable should *probably* still be **PREFIXED** with `_`, so that clippy does not complain
/// about unused variables, but that of course is up to you.
pub fn manual_flame() {
    #[cfg(feature = "flame")]
    let _fg = flame::start_guard("flame_example::manual_flame");
    println!("Hello from manual_flame");
}

/// This example demonstrates how scopes can be layered for more granular measurements, and
/// multiple ways it can be accomplished.
pub fn layered_scopes() {
    #[cfg(feature = "flame")]
    let _fg = flame::start_guard("flame_example::layered_scopes");

    // Scoped version
    {
        #[cfg(feature = "flame")]
        let _fg2 = flame::start_guard("flame_example::layered_scopes::expensive_operation_1");
        // Assume this is one of many expensive operations done by this function
        for _ in 0..1_000_000 {
            let _s = String::with_capacity(4_000);
        }
    } // _fg2 is dropped here, running the flamegraph capture code

    // Manual drop version
    #[cfg(feature = "flame")]
    let _fg3 = flame::start_guard("flame_example::layered_scopes::expensive_operation_2");
    for _ in 0..1_000_000 {
        let _v = Vec::<(usize, usize, usize, usize)>::with_capacity(4_000);
    }
    // Drop so the capture code is run now
    #[cfg(feature = "flame")]
    drop(_fg3);

    // Do some other stuff here maybe.
    println!("Hello from layered_scopes");

    // Keep in mind that situations like this would *likely* be handled better by extracting each expensive
    // operation into a separate function, but that is just personal preference.
}

/// This example just demonstrates using the `module_path!` macro to make things a tiny bit easier,
/// especially for deeply nested modules.
pub fn using_module_path() {
    #[cfg(feature = "flame")]
    let _fg = flame::start_guard(format!("{}::using_module_path", module_path!()));
    println!("Hello from using_module_path");
}

/// This function demonstrates how to actually get the flamegraph data output. All of the above code sets up
/// the capturing, but without actually requesting the output, nothing will be generated.
#[cfg(feature = "flame")]
pub fn flamegraph_main() {
    // This part is optional, but will give you a baseline scope for the entire run.
    let _fg = flame::start_guard("flame_example::main");

    // Call the flamed functions (you normally wouldn't do this manually, it would just happen in the course of your program)
    attribute_flame();
    flamer_with_locals();
    manual_flame();
    layered_scopes();
    using_module_path();

    // Here is where the flamegraph output is generated
    // (Note the use of scopes here is only to better group the calls for each output method,
    // they are not necessary for any of the code itself.)

    // First an HTML dump
    {
        // The `std::time` call here is used to create filenames such as `flames/flame_example.1934903562.html`,
        // useful for ensuring the results from each run are kept and nothing is overwritten.
        // `flame` will automatically use your `CARGO_MANIFEST_DIR` (AKA your project directory) as the base directory
        // for whatever path you give it.
        let html_filename = format!(
            "flames/{}.{}.html",
            "flame_example",
            std::time::UNIX_EPOCH
                .elapsed()
                .expect("Unable to get time since epoch")
                .as_secs()
        );
        // `flame::dump_html` & `flame::dump_json` take a `std::io::Write` object, so we create a `std::fs::File` to pass to it.
        // Note the use of `File::options` to ensure no files are overwritten.
        let html_file = std::fs::File::options()
            .write(true)
            .create_new(true)
            .open(html_filename)
            .expect("Unable to open/create html file.");
        flame::dump_html(html_file).expect("Unable to dump flamegraph data to html");
    }

    // Next a JSON dump:
    {
        // This creates a path like `flames/flame_example.json`, useful when you'd like to keep only the most recent results.
        let json_filename = "flames/flame_example.json";
        // Note the use of `File::options` to ensure the file is truncated and overwritten if it already exists, and created otherwise.
        let mut json_file = std::fs::File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(json_filename)
            .expect("Unable to open/create json file.");
        flame::dump_json(&mut json_file).expect("Unable to dump flamegraph data to json");
    }

    // Finally the stdout dump:
    {
        // This is the only call that does not need an argument, and does not return a result.
        flame::dump_stdout();
    }
}

/// Version of the function indicating that this code should only be run with the `flame` feature.
/// This is useful so your entry point function can call this universally, without having to use `cfg`
/// attributes.
#[cfg(not(feature = "flame"))]
pub fn flamegraph_main() {
    println!("This function should be called with the `flame` feature enabled.");
}
