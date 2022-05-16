//! Pre-Setup Script
//!
//! I've chosen to use this to prompt the user for input rather than relying on `cargo-generate.toml`, it
//! bugged me a little bit that on each run, the order of prompts seemed to be randomized, this way they will
//! always follow the same order so as to be more consistent.

fn log(x) {
    const DEBUG = true;
    if DEBUG {
        debug(x);
    }
}
log("Running project pre-setup script...");
print("🌟 Welcome to the only Rust project template you'll ever need. 🌟");

let LICENSES = #{
  "Apache-2.0": "LICENSE-APACHE",
  "Apache-2.0 OR MIT": "",
  "BSL1": "LICENSE-BSL1",
  "BSD2": "LICENSE-BSD2",
  "BSD3": "LICENSE-BSD3",
  "GPLv2": "LICENSE-GPL2",
  "GPLv3": "LICENSE-GPL3",
  "MIT": "LICENSE-MIT",
  "MPL-2.0": "LICENSE-MPLv2",
  "Unlicense": "LICENSE-UNLICENSE",
  "None": "",
};

// Here we set up the conditional variables that determine what files are kept and what segments are inserted
// into the templated files.

// Prompt for project type, this is done by cargo generate but I live having control so whatever. Default = 'library'
// Originally: placeholders.is_library = { type = "bool", prompt = "...", default = true }
let project_kind = variable::prompt("Is this project a library or binary?", "library", ["library", "binary"]);
variable::set("is_library", project_kind == "library");
log(`script project_kind = ${project_kind}`);
log(` outer is_library = ${variable::get("is_library")}`);

// Originally: placeholders.gh_username = { type = "string", prompt = "...", default = "tonyb983" }
// TODO: Should / can this be made optional? Also, should we maybe prompt whether the user wants to keep all of the README badges?
let gh_username = variable::prompt("Please enter your github username (used for setting up gh actions and readme badges):", "tonyb983");
variable::set("gh_username", gh_username);
log(`script gh_username = ${gh_username}`);
log(` outer gh_username = ${variable::get("gh_username")}`);

// Originally: placeholders.gh_workflows = { type = "bool", prompt = "...", default = true }
// TODO: Should this be changed to default 'false'?
let gh_workflows = variable::prompt("Would you like to enable Github Actions for linting, testing, and code coverage?", true);
variable::set("gh_workflows", gh_workflows);
log(`script gh_workflows = ${gh_workflows}`);
log(` outer gh_workflows = ${variable::get("gh_workflows")}`);

// Ask whether the user would like to use the nightly toolchain instead of the stable toolchain. Default = 'true'
// Originally: placeholders.use_nightly = { type = "bool", prompt = "...", default = true }
// TODO: Should this be changed to default 'false'?
let use_nightly = variable::prompt("Would you like to use the nightly toolchain?", true);
variable::set("use_nightly", use_nightly);
log(`script use_nightly = ${use_nightly}`);
log(` outer use_nightly = ${variable::get("use_nightly")}`);

// Ask whether the user is going to use VSCode, if so we keep the .vscode folder. Default = 'true'
// Originally: placeholders.use_vscode = { type = "bool", prompt = "...", default = true }
let use_vscode = variable::prompt("Are you planning on using VSCode as your editor for this project?", true);
variable::set("use_vscode", use_vscode);
log(`script use_vscode = ${use_vscode}`);
log(` outer use_vscode = ${variable::get("use_vscode")}`);

// Prompt for choice of license, this is used to keep the correct license file and delete the unnecessary ones. Default = 'None'
// Originally: placeholders.license = { type = "string", prompt = "...", default = "MPL-2.0", choices = [ ... ] }
let license = variable::prompt("What license would you like to use for the project?", "None", LICENSES.keys());
variable::set("license", license);
log(`script license = ${license}`);
log(` outer license = ${variable::get("license")}`);

// Switch on license to rename and move the chosen license file, and delete the others
// First we handle the special cases of 'None' & 'Apache-2.0 OR MIT', otherwise we use
// the `LICENSES` map.
switch license {
  "None" => {
    log("no license");
    file::delete("licenses");
  }
  "Apache-2.0 OR MIT" => {
    log("moving apache and mit");
    file::rename("licenses/LICENSE-APACHE", "LICENSE-APACHE");
    file::rename("licenses/LICENSE-MIT", "LICENSE-MIT");
    file::delete("licenses");
  }
  _ => {
    log("getting license from map");
    let license_file = LICENSES[license];
    log(`license_file = ${license_file}`);
    file::rename(`licenses/${license_file}`, "LICENSE");
    log("deleting license directory");
    file::delete("licenses");
  }
  // Previously this switch held a full listing of each license, but since we are giving the user
  // a closed set of choices (the user must choose, they cannot type in a license) this should work
  // completely fine. There SHOULD be no possible way to get a license string other than what we expect.
}

// Ask whether the user would like to setup benchmarking support. Default = true
// Adds the `criterion` dev-dependency, and the `benches` folder.
let use_benches = variable::prompt("Would you like to set up a benchmarking support?", true);
variable::set("use_benches", use_benches);
log(`script use_benches = ${use_benches}`);
log(` outer use_benches = ${variable::get("use_benches")}`);

// If library, ask whether the user would like to set up integration test support. Default = true
// Adds the `tests` folder.
if project_kind == "library" {
    log("checking for integration testing option");
    let use_integration = variable::prompt("Would you like to include integration tests for your library?", true);
    variable::set("use_integration", use_integration);
} else {
    log("project is binary so no integration testing");
    variable::set("use_integration", false);
}

// Ask whether the user would like to setup benchmarking support. Default = true
// Adds the `flame` and `flamer` optional dependencies, and the `flame_example` file.
let use_flamegraph = variable::prompt("Would you like to set up flamegraph support and examples?", true);
variable::set("use_flamegraph", use_flamegraph);
log(`script use_flamegraph = ${use_flamegraph}`);
log(` outer use_flamegraph = ${variable::get("use_flamegraph")}`);

log("pre-setup complete");
